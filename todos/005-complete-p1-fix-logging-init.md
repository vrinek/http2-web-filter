---
status: complete
priority: p1
issue_id: 005
tags: [code-review, bug, logging]
dependencies: []
---

# Fix logging initialization to respect config.logging.level

## Problem Statement

The logging initialization in `main.rs` completely ignores the `config.logging.level` setting from the configuration file. It uses a hardcoded default of "info" and only checks the RUST_LOG environment variable.

## Findings

**Location:** `src/main.rs:45-54`

**Current Broken Code:**
```rust
fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));  // Ignores config!
    
    fmt::init();
}
```

**Config Setting That's Ignored:**
```yaml
logging:
  level: "debug"  # This is never used!
```

**Impact:**
- Users cannot control log level via config file
- Must use environment variables (RUST_LOG) which is less convenient
- Documentation and config are misleading

## Proposed Solutions

### Option A: Pass Config to init_logging (Recommended)
**Effort:** Small
**Risk:** Low
**Implementation:**
```rust
fn init_logging(config: &LoggingConfig) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Use config level if env var not set
            EnvFilter::new(&config.level)
        });
    
    // Also support format configuration
    match config.format.as_str() {
        "json" => {
            tracing_subscriber::fmt()
                .json()
                .with_env_filter(filter)
                .init();
        }
        "pretty" => {
            tracing_subscriber::fmt()
                .pretty()
                .with_env_filter(filter)
                .init();
        }
        _ => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .init();
        }
    }
}

fn main() {
    // Load config FIRST (with minimal logging)
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    
    // Then initialize logging with config
    init_logging(&config.logging);
    // ...
}
```

### Option B: Remove LoggingConfig (If Not Needed)
**Effort:** Small
**Risk:** Low
**Consideration:** If we only want to support RUST_LOG env var, remove the confusing config option entirely.

**Cons:** Less user-friendly, agents must manipulate env vars

## Recommended Action

**Implement Option A** - Fix the initialization order (load config before init_logging) and pass the logging config to respect user settings.

## Technical Details

**Affected Files:**
- `src/main.rs` - Reorder initialization, update init_logging signature

**Changes Required:**
1. Move `init_logging()` call after `load_config()`
2. Pass `&config.logging` to `init_logging()`
3. Use `config.level` as default filter
4. Support `config.format` for output formatting

**Edge Cases:**
- If config fails to load, we can't use its logging settings (use eprintln! for errors)
- RUST_LOG env var should still override config (env var priority)

## Acceptance Criteria

- [ ] Load config BEFORE initializing logging
- [ ] Pass `&LoggingConfig` to `init_logging()`
- [ ] Use `config.level` as default filter when RUST_LOG not set
- [ ] Support `config.format` for JSON/pretty/compact output
- [ ] Add test verifying logging respects config level
- [ ] Document that RUST_LOG overrides config.level

## Work Log

- **2026-02-26**: Identified during architecture review of PR #1
- **2026-02-26**: Fixed logging initialization to respect config.logging.level and config.logging.format. Reordered main() to load config before init_logging(). Added as_str() method to LogLevel enum. All 16 tests pass.
