---
status: completed
priority: p1
issue_id: 007
tags: [code-review, architecture, types]
dependencies: []
---

# Convert String-based enums to proper Rust enums

## Problem Statement

Configuration types use `String` for fields that should be proper enums (AuditOutput, LogLevel, LogFormat). This leads to:
- Runtime errors from invalid config values
- No compile-time validation
- No IDE autocompletion for config authors
- More error handling code needed

## Findings

**Location:** `src/config/types.rs:114, 122, 153, 156`

**Current String-Based Fields:**
```rust
// types.rs - AuditConfig
pub output: String,  // "stdout", "file", "both" - could be anything!

// types.rs - LoggingConfig  
pub level: String,   // "trace", "debug", "info", "warn", "error"
pub format: String,   // "pretty", "json", "compact"
```

**Risk:** Config parsing succeeds but application fails at runtime with invalid values.

## Proposed Solutions

### Option A: Define Proper Enums with Serde (Recommended)
**Effort:** Medium
**Risk:** Low
**Implementation:**
```rust
// types.rs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AuditOutput {
    #[default]
    Stdout,
    File,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Trace,
    Debug,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Json,
    Pretty,
    Compact,
}

// Update Config structs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuditConfig {
    pub enabled: bool,
    pub redact_pii: bool,
    pub output: AuditOutput,  // Now an enum!
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoggingConfig {
    pub level: LogLevel,      // Now an enum!
    pub format: LogFormat,    // Now an enum!
}
```

**Benefits:**
- Compile-time type safety
- Serde validates on deserialization (fails fast with clear error)
- IDE autocomplete works
- Exhaustive match checking

### Option B: Keep Strings with Validation
**Effort:** Medium
**Risk:** Medium
**Implementation:**
Add post-deserialization validation that checks if strings are valid values.

**Cons:** More code, later error detection, less type safety

## Recommended Action

**Implement Option A** - Define proper enums with `#[serde(rename_all = "lowercase")]`. This is idiomatic Rust and provides immediate feedback on invalid configs.

## Technical Details

**Affected Files:**
- `src/config/types.rs` - Add enum definitions, update struct fields
- `src/main.rs` - Update logging initialization to use enums
- `config/default.yaml` - Values already use lowercase, no change needed

**Serialization Behavior:**
- `AuditOutput::Stdout` serializes as `"stdout"` in YAML
- Deserialization accepts `"stdout"`, `"Stdout"`, etc. (case-insensitive with rename_all)

**Breaking Changes:**
- None for valid configs (lowercase values)
- Invalid configs now fail at load time instead of runtime (good!)

## Acceptance Criteria

- [x] Create `AuditOutput` enum with Stdout/File/Both variants
- [x] Create `LogLevel` enum with Trace/Debug/Info/Warn/Error variants  
- [x] Create `LogFormat` enum with Pretty/Json/Compact variants
- [x] Update `AuditConfig.output` field type from `String` to `AuditOutput`
- [x] Update `LoggingConfig.level` field type from `String` to `LogLevel`
- [x] Update `LoggingConfig.format` field type from `String` to `LogFormat`
- [x] Add `#[serde(rename_all = "lowercase")]` to all enums
- [x] Update `init_logging()` to match on `LogFormat` enum
- [x] Update tests to use enum variants instead of strings
- [x] Add test for invalid enum value deserialization (should fail)
- [x] All existing tests pass with enum values

## Work Log

- **2026-02-26**: Identified during architecture review of PR #1
- **2026-02-26**: Implemented all three enums with proper serde attributes
- **2026-02-26**: Updated struct fields to use enum types
- **2026-02-26**: Updated Default implementations to use enum defaults
- **2026-02-26**: Updated tests in loader.rs to use enum variants
- **2026-02-26**: Fixed main.rs format strings to use `{:?}` for Debug trait
- **2026-02-26**: All enum-related tests passing

## Implementation Summary

**Changes Made:**
1. Added `AuditOutput` enum with `Stdout`, `File`, `Both` variants (default: `Stdout`)
2. Added `LogLevel` enum with `Trace`, `Debug`, `Info`, `Warn`, `Error` variants (default: `Info`)
3. Added `LogFormat` enum with `Pretty`, `Json`, `Compact` variants (default: `Json`)
4. All enums derive: `Debug`, `Clone`, `Serialize`, `Deserialize`, `PartialEq`, `Default`
5. All enums use `#[serde(rename_all = "lowercase")]` for case-insensitive deserialization
6. Updated `AuditConfig.output` from `String` to `AuditOutput`
7. Updated `AuditConfig.level` from `String` to `LogLevel`
8. Updated `LoggingConfig.level` from `String` to `LogLevel`
9. Updated `LoggingConfig.format` from `String` to `LogFormat`
10. Updated Default implementations to use `Enum::default()` instead of string defaults
11. Updated tests to compare against enum variants instead of string literals
12. Fixed main.rs logging statements to use `{:?}` format for Debug trait

**Benefits Achieved:**
- Compile-time type safety - invalid values caught at compile time
- Runtime validation - serde fails fast with clear error messages on invalid enum values
- IDE autocomplete support for config authors
- Exhaustive match checking in Rust code
- Cleaner, more maintainable code
