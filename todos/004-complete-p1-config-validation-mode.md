---
status: complete
priority: p1
issue_id: 004
tags: [code-review, agent-native, cli]
dependencies: []
---

# Add config validation mode (--validate-config flag)

## Problem Statement

Agents cannot validate configuration changes without starting the service. A bad config only surfaces when the service starts, potentially causing downtime or failed deployments. There's no way to check if a config file is valid before deploying it.

## Findings

**Location:** `src/main.rs:13-23`

**Current Behavior:**
```rust
fn main() {
    init_logging();
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    // ... continues to start service
}
```

**Agent Impact:**
- Cannot validate config syntax without starting service
- Deployment automation cannot pre-validate configs
- Risk of deploying broken configs that fail on startup

## Proposed Solutions

### Option A: Add --validate-config Flag (Recommended)
**Effort:** Small
**Risk:** Low
**Implementation:**
```rust
use std::env;

fn main() {
    // Check for validation mode
    if env::args().any(|a| a == "--validate-config") {
        match config::load_config() {
            Ok(_) => {
                println!("Configuration is valid");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Configuration error: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Normal startup path...
    init_logging();
    // ...
}
```

### Option B: Add validate-config Subcommand
**Effort:** Medium
**Risk:** Low
**Implementation:**
Use a simple subcommand pattern:
```rust
match env::args().nth(1).as_deref() {
    Some("validate-config") => validate_and_exit(),
    _ => run_server(),
}
```

### Option C: Separate Validate Binary
**Effort:** Medium
**Risk:** Medium
**Implementation:**
Create a separate `http2-web-filter-validate` binary that just loads and validates config.

**Cons:**
- More complex build
- Another binary to maintain

## Recommended Action

**Implement Option A** - Add `--validate-config` flag. It's the simplest solution that doesn't change the default behavior and allows agents to validate configs.

## Technical Details

**Affected Files:**
- `src/main.rs` - Add flag detection and validation logic

**Agent Usage:**
```bash
# Validate before deploying
./http2-web-filter --validate-config || exit 1

# In CI/CD
http2-web-filter --validate-config && deploy || rollback
```

## Acceptance Criteria

- [ ] Add `--validate-config` flag support in main()
- [ ] When flag is present, load config and exit with status 0 if valid
- [ ] Exit with status 1 and print error if config is invalid
- [ ] Add unit test for validation success case
- [ ] Add unit test for validation failure case
- [ ] Update README with usage example
- [ ] Default behavior (no flag) unchanged

## Work Log

- **2026-02-26**: Identified during agent-native review of PR #1
- **2026-02-26**: Added --validate-config flag support. Validates config and exits with status 0 on success, status 1 on failure. Added 3 unit tests for success and failure cases. All 16 tests pass.
