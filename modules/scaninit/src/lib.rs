use std::future::Future;
/// This crate is used for parsing arguments and propagating them to other services. i.e. this starts the entire scan process
use std::sync::Arc;

use entities::portscan;
use pubsub::interface::{Event, PubSubInterface};
use clap::Parser;

#[derive(Parser)]
struct Args {
    // Inclusive first port to scan
    pub start: u16,
    // Inclusive last port to scan
    pub end: u16,
}

/// Register the stuff we're listening for. One should not subscribe to entries inside the callback
/// since it could cause race conditions...
pub fn register<T: PubSubInterface>(handle: Arc<T>) -> Result<impl Future<Output=anyhow::Result<()>>, pubsub::interface::Error> {
    // We dont need to do much here, yet... But let's stay consistent to our implementation
    Ok(scaninitiator(handle))
}

async fn scaninitiator<T: PubSubInterface>(handle: Arc<T>) -> anyhow::Result<()> {

    let args = Args::parse();
    // initialize a portscan of google.com for now until I figure out how we can take arguments in a neat way
    let ports: Vec<u16> = (args.start..args.end+1).collect();
    const DOMAIN: &str = "google.com";
    log::debug!("scheduling scan for {} on ports {:?} TCP", DOMAIN, &ports);
    let scan_arguments = portscan::Address::new(DOMAIN, &ports);

    handle.publish(Event::Scan, b"scaninit",&entities::serialize(&scan_arguments)?)?;
    Ok(())
}
