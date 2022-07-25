use std::sync::Arc;

use futures_core::{future::BoxFuture, stream::Stream, Future};
use hydro_ferrum_primatives::*;
use tokio::sync::RwLock;

pub struct Share;

#[derive(Clone)]
pub struct ShareContainer<T>(Arc<RwLock<T>>);

impl<Up, E, F: Future<Output = Result<(), E>>, T: Notify<Request = Up, Error = E, Future = F>>
    BCoSynth<Up, E, F, T> for Share
{
    type Output = ShareContainer<T>;

    fn synth(self, parent: T) -> Self::Output {
        ShareContainer(Arc::new(RwLock::new(parent)))
    }
}
impl<
        Down,
        Up,
        E,
        F: Future<Output = Result<(), E>>,
        T: Stream<Item = Down> + Notify<Request = Up, Error = E, Future = F>,
    > PCoSynth<Down, Up, E, F, T> for Share
{
    type Output = ShareContainer<T>;

    fn synth(self, parent: T) -> Self::Output {
        ShareContainer(Arc::new(RwLock::new(parent)))
    }
}
impl<Down, T: Stream<Item = Down>> FCoSynth<Down, T> for Share {
    type Output = ShareContainer<T>;

    fn synth(self, parent: T) -> Self::Output {
        ShareContainer(Arc::new(RwLock::new(parent)))
    }
}

impl<
        Up,
        E,
        F: Future<Output = Result<(), E>>,
        T: Notify<Request = Up, Error = E, Future = F> + Send + Sync,
    > Notify for ShareContainer<T>
{
    type Request = Up;
    type Error = E;
    type Future = BoxFuture<'static, Result<(), E>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let lock = self.0.write();

        todo!()
    }
}
