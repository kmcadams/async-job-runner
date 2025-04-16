use std::fmt::Display;
use std::time::Duration;

use anyhow::Result;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio::{self, select, signal};

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = HashMap::<u32, JoinHandle<()>>::new();

    for i in 0..10 {
        let job = Job::new(i, format!("job {}", i));
        let id = job.id;
        let handle = tokio::spawn(async move {
            println!("Starting Job {}", job.id);
            let result = job.execute().await;
            println!("Finished: {}", result);
        });

        handles.insert(id, handle);
    }

    println!("Handles {:?}", handles);

    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to register SIGTERM handler");

    select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C..cancelling tasks");
            for handle in handles.values() {
                handle.abort();
            }
        }
        _ = sigterm.recv() => {
            println!("Received SIGTERM");
            for handle in handles.values() {
                handle.abort();
            }
        }
    }

    for (id, handle) in handles {
        match handle.await {
            Ok(_) => println!("Job {} completed before shutdown", id),
            Err(e) if e.is_cancelled() => println!("Job {} was cancelled", id),
            Err(e) => println!("Job {} failed: {:?}", id, e),
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
    async fn execute(&self) -> String {
        println!("Running job {}: {}", self.id, self.payload);
        tokio::time::sleep(Duration::from_millis(5000)).await;
        format!("Job {} ran for 5000ms", self.id)
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job {{ id: {}, payload: {} }}", self.id, self.payload)
    }
}

//TODO:
// - Accept new jobs (simulated structs or boxed functions)
// - Queue them in a bounded async channel
// - Spawn and track task handles using tokio::spawn
// - Allow for graceful shutdown (e.g., on Ctrl+C)
