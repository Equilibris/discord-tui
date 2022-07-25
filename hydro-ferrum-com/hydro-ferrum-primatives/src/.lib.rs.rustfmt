use futures_core::stream::Stream;
use std::future::Future;

pub trait Notify {
    /// Requests handled by the service.
    type Request;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<(), Self::Error>>;

    /// Process the request and return the response asynchronously.
    ///
    /// This function is expected to be callable off task. As such,
    /// implementations should take care to not call `poll_ready`. If the
    /// service is at capacity and the request is unable to be handled, the
    /// returned `Future` should resolve to an error.
    ///
    /// Calling `call` without calling `poll_ready` is permitted. The
    /// implementation must be resilient to this fact.
    fn call(&mut self, req: Self::Request) -> Self::Future;
}

pub trait BCoSynth<Up, E, F, T>
where
    F: Future<Output = Result<(), E>>,
    T: Notify<Request = Up, Error = E, Future = F>,
{
    type Output;

    fn synth(self, parent: T) -> Self::Output;
}
pub trait FCoSynth<Down, T>
where
    T: Stream<Item = Down>,
{
    type Output;

    fn synth(self, parent: T) -> Self::Output;
}

pub trait PCoSynth<Down, Up, E, F: Future<Output = Result<(), E>>, T>
where
    T: Notify<Request = Up, Error = E, Future = F> + Stream<Item = Down>,
{
    type Output;

    fn synth(self, parent: T) -> Self::Output;
}

pub trait BSynth<Up>: Notify<Request = Up> + Sized {
    fn bpipe<T: BCoSynth<Up, Self::Error, Self::Future, Self>>(self, value: T) -> T::Output {
        value.synth(self)
    }
}
pub trait FSynth<Down>: Stream<Item = Down> + Sized {
    fn fpipe<T: FCoSynth<Down, Self>>(self, value: T) -> T::Output {
        value.synth(self)
    }
}
pub trait PSynth<Up, Down>: Stream<Item = Down> + Notify<Request = Up> + Sized {
    fn pipe<T: PCoSynth<Down, Up, Self::Error, Self::Future, Self>>(self, value: T) -> T::Output {
        value.synth(self)
    }
}

impl<Up, T: Notify<Request = Up>> BSynth<Up> for T {}
impl<Down, T: Stream<Item = Down>> FSynth<Down> for T {}
impl<Up, Down, T: Notify<Request = Up> + Stream<Item = Down>> PSynth<Up, Down> for T {}
