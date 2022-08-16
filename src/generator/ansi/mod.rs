//! Generator for asni terminal.

mod span;
mod tag;

use ansi_term::{ANSIString, Style};

use super::{
    helper::{flatten, CustomTagParser, GeneratorInfallible, NoopCustomTagParser},
    Generator,
};

pub use span::WrappedStyle;
pub use tag::ANSITermTagConvertor;

/// Generator for ansi terminal strings.
#[cfg_attr(docsrs, doc(cfg(feature = "ansi")))]
#[derive(Debug)]
pub struct ANSIStringsGenerator<P = NoopCustomTagParser<Style>> {
    convertor: ANSITermTagConvertor<P>,
}

impl<P> Default for ANSIStringsGenerator<P> {
    fn default() -> Self {
        Self {
            convertor: Default::default(),
        }
    }
}

impl<P> ANSIStringsGenerator<P> {
    /// Create a new ansi term string generator from a custom tag parser.
    pub fn new(p: P) -> Self {
        Self {
            convertor: ANSITermTagConvertor::new(p),
        }
    }
}

impl<'a, P> Generator<'a> for ANSIStringsGenerator<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Convertor = ANSITermTagConvertor<P>;

    type Output = Vec<ANSIString<'a>>;

    type Err = GeneratorInfallible;

    fn convertor(&mut self) -> &mut Self::Convertor {
        &mut self.convertor
    }

    fn generate(&mut self, markup: Vec<Vec<crate::parser::ItemG<'a, Self>>>) -> Result<Self::Output, Self::Err> {
        Ok(markup.into_iter().map(flatten).fold(vec![], |mut acc, line| {
            acc.push(Style::default().paint("\n"));
            acc.extend(line);
            acc
        }))
    }
}
