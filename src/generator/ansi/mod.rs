//! Generator for asni terminal string.

mod span;
mod tag;

use ansi_term::{ANSIString, Style};

use super::{
    helper::{flatten, CustomTagParser, GeneratorInfallible, NoopCustomTagParser},
    Generator,
};

pub use tag::ANSITermTagConvertor;

/// Generator for ansi terminal strings.
///
/// See [docs/ansi-tags.ebnf] for supported tags.
///
/// ## Example
///
/// ```
/// use ansi_term::ANSIStrings;
/// use tui_markup::{compile, generator::ANSIStringsGenerator};
///
/// let result = compile::<ANSIStringsGenerator>("I have a <green green text>").unwrap();
///
/// println!("{}", ANSIStrings(&result));
/// ```
///
/// ### With custom tags
///
/// ```
/// use ansi_term::{ANSIStrings, Style, Color};
/// use tui_markup::{compile_with, generator::ANSIStringsGenerator};
///
/// let gen = ANSIStringsGenerator::new(|tag: &str| match tag {
///     "keyboard" => Some(Style::default().fg(Color::Green).on(Color::White).bold()),
///     _ => None,
/// });
///
/// let result = compile_with("Press <keyboard W> to move up", gen).unwrap();
///
/// println!("{}", ANSIStrings(&result));
/// ```
///
/// ## Show output
///
/// Like example above, use [ansi_term::ANSIStrings()] to make a temp variable and just print it.
///
/// [docs/ansi-tags.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/ansi-tags.ebnf
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
            if !acc.is_empty() {
                acc.push(Style::default().paint("\n"));
            }
            acc.extend(line);
            acc
        }))
    }
}
