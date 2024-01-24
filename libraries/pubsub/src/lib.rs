/*
    Events are identified using a string?
    Registered events are used as keys in map, which maps to vector of write interfaces where we write Event information or something


    Register functions returns list of tuples, where it's (key, interface).
    key can be an enum, or a string. Strings allow more decoupling but have an issue where people can misspell

    Each interface is of type Write<Vec<u8>>?
        * Allows interfaces to json/protobuf serialize the contents when writing a message. Readers then decode it

*/

use std::collections::HashMap;

use interface::{Event, PubSubInterface, Publisher, Subscriber};

/// Defines traits and structs for publishing/subscribing to data
pub mod interface;

pub struct PubSub {
    senders: HashMap<Event, Publisher>,
}

impl PubSubInterface for PubSub {
    fn subscribe(&self, event: Event) -> Result<Subscriber, interface::Error> {
        let r = self
            .senders
            .get(&event)
            .expect("No such event registered")
            .subscribe();
        Ok(r)
    }

    fn publish(&self, event: Event, payload: &[u8]) -> Result<(), interface::Error> {
        self.senders
            .get(&event)
            .expect("No such event registered")
            .send(payload.to_vec())?;
        Ok(())
    }
}

impl PubSub {
    pub fn new() -> Self {
        let mut senders = HashMap::new();
        for event in interface::MSG_VARIANTS {
            senders.insert(event, Publisher::new(interface::PUBLISHER_SIZE));
        }
        Self { senders }
    }
}
impl Default for PubSub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::{Event, PubSubInterface};
    use crate::PubSub;

    pub async fn entrypoint<T: PubSubInterface>(handle: &T) {
        let mut int = handle.subscribe(Event::Log).unwrap();

        let data = int.recv().await.unwrap();
        assert_eq!(data, b"message published")
    }

    pub async fn entrypoint2<T: PubSubInterface>(handle: &T) {
        handle.publish(Event::Log, b"message published").unwrap();
    }

    #[tokio::test]
    async fn test_interface() {
        let handle = PubSub::new();

        let dummy1 = entrypoint(&handle);
        let dummy2 = entrypoint2(&handle);

        tokio::join!(dummy1, dummy2);
    }
}
