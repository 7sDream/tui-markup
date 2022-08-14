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
//! ## Concept
//!
//! A compile process usually consists of at least three state: source, IR, output,
//! and two transformers: parser, generator.
//!
//! The parser convert source text into IR(Intermediate Representation),
//! and generator generate final output from this it.
//!
//! ```none
//! Source ---Parser---> IR ---Generator---> Output
//! ```
//!
//! In this crate, we defined the [syntax] of source, a [parser][parser::parse] and [IR][parser::Item],
//! but allows you to write generators yourself.
//!
//! We also provided generator for ansi terminal and the popular [tui crate][generator::TuiTextGenerator] for convenient,
//! you can enable them using features `ansi_term` and `tui`.
//!
//! The example page above is using the `tui` generator, print in Windows Terminal.
//!
//! [syntax]: https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf
//! [help-text-screenshot]: https://rikka.7sdre.am/files/37778eea-660b-47a6-bfd1-43979b5c703b.png
//! [help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt

mod error;
pub mod generator;
pub mod parser;

pub use error::{Error, LocatedError};

use generator::Generator;

/// Parse markup language source, then generate final output using the default configure of a generator type.
///
/// See document of generator type for examples.
///
/// ## Errors
///
/// If provided string has invalid markup syntax or the generator emit a error.
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
///
/// ## Errors
///
/// If provided string has invalid markup syntax or unknown tag.
pub fn compile_with<'a, G>(s: &'a str, mut gen: G) -> Result<G::Output, Error<'a, G::Err>>
where
    G: Generator<'a>,
    Error<'a, G::Err>: From<G::Err>,
{
    let ast = parser::parse(s)?;
    Ok(gen.generate(ast)?)
}
