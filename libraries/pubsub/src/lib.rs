use interface::{Event,  PubSubInterface, Publisher, Subscriber};

/// Defines traits and structs for publishing/subscribing to data
pub mod interface;

struct Grouping {
    event: Event,
    key: Option<Vec<u8>>,
    publisher: Publisher,
}
pub struct PubSub {
    senders: Vec<Grouping>,
}

impl PubSubInterface for PubSub {
    fn subscribe(&mut self, event: Event, key: Option<&[u8]>) -> Result<Subscriber, interface::Error> {
        // Ensure we only do one memory allocation
        let allocd: Option<Vec<u8>> = key.map(|x| x.to_vec());
        let r = if let Some(x) = self.senders.iter().find(|x|{
            x.event == event && if key.is_some() {x.key == allocd} else {true}
        }) {
            x.publisher.subscribe()
        } else {
            let publisher = Publisher::new(interface::PUBLISHER_SIZE);
            let receiver = publisher.subscribe();
            self.senders.push(Grouping{event, key: allocd, publisher });
            receiver
        };

        Ok(r)
    }

    fn publish(&self, event: Event, key: &[u8], payload: &[u8]) -> Result<(), interface::Error> {
        if let Some(publish) = self.senders.iter().find(|x|{
            x.event == event && if let Some(val) = &x.key { val == key} else{true}
        }) {
            publish.publisher.send(payload.into())?;
        } else {
            log::debug!("Event {:?} with key: {:?} has no subscribers", event, key);
        }

        Ok(())
    }
}

impl PubSub {
    pub fn new() -> Self {
        Self { senders: vec![] }
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


    #[tokio::test]
    async fn test_interface() {
        let mut handle = PubSub::new();
        let mut int = handle.subscribe(Event::Log, None).unwrap();

        handle
            .publish(Event::Log, b"aaa", b"message published")
            .unwrap();
        let data = int.recv().await.unwrap();
        assert_eq!(data, b"message published");
    }

    #[tokio::test]
    async fn test_filter() {
        let mut handle = PubSub::new();
        const PAYLOAD: &[u8] = b"this is an event";
        const PAYLOAD2: &[u8] = b"No :(";

        const KEY: &[u8] = b"aaaaaaaaaaaaaaaaa";
        const KEY2: &[u8] = b"yas";

        let mut int = handle.subscribe(Event::Log, Some(KEY)).unwrap();
        let mut int2 = handle.subscribe(Event::Log, Some(KEY2)).unwrap();


        handle.publish(Event::Log, KEY2, PAYLOAD2).unwrap();
        handle.publish(Event::Log, KEY, PAYLOAD).unwrap();

        assert_eq!(int.recv().await.unwrap(), PAYLOAD);
        assert_eq!(int2.recv().await.unwrap(), PAYLOAD2);



    }
}
