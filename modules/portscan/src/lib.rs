use std::{sync::Arc, time::Duration};

use anyhow::Context;
use entities::{arguments::Argument, portscan};
use pubsub::interface::{Event, PubSubInterface};
use tokio::{net::TcpStream, time};

pub async fn entrypoint<T: PubSubInterface>(
    handle: Arc<T>,
    _arguments: Argument,
) -> anyhow::Result<()> {
    let mut scans = handle
        .subscribe(Event::Scan)
        .context("unable to subscribe :(")?;

    while let Ok(raw) = scans.recv().await {
        let addr: portscan::Address = portscan::deserialize(&raw)?;
        log::info!("Starting scan of {}:{}", addr.addr, addr.port);

        match time::timeout(
            Duration::new(2, 0),
            TcpStream::connect(format!("{}:{}", addr.addr, addr.port)),
        )
        .await
        {
            Ok(_) => {
                log::info!("Port {}:{} is open", addr.addr, addr.port);
                let serialized = portscan::serialize(&portscan::Port::new(addr.port))?;
                handle.publish(Event::PortIdentified, &serialized)?;
            }
            Err(_) => {
                log::debug!("Port {}:{} is closed", addr.addr, addr.port);
            }
        }
    }

    Ok(())
}

pub async fn dummy_scan<T: PubSubInterface>(
    handle: Arc<T>,
    _arguments: Argument,
) -> anyhow::Result<()> {
    if let Argument::Portscan(start, end) = _arguments {
        for port in start..end + 1 {
            let domain = "google.com";
            let ser = portscan::serialize(&portscan::Address::new(domain, port))?;
            log::info!("Ordering scan of {}:{}", domain, port);
            handle.publish(Event::Scan, &ser)?;
        }
    } else {
        log::error!("No arguments found :/");
    }

    Ok(())
}

pub async fn dummy_service<T: PubSubInterface>(
    handle: Arc<T>,
    _arguments: Argument,
) -> anyhow::Result<()> {
    let mut sub = handle.subscribe(Event::PortIdentified)?;

    let raw = sub.recv().await?;
    let res: portscan::Port = portscan::deserialize(&raw)?;

    log::info!("Port {} is open", res.port);

    Ok(())
}
