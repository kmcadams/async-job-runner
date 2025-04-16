use std::fmt::Display;
use std::time::Duration;

use anyhow::Result;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio::{self, select, signal};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    tracing::info!("Async job runner starting...");

    let mut handles = HashMap::<u32, JoinHandle<()>>::new();

    for i in 0..10 {
        let job = Job::new(i, format!("job {}", i));
        let id = job.id;
        let handle = tokio::spawn(async move {
            tracing::info!("Spawning Job {}", job.id);
            let result = job.execute().await;
            tracing::debug!("Tokio task finished: {}", result);
        });

        handles.insert(id, handle);
    }

    tracing::debug!("Handles {:?}", handles);

    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to register SIGTERM handler");

    select! {
        _ = signal::ctrl_c() => {
            tracing::warn!("Received Ctrl+C..cancelling tasks");
            for handle in handles.values() {
                handle.abort();
            }
        }
        _ = sigterm.recv() => {
            tracing::warn!("Received SIGTERM");
            for handle in handles.values() {
                handle.abort();
            }
        }
    }

    for (id, handle) in handles {
        match handle.await {
            Ok(_) => tracing::info!("Job {} completed before shutdown", id),
            Err(e) if e.is_cancelled() => tracing::info!("Job {} was cancelled", id),
            Err(e) => tracing::error!("Job {} failed: {:?}", id, e),
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Job {
    id: u32,
    payload: String,
}

impl Job {
    fn new(id: u32, payload: String) -> Job {
        Job { id, payload }
    }

    #[tracing::instrument(skip(self))]
    async fn execute(&self) -> String {
        tracing::info!(job_id = self.id, "Started executing job");
        tokio::time::sleep(Duration::from_millis(5000)).await;
        tracing::info!(job_id = self.id, "Finished job");
        format!("Job {} ran for 5000ms", self.id)
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job {{ id: {}, payload: {} }}", self.id, self.payload)
    }
}
