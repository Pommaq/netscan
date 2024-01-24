use std::{future::Future, time::Duration};
use tokio::time;
use tokio_util::sync::CancellationToken;

/// We need to define an interface for modules to fullfill.
/// It must implement a timeout functionality as well as cancellation functionality and it must allow passing arguments.
/// Likely using tokio::Select and cancellation tokens
/// A wrapper that implements our basic error handling and logging capabilties as well as the possiblity of cancelling modules
pub async fn wrapper<T: Future<Output = anyhow::Result<()>>>(
    timeout: Duration,
    cancellation: CancellationToken,
    module: T,
    identifier: &str,
) {
    let res = time::timeout(timeout, module);

    tokio::select! {
        _ = cancellation.cancelled() => {
            // We are cancelled and need to return
            log::info!("We are cancelled. Returning.");
        },

        result = res => {
            match result {
                Ok(returned_status) => {
                    match returned_status {
                        Ok(_) => log::info!("Module {} finished successfully", identifier),
                        Err(err) => log::error!("Module {} exited with an error: {}",identifier, err),
                    };
                },
                Err(elapsed) => {
                    // We timed out
                    log::error!("Module {} timed out. Elapsed: {}", identifier, elapsed);
                }
            }
        }
    };
    log::debug!("initiator exiting");
}
