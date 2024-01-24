use std::sync::Arc;

use entities::portscan;
use pubsub::interface::{Event, PubSubInterface};


pub async fn scaninitiator<T: PubSubInterface>(handle: Arc<T>) -> anyhow::Result<()> {
    // initialize a portscan of google.com for now until I figure out how we can take arguments in a neat way
    let args = portscan::Address::new("google.com", &[1, 2, 80, 443]);

    handle.publish(Event::Scan, &entities::serialize(&args)?)?;
    Ok(())
}
