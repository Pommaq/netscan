use anyhow::Context;
use entities::portscan;
use pubsub::interface::{Event, PubSubInterface};
use tokio::net::TcpStream;

pub async fn entrypoint<T: PubSubInterface>(handle: &T) -> anyhow::Result<()> {
    let mut scans = handle
        .subscribe(Event::Scan)
        .context("unable to subscribe :(")?;
    let raw = scans.recv().await?;

    let addr: portscan::Address = portscan::deserialize(&raw)?;
    log::info!("Starting scan of {}:{}", addr.addr, addr.port);

    let _ = TcpStream::connect(format!("{}:{}", addr.addr, addr.port)).await?;
    // We could connect
    log::info!("it worked");
    let serialized = portscan::serialize(&portscan::Port::new(addr.port))?;
    handle.publish(Event::PortIdentified, &serialized)?;
    Ok(())
}

pub async fn dummy_scan<T: PubSubInterface>(handle: &T) -> anyhow::Result<()> {
    
    let ser = portscan::serialize(&portscan::Address::new("google.com", 443))?;
    log::info!("Ordering scan of google :)");
    handle.publish(Event::Scan, &ser)?;

    Ok(())
}

pub async fn dummy_service<T: PubSubInterface>(handle: &T) -> anyhow::Result<()> {
    let mut sub = handle.subscribe(Event::PortIdentified)?;

    let raw = sub.recv().await?;
    let res: portscan::Port = portscan::deserialize(&raw)?;

    log::info!("Port {} is open", res.port );

    Ok(())
}