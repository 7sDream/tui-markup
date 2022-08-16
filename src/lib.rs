#![warn(clippy::all)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(warnings)]
#![cfg_attr(not(test), forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # tui markup
//!
//! This crate provides a markup language to
//! quickly write colorful and styled terminal text in plain text.
//!
//! I suggest to check [help.txt] in examples folder,
//! which generated this self-describing syntax help document:
//!
//! ![][help-text-screenshot]
//!
//! For formal syntax specification, see [syntax.ebnf][syntax].
//!
//! ## How two use
//!
//! ```ignore
//! let output = tui_markup::compile::<GeneratorType>("<geeen hello>").unwrap();
//! ```
//!
//! How two print the output vary depending on the the [Generator] you use, See their document for more info.
//!
//! ### Builtin generators
//!
//! The builtin generators are under feature gates, there is the list:
//!
//! feature | environment             | generator type
//! :------ | :---------------------- | :-------------
//! `tui`   | the popular [tui] crate | [TuiTextGenerator][generator::tui::TuiTextGenerator]
//!
//! The example page above is using the `tui` generator, print in Windows Terminal.
//!
//! If you want write your own generator, please see document of [Generator] trait.
//!
//!
//! [syntax]: https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf
//! [help-text-screenshot]: https://rikka.7sdre.am/files/37778eea-660b-47a6-bfd1-43979b5c703b.png
//! [help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt

mod error;
pub mod generator;
pub mod parser;

pub use error::{Error, LocatedError};

use generator::{Generator, TagConvertor};

/// Parse markup language source, then generate final output using the default configure of a generator type.
///
/// See document of generator type for examples.
pub fn compile<'a, G>(s: &'a str) -> Result<G::Output, Error<'a, G::Err>>
where
    G: Default,
    G: Generator<'a>,
    Error<'a, G::Err>: From<G::Err>,
{
    compile_with(s, G::default())
}

/// Parse markup language source, then generate final output using the provided generator.
///
/// See document of generator type for examples.
pub fn compile_with<'a, G>(s: &'a str, mut gen: G) -> Result<G::Output, Error<'a, G::Err>>
where
    G: Generator<'a>,
    Error<'a, G::Err>: From<G::Err>,
{
    let ast = parser::parse(s)?;
    let ir = gen.convertor().convert(ast).map_err(Error::Tag)?;
    Ok(gen.generate(ir)?)
}
