use std::pin::Pin;

use futures::{
    future::{BoxFuture, Either, Ready},
    Future,
};

#[pin_project::pin_project]
pub struct ReadyOrNot<'a, T>(#[pin] Either<Ready<T>, BoxFuture<'a, T>>);

impl<T> From<Ready<T>> for ReadyOrNot<'static, T> {
    fn from(future: Ready<T>) -> Self {
        Self(Either::Left(future))
    }
}

impl<'a, T> From<BoxFuture<'a, T>> for ReadyOrNot<'a, T> {
    fn from(future: BoxFuture<'a, T>) -> Self {
        Self(Either::Right(future))
    }
}

impl<'a, T> Future for ReadyOrNot<'a, T> {
    type Output = T;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.project().0.poll(cx)
    }
}
