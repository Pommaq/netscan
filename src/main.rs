use std::{sync::Arc, time::Duration};

use clap::Parser;
use entities::arguments::Argument;
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
        portscan::entrypoint(handle.clone(), Argument::Test),
    );

    let dummyscan = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        portscan::dummy_scan(handle.clone(), Argument::Portscan(args.start, args.end)),
    );

    let dummyservice = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        portscan::dummy_service(handle.clone(), Argument::Test),
    );

    tracker.spawn(porscanner);
    tracker.spawn(dummyscan);
    tracker.spawn(dummyservice);
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
