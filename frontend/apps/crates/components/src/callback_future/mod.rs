use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;
use futures_channel::oneshot;
use utils::unwrap::UnwrapJiExt;

pub struct CallbackFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    rx: oneshot::Receiver<T>,
}

impl<T> CallbackFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    pub fn new(c: Box<dyn FnOnce(Box<dyn FnOnce(T)>)>) -> Self {
        let (tx, rx) = oneshot::channel();

        (c)(Box::new(move |val| {
            tx.send(val).unwrap_ji();
        }));

        Self { rx }
    }
}

impl<T> Future for CallbackFuture<T>
where
    T: std::fmt::Debug + 'static,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut self.rx), cx).map(|t| t.unwrap_ji())
    }
}
