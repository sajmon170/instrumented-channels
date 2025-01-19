use tokio::sync::mpsc::{self, error::SendError};
use tracing::{
    debug_span,
    instrument::{Instrument, Instrumented}
};

use crate::tools;

pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = mpsc::channel::<T>(buffer);
    let uuid = uuid::Uuid::new_v4();
    let span = debug_span!("mpsc", uuid = tools::base64_text(uuid.as_bytes()));
    (sender.instrument(span.clone()).into(), receiver.instrument(span).into())
}

pub struct Sender<T>(Instrumented<mpsc::Sender<T>>);

impl<T> From<Instrumented<mpsc::Sender<T>>> for Sender<T> {
    fn from(value: Instrumented<mpsc::Sender<T>>) -> Self {
        Self(value)
    }
}

impl<T> Sender<T> {
    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.0.inner().send(value).await
    }
}

pub struct Receiver<T>(Instrumented<mpsc::Receiver<T>>);

impl<T> From<Instrumented<mpsc::Receiver<T>>> for Receiver<T> {
    fn from(value: Instrumented<mpsc::Receiver<T>>) -> Self {
        Self(value)
    }
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.0.inner_mut().recv().await
    }

    pub fn close(&mut self) {
        self.0.inner_mut().close()
    }
}
