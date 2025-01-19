use tokio::sync::oneshot::{self, error::RecvError};
use tracing::{
    debug_span,
    instrument::{Instrument, Instrumented}
};

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};

use pin_project::pin_project;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = oneshot::channel::<T>();
    let span = debug_span!("oneshot");
    (sender.instrument(span.clone()).into(), receiver.instrument(span).into())
}

pub struct Sender<T>(Instrumented<oneshot::Sender<T>>);

impl<T> From<Instrumented<oneshot::Sender<T>>> for Sender<T> {
    fn from(value: Instrumented<oneshot::Sender<T>>) -> Self {
        Self(value)
    }
}

impl<T> Sender<T> {
    pub fn send(self, value: T) -> Result<(), T> {
        self.0.into_inner().send(value)
    }
}

#[pin_project]
pub struct Receiver<T>(#[pin] Instrumented<oneshot::Receiver<T>>);

impl<T> From<Instrumented<oneshot::Receiver<T>>> for Receiver<T> {
    fn from(value: Instrumented<oneshot::Receiver<T>>) -> Self {
        Self(value)
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.0.poll(cx)
    }
}

impl<T> Receiver<T> {
    pub fn close(&mut self) {
        self.0.inner_mut().close()
    }
}
