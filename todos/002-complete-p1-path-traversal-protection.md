---
status: complete
priority: p1
issue_id: 002
tags: [code-review, security, path-traversal]
dependencies: []
---

# Add path traversal protection in config loader

## Problem Statement

The `CONFIG_PATH` environment variable is used directly without sanitization in the config loader. An attacker with ability to set environment variables could read arbitrary files using path traversal sequences like `../../../etc/passwd`.

## Findings

**Location:** `src/config/loader.rs:26-30`

**Vulnerable Code:**
```rust
pub fn load_config() -> Result<Config, ConfigError> {
    let config_path =
        env::var("CONFIG_PATH").unwrap_or_else(|_| "./config/default.yaml".to_string());
    load_config_from_path(&config_path)
}
```

**Attack Scenario:**
```bash
CONFIG_PATH="../../../etc/passwd" ./http2-web-filter
```

This could expose sensitive system files if the process has elevated privileges.

**Impact:**
- Information disclosure
- Could expose sensitive files if attacker controls env vars
- Medium severity but easy to exploit

## Proposed Solutions

### Option A: Canonicalize and Validate Path (Recommended)
**Effort:** Medium
**Risk:** Low
**Implementation:**
```rust
use std::path::PathBuf;

pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "./config/default.yaml".to_string());
    
    // Canonicalize and validate path
    let path = PathBuf::from(&config_path);
    let canonical = path.canonicalize()
        .map_err(|_| ConfigError::InvalidPath(config_path.clone()))?;
    
    // Ensure path is within allowed directory (project root or explicit allowlist)
    let current_dir = std::env::current_dir()
        .map_err(|e| ConfigError::IoError(e))?;
    
    if !canonical.starts_with(&current_dir) {
        return Err(ConfigError::PathTraversalDetected(config_path));
    }
    
    load_config_from_path(&canonical)
}
```

Add to ConfigError:
```rust
#[error("Invalid config path: {0}")]
InvalidPath(String),
#[error("Path traversal detected: {0}")]
PathTraversalDetected(String),
```

### Option B: Use Allowed Paths Allowlist
**Effort:** Medium
**Risk:** Low
**Implementation:**
- Maintain list of allowed config paths
- Reject any path not in the allowlist
- More restrictive but less flexible

### Option C: Disable CONFIG_PATH in Production
**Effort:** Small
**Risk:** Medium
**Implementation:**
- Add compile-time or runtime flag to disable env var override
- Force use of default path in production builds

## Recommended Action

**Implement Option A** - Add path canonicalization and directory validation to prevent traversal attacks while maintaining flexibility.

## Technical Details

**Affected Files:**
- `src/config/loader.rs` - Add path validation logic
- `src/config/types.rs` - Add new error variants

**Test Cases:**
```rust
#[test]
fn test_path_traversal_blocked() {
    env::set_var("CONFIG_PATH", "../../../etc/passwd");
    let result = load_config();
    assert!(matches!(result, Err(ConfigError::PathTraversalDetected(_))));
}
```

## Acceptance Criteria

- [ ] Add path canonicalization using `std::fs::canonicalize()`
- [ ] Validate canonical path is within project directory
- [ ] Add new error variants: `InvalidPath` and `PathTraversalDetected`
- [ ] Unit test for path traversal attack prevention
- [ ] Unit test for valid relative paths (still work)
- [ ] Unit test for valid absolute paths within allowed directory
- [ ] All existing tests continue to pass

## Work Log

- **2026-02-26**: Identified during security review of PR #1
- **2026-02-26**: Implemented path validation with canonicalization, added InvalidPath and PathTraversalDetected error variants, added unit tests for path traversal detection and project root finding. All 16 tests pass.
