/// GenericStyle
///
/// TODO: Doc
pub trait GenericStyle: Default + Clone {
    /// Patch self with other style
    fn patch(self, other: Self) -> Self;
}

/// GenericSpan
///
/// TODO: Doc
pub trait GenericSpan<'a, S: GenericStyle> {
    /// Create a span from str and a optional style
    fn with_style(s: &'a str, style: Option<S>) -> Self;
}
