use std::{sync::Arc, time::Duration};

use entities::arguments::Argument;
use pubsub::PubSub;
use tokio::signal;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    let handle = Arc::new(PubSub::default());
    // TaskTracker::new() to spawn tasks...

    // 2 second timeout
    let max_timeout = Duration::new(2, 0);

    let cancellation = CancellationToken::new();

    let arguments = Argument::Test;

    let tracker = TaskTracker::new();

    let porscanner = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        portscan::entrypoint(handle.clone(), arguments),
    );

    let dummyscan = scheduler::wrapper(
        max_timeout,
        cancellation.clone(),
        portscan::dummy_scan(handle.clone(), Argument::Test),
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
