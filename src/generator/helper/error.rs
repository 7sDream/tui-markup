use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::LocatedError;

/// Error type for infallible generator.
///
/// You should never return (even construct) this error if choose this as the Error type of your Generator.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct GeneratorInfallible;

impl GeneratorInfallible {
    fn panic() -> ! {
        panic!("infallible generator emitted a error, please report this as bug")
    }
}

impl Debug for GeneratorInfallible {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GeneratorInfallible::panic()
    }
}

impl Display for GeneratorInfallible {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GeneratorInfallible::panic()
    }
}

impl Error for GeneratorInfallible {}

impl LocatedError for GeneratorInfallible {
    fn location(&self) -> (usize, usize) {
        GeneratorInfallible::panic()
    }
}

impl<'a> From<GeneratorInfallible> for crate::Error<'a, GeneratorInfallible> {
    fn from(_e: GeneratorInfallible) -> Self {
        GeneratorInfallible::panic()
    }
}
