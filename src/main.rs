use std::{sync::Arc, time::Duration};

use clap::Parser;
use pubsub::PubSub;
use tokio::signal;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[derive(Parser)]
struct Args {
    // Inclusive first port to scan
    pub start: u16,
    // Inclusive last port to scan
    pub end: u16,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let handle = Arc::new(PubSub::default());

    let args = Args::parse();

    // 10 minute timeout
    let max_timeout = Duration::new(600, 0);

    let cancellation = CancellationToken::new();

    let tracker = TaskTracker::new();

    let porscanner = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        portscan::entrypoint(handle.clone()),
    );
    let initiator = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        scaninit::scaninitiator(handle.clone()),
    );

    tracker.spawn(porscanner);
    tracker.spawn(initiator);
    tracker.close();

    tokio::select! {
        _ = signal::ctrl_c() => {
            // ordered to exit, kill cancellationtoken
            log::info!("Caught exit signal, informing modules...");
            cancellation.cancel();
        }
        _ = tracker.wait() => {
            log::info!("All modules exited, quitting...");
        }
    }

    log::info!("Done");
}
