use tokio::sync::broadcast::{error::SendError, Receiver, Sender};

pub type Subscriber = Receiver<Vec<u8>>;
pub const PUBLISHER_SIZE: usize = 1337;
pub(crate) type Publisher = Sender<Vec<u8>>;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
    Log,
    Scan,
    PortOpen,
    ServiceIdentified,
}

pub const MSG_VARIANTS: [Event; 4] = [
    Event::Log,
    Event::Scan,
    Event::PortOpen,
    Event::ServiceIdentified,
];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to publish event")]
    PublishError(#[from] SendError<Vec<u8>>),
    #[error("stream is empty")]
    Empty,
}


pub trait PubSubInterface {
    fn subscribe(&mut self, event: Event, key: Option<&[u8]>) -> Result<Subscriber, Error>;
    fn publish(&self, event: Event, key: &[u8], payload: &[u8]) -> Result<(), Error>;
}
