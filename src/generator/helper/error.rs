use std::fmt::Display;

use crate::{Error, LocatedError};

/// Error type for infallible generator.
///
/// You should never return this error in generating step if choose this as the Error type of your Generator.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GeneratorInfallible;

impl Display for GeneratorInfallible {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("infallible generator emitted a error, please report this bug")
    }
}

impl LocatedError for GeneratorInfallible {
    fn location(&self) -> (usize, usize) {
        panic!("infallible generator emitted a error, please report this bug")
    }
}

impl<'a> From<GeneratorInfallible> for Error<'a, GeneratorInfallible> {
    fn from(e: GeneratorInfallible) -> Self {
        Error::Gen(e)
    }
}
