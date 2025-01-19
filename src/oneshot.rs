use crate::tools;

use tracing::{
    debug_span, Span,
    debug,
    instrument::{Instrument, Instrumented}
};

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};

use tokio::sync::oneshot::{self, error::RecvError};
use uuid::Uuid;
use pin_project::pin_project;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = oneshot::channel::<T>();
    let uuid = Uuid::new_v4();
    (Sender::new(tx, uuid), Receiver::new(rx, uuid))
}

#[derive(Debug)]
pub struct Sender<T> {
    tx: Instrumented<oneshot::Sender<T>>,
    span: Span
}

impl<T> Sender<T> {
    pub fn new(tx: oneshot::Sender<T>, uuid: Uuid) -> Self {
        let span = debug_span!("oneshot-tx",
                               uuid = tools::base64_text(uuid.as_bytes()));
        Self { tx: tx.instrument(span.clone()), span }
    }
    
    pub fn send(self, value: T) -> Result<(), T> {
        debug!(parent: &self.span, "Sending value");
        self.tx.into_inner().send(value)
    }
}

#[derive(Debug)]
#[pin_project]
pub struct Receiver<T> {
    #[pin]
    rx: Instrumented<oneshot::Receiver<T>>,
    span: Span
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let span = this.span.clone();
        let result = this.rx.poll(cx);

        if let Poll::Ready(_) = result {
            debug!(parent: &span, "Received value");
        }
        
        result
    }
}

impl<T> Receiver<T> {
    pub fn new(rx: oneshot::Receiver<T>, uuid: Uuid) -> Self {
        let span = debug_span!("oneshot-rx",
                               uuid = tools::base64_text(uuid.as_bytes()));

        Self { rx: rx.instrument(span.clone()), span }
    }
    
    pub fn close(&mut self) {
        self.rx.inner_mut().close()
    }
}
