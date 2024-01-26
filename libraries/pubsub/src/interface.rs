use entities::filter::Wrapper;
use tokio::sync::broadcast::{error::SendError, Receiver, Sender};

pub type Subscriber = Receiver<Vec<u8>>;
pub const PUBLISHER_SIZE: usize = 1337;
pub(crate) type Publisher = Sender<Vec<u8>>;

#[derive(PartialEq, Eq, Hash)]
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

    #[error("unable to decode contents")]
    Decode(#[from] entities::Error)
}

pub struct Filter {
    internal: Subscriber,
    req: fn(&Wrapper) -> bool,
}

impl Filter {
    pub fn new(source: Subscriber, req: fn(&Wrapper)->bool) -> Self {
        Self{internal: source, req}
    }

    pub async fn next(&mut self) -> Result<Vec<u8>, Error> {

        while let Ok(val) = self.internal.recv().await {
            let wrap: Wrapper = entities::deserialize(&val)?;

            if (self.req)(&wrap) {
                return Ok(wrap.value)
            }
        }

        Err(Error::Empty)
    }
}

pub trait PubSubInterface {
    fn subscribe(&self, event: Event) -> Result<Subscriber, Error>;
    fn publish(&self, event: Event,key: &[u8], payload: &[u8]) -> Result<(), Error>;
    fn filtered(&self, event:Event, callback: fn(&Wrapper) -> bool) -> Result<Filter, Error>;
}