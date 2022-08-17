//! Generator generates final output for showing.

pub mod helper;
mod tag;

#[cfg(feature = "tui")]
#[cfg_attr(docsrs, doc(cfg(feature = "tui")))]
pub mod tui;
#[cfg(feature = "tui")]
pub use self::tui::TuiTextGenerator;

#[cfg(feature = "ansi")]
#[cfg_attr(docsrs, doc(cfg(feature = "ansi")))]
pub mod ansi;
#[cfg(feature = "ansi")]
pub use self::ansi::ANSIStringsGenerator;

// TODO: Crossterm generator
// TODO: Termion generator

use std::fmt::{Debug, Display};

use crate::{error::LocatedError, parser::ItemG, Error};

pub use tag::{Tag, TagConvertor, TagG};

/// Generator generates final output to show tui markup in some backend.
///
/// ## How to add support for new backend
///
/// Some concepts:
///
/// - Markup text/Source: the text you write in tui markup language.
/// - [Parser][crate::parser::parse]: parse markup text into a series of [Item][crate::parser::Item],
/// which usually be called as AST.
/// - [Tag Convertor][TagConvertor]: Convert raw tag string like `green`, `bg:66ccff`, `mod:b` into [Tag].
/// - [Generator]: generator final output from `Item<Tag>`.
///
/// So the whole pipeline is:
///
/// ```none
/// Source --- Parser --> Item --- Tag Convertor --> Item<Tag> --- Generator --> Output --> Show it in some backend
/// ```
///
/// The source, parser, Item, Tag, is already defined, so just write a [Tag Convertor][TagConvertor]
/// and a [Generator], a new backend will be supported.
///
/// ## Generic implementation using flatten
///
/// Your tag convertor will parse color/modifiers string to some `Color`/`Modifier` type, and will support custom tag
/// by a `Style` type, which can be converted from `Color` and `Modifier` too.
///
/// In this case, a [Tag] of this convertor can be convert to the `Style` type easily.
///
/// If this `Style` can be patched by other, then you can use the [`flatten`][helper::flatten]
/// help method to do almost all the convert staff from AST to your final result.
///
/// Read document of [`flatten`][helper::flatten] to learn more, or just checkout a builtin implementation.
pub trait Generator<'a> {
    /// Tag convertor type.
    type Convertor: TagConvertor<'a>;

    /// Output type.
    type Output;

    /// Error type.
    ///
    /// If the generator can't fall, please use [`GeneratorInfallible`][helper::GeneratorInfallible].
    type Err: LocatedError + Display + Debug + Into<Error<'a, Self::Err>>;

    /// Get the tag convertor.
    fn convertor(&mut self) -> &mut Self::Convertor;

    /// Generates final output from ast, which is output result of the Convertor.
    fn generate(&mut self, markup: Vec<Vec<ItemG<'a, Self>>>) -> Result<Self::Output, Self::Err>;
}

impl<'a, G: Generator<'a>> Generator<'a> for &mut G {
    type Convertor = G::Convertor;

    type Output = G::Output;

    type Err = G::Err;

    fn convertor(&mut self) -> &mut Self::Convertor {
        <G as Generator<'a>>::convertor(self)
    }

    fn generate(&mut self, ir: Vec<Vec<ItemG<'a, G>>>) -> Result<Self::Output, Self::Err> {
        <G as Generator<'a>>::generate(self, ir)
    }
}
