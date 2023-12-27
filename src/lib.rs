#![warn(clippy::all, clippy::pedantic)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(warnings)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(test), forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # tui markup
//!
//! This crate provides a markup language to
//! quickly write colorful and styled terminal text in plain text.
//!
//! I suggest to check [examples/help.txt], which generated this self-describing syntax help document:
//!
//! ![][help-text-screenshot]
//!
//! For formal syntax specification, see [docs/syntax.ebnf].
//!
//! ## How to use
//!
//! ```ignore
//! let output = tui_markup::compile::<Generator>("<bg:blue,green,b hello>").unwrap();
//! ```
//!
//! The string wrapped in `<>`(like the `bg:blue,green,b` in above example) is called a element,
//! start with a tag list(comma separated), those tags add styles to inner items.
//!
//! Usable tags are vary depending on the the [Generator] you use,
//! and generator will ignore all tags it does not understand.
//!
//! So it's better checkout their document before write your markup text.
//!
//! ### Builtin generators
//!
//! The builtin generators are under feature gates:
//!
//! feature     | Target                                                              | generator type
//! :---------- | :------------------------------------------------------------------ | :-------------
//! `ansi`      | Direct print into stdout when using an asni compatible terminal     | [`ANSIStringsGenerator`][generator::ANSIStringsGenerator]
//! `tui`       | Integrated with the popular [tui] crate                             | [`TuiTextGenerator`][generator::TuiTextGenerator]
//! `ratatui`   | Integrated with the popular [ratatui] crate                         | [`RatatuiTextGenerator`][generator::RatatuiTextGenerator]
//! `crossterm` | Integrated with [crossterm] crate                                   | [`CrosstermCommandsGenerator`][generator::CrosstermCommandsGenerator]
//!
//! The example screenshot above is using the `tui` generator, print in Windows Terminal.
//!
//! If you want write your own generator, please checkout documents of [Generator] trait.
//!
//! [docs/syntax.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf
//! [help-text-screenshot]: https://rikka.7sdre.am/files/ee68d36d-b1e7-4575-bb13-e37ba7ead044.png
//! [examples/help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt

mod error;
pub mod generator;
pub mod parser;

pub use error::{Error, LocatedError};

use generator::{Generator, TagConvertor};

/// Parse markup language source, then generate final output using the default configure of a generator type.
///
/// See document of generator type for examples.
///
/// ## Errors
///
/// If input source contains invalid syntax or generator failed.
pub fn compile<'a, G>(s: &'a str) -> Result<G::Output, Error<'a, G::Err>>
where
    G: Generator<'a> + Default,
{
    compile_with(s, G::default())
}

/// Parse markup language source, then generate final output using the provided generator.
///
/// See document of generator type for examples.
///
/// ## Errors
///
/// If input source contains invalid syntax or generator failed.
pub fn compile_with<'a, G>(s: &'a str, mut gen: G) -> Result<G::Output, Error<'a, G::Err>>
where
    G: Generator<'a>,
{
    let ast = parser::parse(s)?;
    let ir = gen.convertor().convert_ast(ast);
    match gen.generate(ir) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.into()),
    }
}
