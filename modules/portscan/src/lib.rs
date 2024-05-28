use std::future::Future;
use std::{sync::Arc, time::Duration};

use anyhow::Context;
use entities::{filter, portscan};
use log::info;
use pubsub::interface::{Event, PubSubInterface, Subscriber};
use tokio::{net::TcpStream, time};

const TIMEOUT: u64 = 2;

/// Register the stuff we're listening for. One should not subscribe to entries inside the callback
/// since it could cause race conditions...
pub fn register<T: PubSubInterface>(
    handle: Arc<T>,
) -> anyhow::Result<impl Future<Output = anyhow::Result<()>>> {
    let scans = handle
        .subscribe(Event::Scan)
        .context("unable to subscribe to scans :(")?;
    Ok(entrypoint(handle, scans))
}

async fn entrypoint<T: PubSubInterface>(
    handle: Arc<T>,
    mut scans: Subscriber,
) -> anyhow::Result<()> {
    while let Ok(raw) = scans.recv().await {
        let addr: portscan::Address = entities::deserialize(&filter::unwrap(&raw)?)?;

        for port in addr.ports {
            log::info!("Starting scan of {}:{}", addr.addr, port);

            match time::timeout(
                Duration::new(TIMEOUT, 0),
                TcpStream::connect(format!("{}:{}", addr.addr, port)),
            )
            .await
            {
                Ok(_) => {
                    log::info!("Port {}:{} is open", addr.addr, port);
                    let serialized = entities::serialize(&portscan::Port::new(port))?;
                    handle.publish(Event::PortOpen, b"tcp", &serialized).context(format!("Attempted to mark TCP port {port} as open"))?;
                }
                Err(_) => {
                    log::debug!("Port {}:{} is closed", addr.addr, port);
                }
            }
        }
    }
    info!("Portscan exiting");
    Ok(())
}
