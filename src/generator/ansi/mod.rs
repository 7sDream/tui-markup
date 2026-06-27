//! Generator for ANSI terminal string output.

use anstyle::Style;

mod span;
mod tag;

pub use span::{StyledSpan, StyledText};
pub use tag::ANSITagConvertor;

use super::{
    Generator,
    helper::{CustomTagParser, GeneratorInfallible, NoopCustomTagParser, flatten},
};

/// Generator for ANSI terminal strings.
///
/// See [docs/ansi-tags.ebnf] for supported tags.
///
/// ## Example
///
/// ```
/// use tui_markup::{compile, generator::ANSIStringsGenerator};
///
/// let result = compile::<ANSIStringsGenerator>("I have a <green green text>").unwrap();
///
/// println!("{}", result);
/// ```
///
/// ### With custom tags
///
/// ```
/// use anstyle::{AnsiColor, Style};
/// use tui_markup::{compile_with, generator::ANSIStringsGenerator};
///
/// let g = ANSIStringsGenerator::new(|tag: &str| match tag {
///     "keyboard" => Some(
///         Style::new()
///             .bold()
///             .fg_color(Some(AnsiColor::Blue.into()))
///             .bg_color(Some(AnsiColor::Black.into())),
///     ),
///     _ => None,
/// });
///
/// let result = compile_with("Press <keyboard W> to move up", g).unwrap();
///
/// println!("{}", result);
/// ```
///
/// ## Show output
///
/// The result implements `Display`, so just print it.
///
/// [docs/ansi-tags.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/ansi-tags.ebnf
#[derive(Debug)]
pub struct ANSIStringsGenerator<P = NoopCustomTagParser<Style>> {
    convertor: ANSITagConvertor<P>,
}

impl<P> Default for ANSIStringsGenerator<P> {
    fn default() -> Self {
        Self {
            convertor: ANSITagConvertor::<P>::default(),
        }
    }
}

impl<P> ANSIStringsGenerator<P> {
    /// Create a new ANSI generator from a custom tag parser.
    pub fn new(p: P) -> Self {
        Self {
            convertor: ANSITagConvertor::new(p),
        }
    }
}

impl<'a, P> Generator<'a> for ANSIStringsGenerator<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Convertor = ANSITagConvertor<P>;
    type Err = GeneratorInfallible;
    type Output = StyledText<'a>;

    fn convertor(&mut self) -> &mut Self::Convertor {
        &mut self.convertor
    }

    fn generate(
        &mut self, markup: Vec<Vec<crate::parser::ItemG<'a, Self>>>,
    ) -> Result<Self::Output, Self::Err> {
        let mut spans = Vec::with_capacity(markup.len());

        for (i, line) in markup.into_iter().enumerate() {
            if i > 0 {
                spans.push(StyledSpan::new(Style::new(), "\n"));
            }
            spans.extend(flatten(line));
        }

        Ok(StyledText::new(spans))
    }
}
