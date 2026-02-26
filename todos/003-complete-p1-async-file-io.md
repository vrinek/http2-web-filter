---
status: complete
priority: p1
issue_id: 003
tags: [code-review, performance, async]
dependencies: []
---

# Convert blocking I/O to async (std::fs → tokio::fs)

## Problem Statement

The config loader uses blocking file I/O (`std::fs::read_to_string`) inside what will be a Tokio async runtime. Blocking the thread pool can cause latency spikes and reduce throughput under load. While config loading happens only at startup, the pattern should be correct for consistency and future hot-reloading support.

## Findings

**Location:** `src/config/loader.rs:41`

**Current Code:**
```rust
pub async fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // BLOCKING!
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
```

**Impact:**
- Blocks the async runtime thread pool
- Causes latency spikes if config is loaded during request handling (future hot-reload)
- Inconsistent async/await usage

## Proposed Solutions

### Option A: Use tokio::fs for Non-blocking I/O (Recommended)
**Effort:** Small
**Risk:** Low
**Implementation:**
```rust
use tokio::fs;

pub async fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path).await?;  // Non-blocking
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
```

Note: serde_yaml parsing is CPU-bound and should remain synchronous (move to spawn_blocking if it becomes a bottleneck).

### Option B: Use spawn_blocking
**Effort:** Small
**Risk:** Low
**Implementation:**
```rust
pub async fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref().to_owned();
    let content = tokio::task::spawn_blocking(move || {
        std::fs::read_to_string(&path)
    }).await??;
    
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
```

### Option C: Keep Synchronous (Defer)
**Effort:** None
**Risk:** Low (for M0)
**Consideration:** Since config loading happens only at startup, this is acceptable for M0. However, should be fixed before implementing hot-reloading in future milestones.

## Recommended Action

**Implement Option A** - Use `tokio::fs` for async file I/O. It's simple, idiomatic, and prevents technical debt.

## Technical Details

**Affected Files:**
- `src/config/loader.rs` - Change imports and file reading

**Changes Required:**
1. Add `use tokio::fs;`
2. Change `std::fs::read_to_string(path)?` to `fs::read_to_string(path).await?`
3. Ensure all callers are in async context (main.rs already is)

**Dependencies:**
- Requires tokio "fs" feature (currently enabled by "full", but should be explicit after feature optimization)

## Acceptance Criteria

- [ ] Replace `std::fs::read_to_string` with `tokio::fs::read_to_string().await`
- [ ] Add `use tokio::fs;` to loader.rs
- [ ] Ensure tokio "fs" feature is enabled (add explicitly if optimizing features)
- [ ] All tests pass without modification
- [ ] No clippy warnings about blocking I/O in async

## Work Log

- **2026-02-26**: Identified during performance review of PR #1
- **2026-02-26**: Converted blocking file I/O to async using tokio::fs. Changed load_config_from_path to async, updated tests to use #[tokio::test]. All 16 tests pass.
