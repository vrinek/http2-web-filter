---
date: 2026-02-25
topic: http2-tls-web-filter
---

# HTTP/2 Web Filter with TLS Inspection

## What We're Building

An open-source HTTP/2 proxy written in Rust that performs TLS inspection (MITM) to filter web traffic based on URL/domain patterns. Target audience is system administrators who need to monitor or restrict web access in corporate or managed environments.

The tool will accept user-provided CA certificates for TLS interception, decrypt HTTPS traffic, apply filtering rules from a YAML/JSON configuration file, and either forward allowed traffic or block disallowed requests with customizable responses.

## Why This Approach

**Selected Approach: Focused Core (Approach A)**

We chose a minimal, focused implementation over a feature-rich framework. This aligns with YAGNI principles - build what's needed now, extend later.

## Key Decisions

### Approach A: Focused Core (Recommended)

Build a single-purpose tool that does URL/domain filtering well. Core components:
- HTTP/2 proxy with TLS MITM using user-provided CA
- YAML configuration for allow/deny lists
- Simple logging for blocked requests
- Standalone binary with minimal dependencies

**Pros:**
- Simple to understand and deploy
- Faster to build and test
- Clear scope reduces complexity
- Easy to document for sysadmins

**Cons:**
- Limited extensibility for future filter types
- May require refactoring if requirements grow

**Best when:** You need a working solution quickly and have well-defined filtering needs.

### Approach B: Modular Framework

Build a plugin-based architecture where filters are modules:
- Core proxy engine
- Plugin interface for filters (URL, headers, content)
- Configuration system that loads plugins
- Metrics and observability hooks

**Pros:**
- Highly extensible
- Can add filters without core changes
- Better for long-term evolution

**Cons:**
- Significantly more complex
- Plugin system overhead
- Harder to reason about security

**Best when:** You anticipate needing many filter types and want a platform.

### Approach C: Enterprise Integration

Build for integration with existing infrastructure:
- LDAP/AD integration for user-based policies
- Centralized management API
- Clustering and high availability
- SIEM integration for logging

**Pros:**
- Enterprise-ready from day one
- Fits existing workflows
- Scales to large deployments

**Cons:**
- Massive scope increase
- Requires external dependencies
- Much longer development time

**Best when:** You're building for large enterprise deployments immediately.

## Architecture Decisions

1. **Language:** Rust - Memory safety, high performance, excellent async networking ecosystem
2. **TLS Inspection:** MITM with user-provided CA certificates (simpler implementation than auto-generation)
3. **Filtering Focus:** URL/Domain filtering only (initial scope - can add header/content later)
4. **Configuration:** YAML/JSON files (sysadmin-friendly, version-controllable)
5. **Deployment:** Standalone binary (flexible deployment, minimal dependencies)

## Open Questions

Resolved - all technical decisions made. Ready for planning phase.

## Resolved Questions

### Core Decisions
- Language: **Rust** (chosen for performance and safety)
- Target Users: **System Administrators** (corporate/managed environments)
- Architecture: **MITM Proxy** (full TLS inspection)
- Filtering: **URL/Domain filtering** (focused initial scope)
- Configuration: **YAML/JSON files** (simple, version-controllable)
- TLS Certificates: **User-provided CA** (simpler implementation)

### Technical Implementation Decisions
- **HTTP/2 Library:** hyper (higher-level, supports HTTP/1 & 2 automatically)
- **Protocol Support:** HTTP/2 with HTTP/1.1 fallback (maximum compatibility)
- **Block Response:** Simple 403 Forbidden response
- **Logging Format:** Structured JSON (machine parseable, SIEM-friendly)
- **Concurrency:** Multi-threaded with Tokio async runtime
- **Network Support:** Dual-stack IPv4 + IPv6

## Next Steps

→ `/workflows:plan` for implementation details and technical planning
