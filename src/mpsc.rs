use tokio::sync::mpsc::{self, error::SendError};
use tracing::{
    debug_span,
    instrument::{Instrument, Instrumented}
};

pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = mpsc::channel::<T>(buffer);
    let span = debug_span!("mpsc");
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
}
