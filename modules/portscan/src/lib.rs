use std::{sync::Arc, time::Duration};

use anyhow::Context;
use entities::portscan;
use pubsub::interface::{Event, PubSubInterface};
use tokio::{net::TcpStream, time};

const TIMEOUT: u64 = 2;

pub async fn entrypoint<T: PubSubInterface>(handle: Arc<T>) -> anyhow::Result<()> {
    let mut scans = handle
        .subscribe(Event::Scan)
        .context("unable to subscribe :(")?;

    while let Ok(raw) = scans.recv().await {
        let addr: portscan::Address = entities::deserialize(&raw)?;
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
                    handle.publish(Event::PortOpen, &serialized)?;
                }
                Err(_) => {
                    log::debug!("Port {}:{} is closed", addr.addr, port);
                }
            }
        }
    }

    Ok(())
}
