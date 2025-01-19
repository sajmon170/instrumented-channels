use crate::tools;

use tracing::{
    debug_span, Span,
    debug,
    instrument::{Instrument, Instrumented}
};

use tokio::sync::mpsc::{self, error::SendError};
use uuid::Uuid;

pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel::<T>(buffer);
    let uuid = Uuid::new_v4();
    (Sender::new(tx, uuid), Receiver::new(rx, uuid))
}

#[derive(Clone, Debug)]
pub struct Sender<T> {
    tx: Instrumented<mpsc::Sender<T>>,
    span: Span
}

impl<T> Sender<T> {
    pub fn new(tx: mpsc::Sender<T>, uuid: Uuid) -> Self {
        let span = debug_span!("mpsc-tx",
                               uuid = tools::base64_text(uuid.as_bytes()));
        Self { tx: tx.instrument(span.clone()), span }
    }

    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        debug!(parent: &self.span, "Sending value");
        self.tx.inner().send(value).await
    }
}

#[derive(Debug)]
pub struct Receiver<T> {
    rx: Instrumented<mpsc::Receiver<T>>,
    span: Span
}

impl<T> Receiver<T> {
    pub fn new(rx: mpsc::Receiver<T>, uuid: Uuid) -> Self {
        let span = debug_span!("mpsc-rx",
                               uuid = tools::base64_text(uuid.as_bytes()));

        Self { rx: rx.instrument(span.clone()), span }
    }

    pub async fn recv(&mut self) -> Option<T> {
        let value = self.rx.inner_mut().recv().await;
        debug!(parent: &self.span, "Maybe received value");
        value
    }

    pub fn close(&mut self) {
        self.rx.inner_mut().close()
    }
}
