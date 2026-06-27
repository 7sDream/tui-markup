//! Generator implementations for crossterm crate.

mod span;
mod tag;

use crossterm::style::{ContentStyle, Print};
pub use span::Span;
pub use tag::CrosstermTagConvertor;

use crate::{
    generator::{
        Generator,
        helper::{CustomTagParser, GeneratorInfallible, NoopCustomTagParser, flatten},
    },
    parser::ItemG,
};

/// Generator for [crossterm crate][crossterm], generated result is a series of it's
/// [Command][crossterm::Command]s.
///
/// See [docs/tui-tags.ebnf] for supported tags.
///
/// ### Show output
///
/// Execute/Queue all the commands in the buffer where you want to print the result. For example, in
/// stdout:
///
/// ```
/// use std::io::Write;
///
/// use crossterm::QueueableCommand;
/// use tui_markup::{compile, generator::CrosstermCommandsGenerator};
///
/// let mut stdout = std::io::stdout();
/// let spans = compile::<CrosstermCommandsGenerator>("I have a <green green text>").unwrap();
/// for span in &spans {
///     stdout.queue(span).unwrap();
/// }
/// stdout.flush().unwrap();
/// ```
///
/// See [example/crossterm.rs] for a example code.
///
/// [docs/tui-tags.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/tui-tags.ebnf
/// [example/crossterm.rs]: https://github.com/7sDream/tui-markup/blob/master/example/crossterm.rs
#[derive(Debug)]
pub struct CrosstermCommandsGenerator<P = NoopCustomTagParser<ContentStyle>> {
    convertor: CrosstermTagConvertor<P>,
}

impl<P> Default for CrosstermCommandsGenerator<P> {
    fn default() -> Self {
        Self {
            convertor: CrosstermTagConvertor::<P>::default(),
        }
    }
}

impl<P> CrosstermCommandsGenerator<P> {
    /// Create a new generator, with a custom tag parser.
    pub fn new(p: P) -> Self {
        Self {
            convertor: CrosstermTagConvertor::new(p),
        }
    }
}

impl<'a, P> Generator<'a> for CrosstermCommandsGenerator<P>
where
    P: CustomTagParser<Output = ContentStyle>,
{
    type Convertor = CrosstermTagConvertor<P>;
    type Err = GeneratorInfallible;
    type Output = Vec<Span<'a>>;

    fn convertor(&mut self) -> &mut Self::Convertor {
        &mut self.convertor
    }

    fn generate(&mut self, markup: Vec<Vec<ItemG<'a, Self>>>) -> Result<Self::Output, Self::Err> {
        let mut spans = Vec::with_capacity(markup.len());
        for (i, line) in markup.into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::NoStyle(Print("\n")));
            }
            spans.extend(flatten(line));
        }
        Ok(spans)
    }
}
