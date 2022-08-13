use thiserror::Error;

/// Possible errors when parse makeup source text into tui text.
#[derive(Debug, Clone, Error)]
pub enum Error<'a> {
    /// There is invalid syntax in source
    #[error("invalid syntax when parse {0:?} at {1}:{2}")]
    InvalidSyntax(&'a str, usize, usize),

    /// There is a unknown tag in source
    #[error("unknown tag {0:?} at {1}:{2}")]
    InvalidTag(&'a str, usize, usize),
}

impl<'a> Error<'a> {
    /// Get position of error
    pub fn position(&self) -> (usize, usize) {
        match *self {
            Self::InvalidSyntax(_, x, y) => (x, y),
            Self::InvalidTag(_, x, y) => (x, y),
        }
    }
}
