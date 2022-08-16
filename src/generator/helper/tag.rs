use std::marker::PhantomData;

/// Custom tag parser trait.
///
/// Closure `FnMut(&str) -> CustomType` and [`NoopCustomTagParser`] impl this trait for convenient.
pub trait CustomTagParser {
    /// Custom tag type.
    type Output;

    /// Parse string to custom tag type.
    fn parse(&mut self, s: &str) -> Option<Self::Output>;
}

impl<F, O> CustomTagParser for F
where
    F: FnMut(&str) -> Option<O>,
{
    type Output = O;

    fn parse(&mut self, s: &str) -> Option<Self::Output> {
        self(s)
    }
}

/// If a tag convertor support custom tag and you also want it to be optional and impl Default trait,
/// normally you will use a `Option<T: CustomTagParser<Custom>>` in the convertor.
///
/// But the user still need to provide some type of that T(which the usually can't)
/// even if they do not want custom tag.
///
/// With this type, you can add a `T = NoopCustomTagParser<Custom>` in struct to make them happy.
#[derive(Default, Debug)]
pub struct NoopCustomTagParser<S>(PhantomData<*const S>);

impl<S> CustomTagParser for NoopCustomTagParser<S> {
    type Output = S;

    fn parse(&mut self, _s: &str) -> Option<S> {
        None
    }
}
