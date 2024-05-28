use std::future::Future;
/// This crate is used for parsing arguments and propagating them to other services. i.e. this starts the entire scan process
use std::sync::Arc;

use clap::Parser;
use entities::portscan;
use pubsub::interface::{Event, PubSubInterface};

#[derive(Parser)]
struct Args {
    // /Inclusive first port to scan
    pub start: u16,
    /// Inclusive last port to scan
    pub end: u16,
}

/// Register the stuff we're listening for. One should not subscribe to entries inside the callback
/// since it could cause race conditions...
pub fn register<T: PubSubInterface>(
    handle: Arc<T>,
) -> Result<impl Future<Output = anyhow::Result<()>>, pubsub::interface::Error> {
    // We dont need to do much here, yet... But let's stay consistent to our implementation
    Ok(scaninitiator(handle))
}

async fn scaninitiator<T: PubSubInterface>(handle: Arc<T>) -> anyhow::Result<()> {
    let args = Args::parse();
    // initialize a portscan of google.com for now until I figure out how we can take arguments in a neat way
    let ports: Vec<u16> = (args.start..args.end + 1).collect();
    const DOMAIN: &str = "google.com";
    log::debug!("scheduling scan for {} on ports {:?} TCP", DOMAIN, &ports);
    let scan_arguments = portscan::Address::new(DOMAIN, &ports);

    handle.publish(
        Event::Scan,
        b"scaninit",
        &entities::serialize(&scan_arguments)?,
    )?;
    Ok(())
}

/* 
Definiera en struct "Entrypoint" som innehåller en funktionspekare samt lista av de events den vill lyssna på samt en den kan publicera.
Skriv ett attribute macro/derive macro som körs över en funktion. Den läser parametrarna för att lista ut vad för grejer den läser/skriver o bildar instanser av "Entrypoint"
där varje instans funktionspekare är en wrapper runt den derive-ade funktionen. Wrappern unpackar listan o anropar den definierade funktionen.
    * Notera att listan av läser/skriver kan inte ges som parameter till definierade grejen.

Skriv en "register" som tar en sekvens av dessa Entrypoint strukter o anropar varje funktionspekare. De får parametern "PubSubInterface" och den interna wrappern kommer registrera/publicera genom den.
*/