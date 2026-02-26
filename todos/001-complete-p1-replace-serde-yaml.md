---
status: complete
priority: p1
issue_id: 001
tags: [code-review, security, dependencies]
dependencies: []
---

# Replace deprecated serde_yaml with maintained fork

## Problem Statement

The `serde_yaml` crate is officially deprecated and unmaintained. The version currently in use (0.9.34+deprecated) includes "+deprecated" in its name, indicating the maintainer has archived the project. This is a P1 critical security issue because no security patches will be provided for future vulnerabilities.

## Findings

**Location:** `Cargo.toml:19`

**Current State:**
```toml
serde_yaml = "0.9"
```

**Risk Assessment:**
- Supply chain risk: Dependency is no longer maintained
- No security patches for future YAML parsing vulnerabilities
- Related advisory: RUSTSEC-2025-0068 (for serde_yml fork which has unsoundness issues)
- High impact: YAML parsing is core functionality for configuration loading

**References:**
- [serde_yaml deprecation notice](https://docs.rs/crate/serde_yaml/latest/source/Cargo.toml)
- [GitHub archive status](https://github.com/dtolnay/serde-yaml)
- [RUSTSEC-2025-0068](https://rustsec.org/advisories/RUSTSEC-2025-0068.html)

## Proposed Solutions

### Option A: Use serde_yaml_ng (Recommended)
**Effort:** Small (change dependency, verify compatibility)
**Risk:** Low (drop-in replacement, same API)
**Pros:**
- Actively maintained fork
- Uses safe-libyaml instead of yaml-rust
- Same API as serde_yaml, minimal code changes
- Large community adoption

```toml
[dependencies]
serde_yaml_ng = "0.12"
```

### Option B: Use serde_norway
**Effort:** Small
**Risk:** Low
**Pros:**
- Alternative maintained fork
- Pure Rust implementation
- No unsafe code

```toml
[dependencies]
serde_norway = "0.9"
```

### Option C: Switch to JSON configuration
**Effort:** Medium
**Risk:** Medium (requires config file migration)
**Pros:**
- serde_json is actively maintained by same team as serde
- Better performance than YAML
- No external maintenance risk

**Cons:**
- Breaking change for existing configs
- YAML is more human-readable for complex configs

## Recommended Action

**Implement Option A** - Replace with `serde_yaml_ng` as it's the most popular maintained fork and requires minimal changes.

## Technical Details

**Affected Files:**
- `Cargo.toml` - Update dependency
- `Cargo.lock` - Will be regenerated
- `src/config/loader.rs` - Update imports from `serde_yaml` to `serde_yaml_ng`
- All test files using YAML - Verify compatibility

**Breaking Changes:** None expected, but run full test suite to verify.

## Acceptance Criteria

- [x] Replace `serde_yaml` with `serde_yaml_ng` in Cargo.toml
- [x] Update all imports from `serde_yaml` to `serde_yaml_ng`
- [x] All 16 tests pass (7 lib + 3 main + 6 integration)
- [x] Config loading works with existing YAML files
- [x] Run `cargo build` successfully

## Work Log

- **2026-02-26**: Issue identified during security review of PR #1
- **2026-02-26**: Replaced `serde_yaml` with `serde_yaml_ng` v0.10.0, updated imports in `src/config/loader.rs`, all tests passing
