# Async Job Runner (Rust + Tokio)

A small Rust-based async task runner built with Tokio.  
Designed as an interview prep and portfolio project to demonstrate:

- Async concurrency control using `tokio::mpsc` and `tokio::select!`
- Graceful shutdown handling
- Task spawning, cancellation, and tracking
- Clean, modular system structure for async infrastructure

---

## Project Goals

- Accept jobs into a bounded async queue
- Spawn jobs as concurrent tasks using `tokio::spawn`
- Handle graceful shutdown via `ctrl_c` or a cancellation channel
- Log progress using `tracing`

Stretch goals:
- Track inflight jobs by ID
- Expose minimal admin status
- Extend job types and handle retry/cancellation logic

---
