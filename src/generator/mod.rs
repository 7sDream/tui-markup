//! Generator generates final output from IR.

pub mod helper;

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "tui")]
pub use self::tui::TuiTextGenerator;

use std::fmt::Display;

use crate::{error::LocatedError, parser::Item, Error};

/// Generator generates final output from IR.
pub trait Generator<'a>
where
    Self::Err: LocatedError + Display,
    Error<'a, Self::Err>: From<Self::Err>,
{
    /// Output type.
    type Output;

    /// Error type.
    type Err;

    /// Generates final output from IR.
    fn generate(&mut self, markup: Vec<Vec<Item<'a>>>) -> Result<Self::Output, Self::Err>;
}

impl<'a, G: Generator<'a>> Generator<'a> for &mut G
where
    G::Err: LocatedError + Display,
    Error<'a, G::Err>: From<G::Err>,
{
    type Output = G::Output;

    type Err = G::Err;

    fn generate(&mut self, markup: Vec<Vec<Item<'a>>>) -> Result<Self::Output, Self::Err> {
        <G as Generator<'a>>::generate(self, markup)
    }
}
