use pubsub::{PubSub};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let handle = PubSub::default();

    let portscan = portscan::entrypoint(&handle);
    let orders = portscan::dummy_scan(&handle);
    let service = portscan::dummy_service(&handle);
    tokio::join!(portscan, orders, service);

    log::info!("Done");
}
