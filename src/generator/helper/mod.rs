//! Helper functions for create generator.

mod error;
mod flatten;
mod tag;
mod unescape;

pub use error::GeneratorInfallible;
pub use flatten::{flatten, FlattenableSpan, FlattenableStyle};
pub use tag::{CustomTagParser, NoopCustomTagParser};
pub use unescape::{unescape, Unescape};
