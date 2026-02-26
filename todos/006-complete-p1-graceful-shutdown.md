---
status: complete
priority: p1
issue_id: 006
tags: [code-review, agent-native, lifecycle]
dependencies: []
---

# Implement graceful shutdown with tokio signal handling

## Problem Statement

The current main function uses an infinite loop with `std::thread::park()` which:
1. Wastes a thread
2. Doesn't allow graceful shutdown
3. Prevents agents from cleanly stopping the service
4. Will drop active connections abruptly in future milestones

## Findings

**Location:** `src/main.rs:40-42`

**Current Code:**
```rust
loop {
    std::thread::park();  // Wastes a thread, no graceful shutdown!
}
```

**Agent Impact:**
- Agents must use SIGKILL to stop service
- No opportunity to clean up resources
- Active connections will be dropped
- Potential data corruption or incomplete audit logs

## Proposed Solutions

### Option A: Use tokio::signal for Graceful Shutdown (Recommended)
**Effort:** Small
**Risk:** Low
**Implementation:**
```rust
use tokio::signal;

#[tokio::main]
async fn main() {
    init_logging();
    let config = config::load_config().expect("Failed to load config");
    
    info!("Starting proxy on {}:{}", config.proxy.host, config.proxy.port);
    
    // Spawn the server task
    let server_handle = tokio::spawn(async move {
        run_server(config).await
    });
    
    // Wait for shutdown signal
    tokio::select! {
        _ = server_handle => {
            info!("Server task completed");
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
        _ = signal::unix::signal(signal::unix::SignalKind::terminate()) => {
            info!("Received SIGTERM, shutting down gracefully...");
        }
    }
    
    // Cleanup would go here (close connections, flush logs, etc.)
    info!("Shutdown complete");
}
```

### Option B: Use tokio::sync::Notify
**Effort:** Medium
**Risk:** Low
**Implementation:**
Use a Notify channel for more complex shutdown scenarios with multiple components.

### Option C: External Shutdown Channel
**Effort:** Medium
**Risk:** Medium
**Implementation:**
Add a shutdown endpoint or signal for remote shutdown (overkill for M0).

## Recommended Action

**Implement Option A** - Use Tokio's signal handling for SIGINT (Ctrl+C) and SIGTERM support. It's idiomatic, simple, and provides the foundation for graceful shutdown.

## Technical Details

**Affected Files:**
- `src/main.rs` - Replace parking loop with signal handling

**Dependencies:**
- Requires tokio "signal" feature (currently enabled by "full", but should be explicit after optimization)

**Platform Considerations:**
- `signal::ctrl_c()` works on all platforms
- `signal::unix::signal()` is Unix-only; add conditional compilation or handle gracefully on Windows

**Future Extension:**
This pattern enables future milestones to:
- Close active connections gracefully
- Flush audit logs
- Save cache state
- Release certificates

## Acceptance Criteria

- [ ] Replace `loop { park() }` with tokio signal handling
- [ ] Handle SIGINT (Ctrl+C) for interactive use
- [ ] Handle SIGTERM for systemd/docker deployments
- [ ] Add shutdown log message
- [ ] Test with Ctrl+C in terminal
- [ ] Test with `kill -TERM <pid>`
- [ ] Verify process exits cleanly (no zombies)

## Work Log

- **2026-02-26**: Identified during performance and agent-native reviews of PR #1
- **2026-02-26**: Implemented graceful shutdown using tokio::signal. Replaced parking loop with signal handlers for SIGINT and SIGTERM. Added shutdown log messages. All 16 tests pass.
