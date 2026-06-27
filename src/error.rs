use std::fmt::{Debug, Display};

use crate::parser::Error as ParseError;

/// Error with a location info.
pub trait LocatedError {
    /// get error happened location in source input.
    fn location(&self) -> (usize, usize);
}

/// Error for markup source compile pipeline.
///
/// Display this error in `{}` formatter will show a error message with detailed reason and
/// location. So usually you don't need check variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<'a, GE> {
    /// Parsing stage failed, usually means there is invalid syntax in source string
    Parse(ParseError<'a>),

    /// Generating stage failed, see document of generator type for detail.
    Gen(GE),
}

impl<GE> Display for Error<'_, GE>
where
    GE: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(pe) => f.write_fmt(format_args!("parse failed: {}", pe)),
            Error::Gen(ge) => f.write_fmt(format_args!("generate failed: {}", ge)),
        }
    }
}

impl<GE> std::error::Error for Error<'_, GE> where Self: Debug + Display {}

impl<GE: LocatedError> LocatedError for Error<'_, GE> {
    fn location(&self) -> (usize, usize) {
        match self {
            Self::Parse(e) => e.location(),
            Self::Gen(e) => e.location(),
        }
    }
}

impl<'a, GE> From<ParseError<'a>> for Error<'a, GE> {
    fn from(e: ParseError<'a>) -> Self {
        Self::Parse(e)
    }
}

#[cfg(test)]
mod test {
    use crate::generator::helper::GeneratorInfallible;

    #[test]
    fn error_must_impl_std_error() {
        fn is_error<E: std::error::Error>() {}

        is_error::<GeneratorInfallible>();
        is_error::<crate::parser::Error<'_>>();
        is_error::<super::Error<'static, GeneratorInfallible>>();
    }
}
