use std::fmt::Display;

use winnow::{
    error::{AddContext, ParserError},
    stream::{LocatingSlice, Offset},
};

use crate::{error::LocatedError, parser::LSpan};

/// Kind of parse error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// There is unescaped `<`, `>`, `\` character.
    UnescapedChar,
    /// There is a unescapable character after `\`.
    UnescapableChar,
    /// Element not closed but reaches line end.
    ElementNotClose,
}

/// Error type for [parse][super::parse].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    kind: Option<ErrorKind>,
    // The remaining input at the error point. Used for display of the first character.
    pub(crate) input: LSpan<'a>,
    // Line number in source
    pub(crate) line: usize,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.kind {
            Some(kind) => match kind {
                ErrorKind::UnescapedChar => "unescaped character",
                ErrorKind::UnescapableChar => "unescapable character",
                ErrorKind::ElementNotClose => "expect '>' to close element for element starter",
            },
            None => "unknown error",
        })?;

        if let Some(c) = self.input.chars().next() {
            f.write_fmt(format_args!(" \'{}\'", c))?;
        }

        let (line, offset) = self.location();
        f.write_fmt(format_args!(" near {}:{}", line, offset))
    }
}

impl std::error::Error for Error<'_> {}

impl<'a> Error<'a> {
    fn new(input: &LocatingSlice<&'a str>) -> Self {
        Self {
            kind: None,
            input: *input,
            line: 0,
        }
    }

    pub(crate) fn with_input(mut self, input: &LocatingSlice<&'a str>) -> Self {
        self.input = *input;
        self
    }

    /// Set the line number for this error.
    pub(crate) fn with_line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    /// Set the line number for this error.
    pub(crate) fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Get error kind.
    pub fn kind(&self) -> Option<ErrorKind> {
        self.kind
    }
}

impl LocatedError for Error<'_> {
    fn location(&self) -> (usize, usize) {
        let mut start = self.input;
        start.reset_to_start();
        (self.line + 1, self.input.offset_from(&start) + 1)
    }
}

impl<'a> ParserError<LSpan<'a>> for Error<'a> {
    type Inner = Self;

    fn from_input(input: &LocatingSlice<&'a str>) -> Self {
        Self::new(input)
    }

    fn into_inner(self) -> Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<'a> AddContext<LSpan<'a>, ErrorKind> for Error<'a> {
    fn add_context(
        self, _input: &LSpan<'a>, _token_start: &<LSpan<'a> as winnow::stream::Stream>::Checkpoint,
        context: ErrorKind,
    ) -> Self {
        self.with_kind(context)
    }
}
