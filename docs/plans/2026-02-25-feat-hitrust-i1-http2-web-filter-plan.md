---
title: HITRUST i1 HTTP/2 Web Filter with TLS Inspection
type: feat
status: active
date: 2026-02-25
origin: docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md
---

# HITRUST i1 HTTP/2 Web Filter with TLS Inspection

## Overview

Build an HTTP/2 proxy with TLS MITM inspection in Rust for HITRUST i1 Category 10 (Network Security) compliance. Deployed as a forward proxy at a Tailscale exit node, filtering all outbound web traffic from organizational MacBooks.

This implementation directly addresses HITRUST i1 control requirements while providing a focused, auditable solution for healthcare organizations.

## Problem Statement

Healthcare organizations and business associates pursuing HITRUST i1 certification must demonstrate implementation of Category 10 network security controls, specifically:

- **10.c.1 - URL Filtering:** Block access to unauthorized/malicious websites
- **10.c.2 - DNS Filtering:** Prevent resolution of known malicious domains
- **10.c.3 - TLS Inspection:** Decrypt and analyze TLS-encrypted traffic
- **10.c.4 - Content Categorization:** Enforce organizational policies via content classification

Existing solutions are either too complex (enterprise suites) or lack HITRUST-specific audit capabilities. This tool provides a targeted, auditable implementation.

## Proposed Solution

A focused HTTP/2 proxy written in Rust with the following architecture:

- **Forward proxy** deployed at Tailscale exit node
- **TLS MITM inspection** using organization-provided CA certificates
- **Four filtering engines:** URL, DNS, content categorization, TLS certificate analysis
- **Structured JSON audit logging** for HITRUST assessor evidence
- **YAML configuration** for rules, categories, and threat intelligence feeds

### Why This Approach

**Chosen:** HITRUST i1 Focused Core (Approach A from brainstorm)

This approach was selected because it directly addresses specific HITRUST i1 Category 10 controls without over-engineering. The alternatives (Modular Framework and Enterprise Integration) were rejected as YAGNI violations—implementing features not required for current i1 scope.

**Key advantages:**
- Clear compliance story for assessors
- Audit trail explicitly designed for HITRUST evidence
- Simpler to document and demonstrate control implementation
- Can leverage existing minimal proxy patterns

(see brainstorm: docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md - Section "Approach A: HITRUST i1 Focused Core")

## Technical Approach

### Architecture

```
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│   MacBook   │◄────►│  Tailscale   │◄────►│   Target    │
│   Client    │      │  Exit Node   │      │   Server    │
└─────────────┘      └──────────────┘      └─────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │  Web Filter  │
                    │   (Rust)     │
                    └──────────────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
        ┌─────────┐ ┌──────────┐ ┌───────────┐
        │  URL    │ │   DNS    │ │  Content  │
        │ Filter  │ │  Filter  │ │Categories │
        └─────────┘ └──────────┘ └───────────┘
```

### Implementation Phases (Sliced into Milestones)

Each milestone is a shippable, testable increment. Configuration is YAML-only—no CLI flags. Binary is run as:

```bash
$ ./http2-web-filter                    # Uses ./config/default.yaml
$ CONFIG_PATH=/etc/filter.yaml ./http2-web-filter   # Custom config path
```

---

#### M0: Project Foundation (Days 1-3)

**Goal:** Initialize project, establish entrypoint, config loading, and testing scaffold

**Tasks:**
- [ ] Initialize Cargo project with dependencies (tokio, hyper, rustls, serde, tracing)
- [ ] Create minimal binary entrypoint (`src/main.rs`)
- [ ] Implement YAML config loader (`src/config/loader.rs`)
- [ ] Define configuration schema (proxy port, CA paths, log settings)
- [ ] Set up structured logging with `tracing`
- [ ] Write first unit test for config loading (TDD)
- [ ] Write first integration test scaffold

**Deliverables:**
- `Cargo.toml` with core dependencies
- `src/main.rs` - Simple entrypoint, no flags
- `src/config/loader.rs` - YAML config loading
- `config/default.yaml` - Default configuration template
- Working test suite (`cargo test` passes)

**Acceptance Criteria:**
- Binary compiles and runs without errors
- Config loads from default location or `CONFIG_PATH` env var
- Invalid YAML returns clear error message
- First test demonstrates TDD workflow

---

#### M1: HTTP/2 Server Core (Days 4-7)

**Goal:** Basic HTTP/2 proxy server accepting connections

**Tasks:**
- [ ] Implement HTTP/2 server with `hyper` (TDD: write test first)
- [ ] Add TCP listener with configurable port
- [ ] Implement basic HTTP proxy for non-CONNECT requests
- [ ] Add connection logging to audit trail
- [ ] Handle graceful shutdown (SIGTERM)

**Deliverables:**
- `src/proxy/server.rs` - HTTP/2 server
- `src/proxy/handler.rs` - Basic request handler
- `src/audit/logger.rs` - Initial audit logging
- Tests for server startup and HTTP proxying

**Acceptance Criteria:**
- Server starts and listens on configured port
- Plain HTTP requests are proxied successfully
- Every request logged to audit trail
- Server shuts down gracefully on SIGTERM

---

#### M2: CONNECT Tunneling (Week 2)

**Goal:** HTTPS proxy support via CONNECT method (tunnel only, no inspection)

**Tasks:**
- [ ] Implement CONNECT method handler (TDD)
- [ ] Add bidirectional tunneling for TLS passthrough
- [ ] Handle TLS handshake between client and upstream
- [ ] Add connection timeouts (30s default)
- [ ] Implement connection semaphore (max 10,000)
- [ ] Add upstream connection retry (1x with backoff)

**Deliverables:**
- `src/proxy/handler.rs` - CONNECT support
- `src/proxy/tunnel.rs` - Bidirectional tunneling
- `src/proxy/semaphore.rs` - Connection limiting
- Tests for CONNECT handling and tunneling

**Acceptance Criteria:**
- HTTPS sites accessible through proxy
- CONNECT requests establish TLS tunnel
- Connection limits enforced
- Timeouts prevent resource exhaustion
- Audit logs show "tunnel" action for CONNECT

---

#### M3: TLS MITM & Certificate Management (Week 3)

**Goal:** Full TLS MITM inspection with dynamic certificate generation

**Tasks:**
- [ ] Implement CA certificate loading from PEM (TDD)
- [ ] Add dynamic certificate generation with `rcgen`
- [ ] Build certificate cache with `moka` (1000 entries, 24h TTL)
- [ ] Implement TLS MITM interception logic
- [ ] Add certificate pinning bypass list
- [ ] Configure HITRUST-compliant TLS (TLS 1.2+, strong ciphers)

**Deliverables:**
- `src/ca/mod.rs` - CA certificate management
- `src/ca/cert_cache.rs` - Certificate caching
- `src/tls/mitm.rs` - MITM interception
- `src/tls/config.rs` - HITRUST TLS configuration
- `src/security/pinning.rs` - Certificate pinning bypass

**Acceptance Criteria:**
- CA cert loads successfully
- Dynamic certs generated for arbitrary domains
- Certs cached and reused
- TLS 1.2+ enforced, TLS 1.0/1.1 rejected
- Pinned domains bypass inspection (direct tunnel)
- Full MITM working: proxy → decrypt → forward → encrypt

---

#### M4: URL Filtering (Week 4)

**Goal:** First HITRUST control (10.c.1) - URL pattern filtering

**Tasks:**
- [ ] Design `FilterEngine` trait for Dependency Inversion (TDD)
- [ ] Implement URL pattern matching (regex for domains/paths)
- [ ] Add filter rule precedence logic (blocklist > allowlist)
- [ ] Create 403 Forbidden response with category explanation
- [ ] Implement rule loading from `config/filters.yaml`

**Deliverables:**
- `src/filtering/mod.rs` - Filter trait definitions
- `src/filtering/url.rs` - URL pattern matching
- `src/filtering/engine.rs` - Decision engine
- `config/filters.yaml` - Default URL filter rules
- Tests with mocked filter dependencies

**Acceptance Criteria:**
- URLs match regex patterns correctly
- Blocked domains return 403 with explanation
- Rule precedence works (block > allow)
- Audit logs include rule reference
- HITRUST 10.c.1 evidence ready (config + logs)

---

#### M5: Complete Filtering (Week 5)

**Goal:** All four HITRUST filtering controls working

**Tasks:**
- [ ] Implement DNS filtering with threat intelligence feeds
- [ ] Add content categorization (social media, adult, gambling, file sharing)
- [ ] Implement TLS certificate analysis (validity, suspicious indicators)
- [ ] Create category definitions in `config/categories.yaml`
- [ ] Add integration test covering all four filters

**Deliverables:**
- `src/filtering/dns.rs` - DNS threat feed integration
- `src/filtering/categories.rs` - Content categorization
- `src/filtering/certificate.rs` - TLS cert analysis
- `config/categories.yaml` - Category definitions
- Comprehensive integration test

**Acceptance Criteria:**
- DNS filtering blocks known malicious domains
- Content categories correctly identified
- TLS certificate validation detects issues
- All four HITRUST 10.c controls implemented
- Single integration test validates end-to-end

---

#### M6: HITRUST Audit & Security (Week 6)

**Goal:** Production-ready audit logging and security features

**Tasks:**
- [ ] Implement complete HITRUST audit schema (all required fields)
- [ ] Add Tailscale identity extraction from connection metadata
- [ ] Implement PII redaction (SSN, credit card patterns)
- [ ] Add configuration validation with clear errors
- [ ] Create structured JSON output format

**Deliverables:**
- `src/audit/logger.rs` - Complete audit logging
- `src/audit/redaction.rs` - PII redaction
- `src/config/validation.rs` - Config validation
- `docs/AUDIT-LOG-SCHEMA.md` - Log format documentation

**Acceptance Criteria:**
- All HITRUST-required fields present in logs
- PII patterns redacted before logging
- Tailscale user identity extracted when available
- Invalid configs rejected with helpful errors
- Audit logs are assessor-ready

---

#### M7: Production & CI (Week 7-8)

**Goal:** CI/CD, documentation, full test coverage

**Tasks:**
- [ ] Set up GitHub Actions CI (rustfmt, clippy, test, audit)
- [ ] Achieve >80% test coverage (80% unit, 20% integration)
- [ ] Write comprehensive README
- [ ] Create deployment guide for Tailscale exit node
- [ ] Document HITRUST control mapping
- [ ] Add example configurations

**Deliverables:**
- `.github/workflows/ci.yml` - CI pipeline
- `README.md` - Quick start and overview
- `docs/DEPLOYMENT.md` - Tailscale setup guide
- `docs/HITRUST-CONTROLS.md` - Control mapping
- All quality gates passing

**Acceptance Criteria:**
- CI passes on every commit (zero clippy warnings)
- Test coverage >80%
- Documentation complete for deployment
- All acceptance criteria from M0-M6 verified
- Ready for HITRUST assessor review

### Component Architecture

## Implementation Approach

### Development Methodology

**Test-Driven Development (TDD)** is mandatory for all production code. The workflow follows classic TDD:
1. Write tests first, run them, expect them to fail
2. Write minimal code to make tests pass
3. Refactor if necessary, keeping tests green

**Prototypes:** Quick experiments and exploratory spikes are exempt from TDD. This includes:
- Library feasibility testing (e.g., testing a new crate's API)
- CLI UX prototypes for user feedback
- Performance benchmarks

Prototypes are disposable code—if they prove valuable, they are rewritten properly with TDD.

### Git Workflow

**Commit Frequency:** Commit early and often. Use short, single-line commit messages (50 characters or less).

**Examples:**
- `Add CA certificate loading from PEM`
- `Implement dynamic cert generation with rcgen`
- `Add URL filter pattern matching`
- `Fix TLS handshake timeout handling`

**Rationale:** Small commits enable:
- Easy rollback when issues arise
- Clear history of incremental progress
- Better code review (small, focused changes)
- Reduced risk of losing work

**Workflow:**
1. Write tests
2. Run tests (expect failure)
3. Implement feature
4. Run tests (expect pass)
5. `git add` and `git commit` with short message
6. Repeat

### Testing Strategy

**Unit vs Integration Split: 80/20**

- **80% Unit Tests:** Test individual components in isolation using `mockall` for dependency mocking
- **20% Integration Tests:** One comprehensive integration test walks through the entire application stack

**Testing Tools:**
- `mockall` for generating mocks from traits
- Trait-based Dependency Injection enables test doubles
- `tokio::test` for async test support
- `cargo test` with coverage reporting (target: >80% coverage)

**Why This Approach:**
The 80/20 split balances thoroughness with efficiency. Unit tests provide fast feedback during development, while a single comprehensive integration test validates end-to-end behavior. The trait-based architecture enables easy mocking without heavyweight frameworks.

### Code Quality & CI

**GitHub Actions CI Pipeline:**
- `rustfmt` check (enforce consistent formatting)
- `clippy` with strict lints (`-D warnings`, deny all warnings)
- `cargo test` with coverage reporting (80% minimum threshold)
- `cargo audit` for security vulnerability scanning
- Multi-platform builds (Linux, macOS)

**Code Quality Gates:**
- Zero warnings from clippy
- All tests passing
- No `unwrap()` or `expect()` in production paths
- Proper error handling with `thiserror` or `anyhow`

### SOLID Principles

All code will follow SOLID principles:

**Single Responsibility:** Each module handles one concern:
- `ca/` - Certificate authority only
- `proxy/` - HTTP/2 proxy logic only
- `filtering/` - Filter engines only
- `audit/` - Logging only

**Dependency Inversion:** High-level modules depend on abstractions (traits), not concrete implementations:
```rust
// FilterEngine trait enables swapping implementations
#[async_trait]
pub trait FilterEngine: Send + Sync {
    async fn check(&self, request: &Request) -> FilterDecision;
}

// Concrete implementations
pub struct UrlFilterEngine { ... }
pub struct CategoryFilterEngine { ... }
```

This enables:
- Unit testing with mock implementations
- Swapping production implementations without code changes
- Future extensibility (plugin system, external services)

**Open/Closed:** Modules are open for extension (new filters, new log outputs) but closed for modification (existing code doesn't change).

**Liskov Substitution:** All implementations of a trait must be interchangeable without breaking behavior.

**Interface Segregation:** Keep traits small and focused (e.g., `FilterEngine` for filtering, `AuditLogger` for logging, separate from each other).

### Technology Stack Details

**Rust Edition:** 2021 (stable, production-ready)

**Key Dependencies:**
- `tokio` ^1.39 - Async runtime
- `hyper` ^1.0 - HTTP/2 server/client
- `rustls` ^0.23 - TLS stack (FIPS-capable via aws-lc-rs)
- `rcgen` ^0.13 - Dynamic certificate generation
- `moka` ^0.12 - Certificate caching
- `tracing` ^0.1 - Structured logging
- `serde` ^1.0 - YAML configuration
- `mockall` ^0.12 - Test mocking

```rust
// src/lib.rs - Module structure
pub mod ca;           // Certificate authority management
pub mod proxy;        // HTTP/2 proxy implementation
pub mod tls;          // TLS configuration and MITM
pub mod filtering;    // URL, DNS, category filtering
pub mod audit;        // Structured logging
pub mod config;       // YAML configuration
pub mod security;     // PII redaction, pinning bypass
```

### Key Implementation Details

#### TLS MITM with User-Provided CA

```rust
// src/ca/mod.rs
pub struct CertificateAuthority {
    issuer: rcgen::Issuer<'static, rcgen::KeyPair>,
    cert_cache: Cache<String, Arc<ServerConfig>>,
}

impl CertificateAuthority {
    pub fn from_pem(ca_cert: &str, ca_key: &str) -> Result<Self> {
        let signing_key = KeyPair::from_pem(ca_key)?;
        let issuer = Issuer::from_ca_cert_pem(ca_cert, signing_key)?;
        
        Ok(Self {
            issuer,
            cert_cache: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(86400))
                .build(),
        })
    }
    
    pub async fn gen_server_config(&self, authority: &str) -> Result<Arc<ServerConfig>> {
        // Generate or retrieve cached certificate
    }
}
```

#### HTTP/2 Proxy Handler

```rust
// src/proxy/handler.rs
async fn proxy_handler(
    req: Request<Body>,
    remote_addr: SocketAddr,
    ca: Arc<CertificateAuthority>,
    filter_engine: Arc<FilterEngine>,
) -> Result<Response<Body>> {
    if req.method() == Method::CONNECT {
        return handle_connect(req, ca).await;
    }
    
    // Apply filtering rules
    let decision = filter_engine.check(&req).await;
    
    // Log decision
    audit_logger::log_proxy_event("request", &remote_addr, &target, &decision.action, details);
    
    if decision.action == Action::Block {
        return create_block_response(&decision.category);
    }
    
    // Forward request
    forward_request(req).await
}
```

#### Structured Audit Logging

```rust
// src/audit/logger.rs
pub fn log_proxy_event(
    event_type: &str,
    client_addr: &SocketAddr,
    target: &str,
    action: &str,
    category: Option<&str>,
    rule_ref: Option<&str>,
) {
    let event = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "event_type": event_type,
        "client": {
            "ip": client_addr.ip().to_string(),
            "port": client_addr.port(),
            "tailscale_user": extract_tailscale_identity(client_addr),
        },
        "target": target,
        "action": action,
        "category": category,
        "rule_reference": rule_ref,
        "tls_version": "TLSv1.3",
        "cipher_suite": "TLS_AES_256_GCM_SHA384",
    });
    
    info!(event = %event, "proxy_decision");
}
```

## Alternative Approaches Considered

### Alternative 1: Modular Plugin Framework

**Rejected:** Would add significant complexity (plugin system, API boundaries) without addressing immediate HITRUST i1 needs. The YAGNI principle applies—we don't need extensibility for categories we haven't identified yet.

### Alternative 2: Enterprise Integration

**Rejected:** Requires LDAP/AD integration, SIEM connectors, and HA clustering. These are r2-level requirements, not i1. Building them now would be speculative and delay the core compliance functionality.

### Alternative 3: Off-the-Shelf Proxy

**Rejected:** Existing solutions (Squid, Privoxy) lack HITRUST-specific audit logging and Tailscale integration. Custom implementation ensures compliance evidence is exactly what's needed.

(see brainstorm: docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md - Section "Key Decisions")

## System-Wide Impact

### Interaction Graph

```
MacBook Client → Tailscale VPN → Exit Node (proxy binds to 0.0.0.0:8080)
                    ↓
            [HTTP CONNECT Request]
                    ↓
         Proxy Server (hyper)
                    ↓
            [TLS Handshake]
                    ↓
         Certificate Authority (rcgen)
           - Generate cert for target domain
           - Cache in moka
                    ↓
            [Filter Decision]
                    ↓
         Filter Engine
           - Check URL patterns
           - Check DNS blocklist
           - Categorize content
                    ↓
         [Allow] → Forward to target server
         [Block] → Return 403 with category
                    ↓
         Audit Logger (tracing)
           - Write structured JSON to stdout/file
           - Include HITRUST fields
```

### Error & Failure Propagation

| Layer | Error Type | Handling | Impact |
|-------|-----------|----------|--------|
| TLS Handshake | `rustls::Error` | Log + close connection | Client sees connection failure |
| Certificate Generation | `rcgen::Error` | Log + bypass inspection | Direct tunnel without MITM |
| Filter Engine | `FilterError` | Log + default-deny | Request blocked with 403 |
| Upstream Connection | `hyper::Error` | Log + retry (1x) | Client sees 502 if fails |
| Audit Logging | `std::io::Error` | Write to stderr backup | Compliance gap if persistent |

**Retry Strategy:**
- Upstream connection failures: 1 retry with 100ms backoff
- TLS handshake failures: No retry (cert issue is persistent)
- Filter engine failures: Default-deny (fail secure)

### State Lifecycle Risks

**Certificate Cache:**
- Risk: Expired certificates in cache cause TLS failures
- Mitigation: 24-hour TTL on cache entries, periodic cleanup
- Cleanup: Cache eviction is automatic (moka handles this)

**Connection State:**
- Risk: Partial failures leave connections hanging
- Mitigation: 30-second timeout on all operations
- Cleanup: Tokio tasks are cancelled on timeout, resources freed

**Audit Log Files:**
- Risk: Log rotation causes data loss
- Mitigation: Use structured JSON to stdout (let log aggregator handle rotation)
- Backup: Duplicate critical events to stderr

### API Surface Parity

This is a standalone binary, not a library. No public API surface to maintain. Configuration is the only interface:

- **File-based:** YAML configuration files
- **Signal-based:** SIGHUP for configuration reload (future enhancement)
- **No REST API:** Not needed for i1 scope

### Integration Test Scenarios

1. **End-to-End HTTPS Proxy:**
   - Client makes HTTPS request through proxy
   - Proxy generates certificate, decrypts traffic
   - URL filter blocks malicious domain
   - Audit log shows block event with all fields

2. **Certificate Pinning Bypass:**
   - Request to `accounts.google.com`
   - Proxy recognizes pinned domain
   - Direct tunnel without MITM
   - Audit log shows "bypass" action

3. **HTTP/2 to HTTP/2:**
   - Client sends HTTP/2 request
   - Proxy maintains HTTP/2 to upstream
   - Headers and trailers preserved
   - Performance comparable to direct connection

4. **Filter Rule Precedence:**
   - Domain in both allowlist and blocklist
   - Blocklist takes precedence (fail secure)
   - Audit log shows rule reference

5. **High Concurrency:**
   - 1000 concurrent connections
   - Memory usage stays under 512MB
   - No certificate generation failures
   - Response times < 100ms

## Acceptance Criteria

### Functional Requirements

- [ ] HTTP/2 proxy accepts connections on configurable port (default 8080)
- [ ] TLS MITM inspection with organization-provided CA certificate
- [ ] URL filtering with regex pattern matching for domains and paths
- [ ] DNS filtering integration with configurable threat intelligence feeds
- [ ] Content categorization with predefined categories (social media, adult, gambling, file sharing)
- [ ] TLS certificate analysis and validation
- [ ] 403 Forbidden responses for blocked requests with category explanation
- [ ] Structured JSON audit logging with all HITRUST-required fields
- [ ] YAML configuration for rules, categories, and settings
- [ ] Certificate pinning bypass for known pinned domains

### Non-Functional Requirements

- [ ] TLS 1.2+ only (disable TLS 1.0/1.1 for HITRUST compliance)
- [ ] FIPS 140-3 validated cryptography (via `aws-lc-rs` feature)
- [ ] Memory-safe implementation (Rust guarantees this)
- [ ] Zeroization of cryptographic keys on drop
- [ ] PII redaction in audit logs (SSN, credit card patterns)
- [ ] Resource limits: max 10,000 concurrent connections
- [ ] Connection timeouts: 30 seconds for all operations
- [ ] Certificate cache: max 1000 entries, 24-hour TTL

### Quality Gates

- [ ] >80% test coverage (80% unit, 20% integration)
- [ ] All HITRUST Category 10.c controls documented
- [ ] Audit logs validated against HITRUST assessor requirements
- [ ] Deployment guide tested on fresh Tailscale exit node
- [ ] Code review with security focus
- [ ] CI passing: rustfmt, clippy (zero warnings), tests, audit
- [ ] No `unwrap()` or `expect()` in production paths
- [ ] All errors properly logged and handled

## Success Metrics

### Compliance Metrics

- **HITRUST i1 Control Coverage:** 100% of Category 10.c controls demonstrable via audit logs
- **Assessor Evidence Quality:** Audit logs contain all required fields; configuration is version-controllable
- **Time to Evidence:** < 1 hour to generate compliance evidence for any time period

### Performance Metrics

- **Throughput:** Handle 1000 concurrent connections with < 10% CPU usage on 2-core exit node
- **Latency:** Add < 50ms latency vs. direct connection (measured via curl)
- **Memory Usage:** < 512MB RAM under normal load (includes certificate cache)
- **Certificate Generation:** < 100ms per new domain (cached thereafter)

### Operational Metrics

- **Uptime:** 99.9% availability (measured via health check endpoint)
- **Configuration Reload:** < 5 seconds to apply new filter rules
- **Audit Log Volume:** < 1GB per day for 100-user organization

## Dependencies & Prerequisites

### System Requirements

- **OS:** Linux (Ubuntu 22.04 LTS or similar)
- **Rust:** 1.75+ (2021 edition)
- **Tailscale:** Installed and configured as exit node
- **Certificates:** Organization CA certificate and private key (PEM format)

### External Dependencies

| Crate | Version | Purpose | HITRUST Relevance |
|-------|---------|---------|-------------------|
| tokio | ^1.39 | Async runtime | Resource management |
| hyper | ^1.0 | HTTP/2 server | Protocol support |
| rustls | ^0.23 | TLS stack | FIPS 140-3 capable |
| rcgen | ^0.13 | Certificate generation | CA chain validation |
| moka | ^0.12 | Certificate caching | Performance |
| tracing | ^0.1 | Structured logging | Audit evidence |
| serde | ^1.0 | YAML configuration | Version control |

### Tailscale Configuration

- Exit node must advertise routes (`--advertise-exit-node`)
- MacBooks must use exit node (`--exit-node=<node-ip>`)
- Proxy must bind to `0.0.0.0:8080` (all interfaces)

### Network Requirements

- Outbound HTTPS (443) to all destinations
- Outbound HTTP (80) for OCSP/certificate validation
- Inbound TCP on proxy port (default 8080)
- DNS resolution for threat intelligence feed URLs

## Risk Analysis & Mitigation

### Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Certificate pinning causes app failures | High | Medium | Maintain bypass list for known pinned domains |
| Memory exhaustion under load | Medium | High | Connection semaphore (10,000 max), certificate cache limits |
| TLS handshake failures | Low | Medium | Fallback to direct tunnel without inspection |
| Audit log data loss | Low | Critical | Write to stdout+file, use external log aggregator |
| CA certificate compromise | Low | Critical | Document key management procedures, use HSM if available |

### Compliance Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Audit logs missing required fields | Medium | High | Schema validation in tests, comprehensive test coverage |
| Filter rules don't cover all threats | Medium | Medium | Regular threat intelligence feed updates, documented coverage gaps |
| Assessor questions implementation | Medium | Medium | Detailed documentation of design decisions, HITRUST control mapping |

### Operational Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Tailscale exit node failure | Medium | High | Document failover procedures (start with single exit node) |
| Configuration errors block legitimate traffic | Medium | High | Test configuration before deployment, default-deny with logging |
| Performance degradation over time | Low | Medium | Monitor memory usage, certificate cache size; restart if needed |

## Resource Requirements

### Development Team

- **1 Senior Rust Developer:** Core implementation, TLS handling, async architecture
- **1 Security Engineer:** HITRUST compliance validation, audit log review
- **1 DevOps Engineer:** Tailscale deployment, monitoring, log aggregation

### Time Estimate

- **Phase 1 (Foundation):** 2 weeks
- **Phase 2 (Core Proxy):** 2 weeks
- **Phase 3 (Filtering):** 2 weeks
- **Phase 4 (Audit/Config):** 1 week
- **Phase 5 (Testing/Docs):** 1 week
- **Total:** 8 weeks (1 developer full-time)

### Infrastructure

- **Development:** Local machine with Rust toolchain
- **Testing:** Docker environment with test CA certificates
- **Staging:** Tailscale exit node (can be existing test node)
- **Production:** Dedicated Tailscale exit node (2 CPU, 4GB RAM minimum)

## Future Considerations

### Extensibility

The modular architecture supports future enhancements:

- **Plugin system:** Add custom filters without core changes
- **REST API:** Remote configuration management
- **SIEM integration:** Direct export to Splunk/Datadog
- **User-based policies:** Tailscale identity integration for per-user rules
- **Transparent mode:** iptables/nftables integration for gateway deployment

### HITRUST r2 Readiness

To evolve toward r2 certification:

- Add centralized policy management API
- Implement user/group-based access control
- Add SIEM integration with alerting
- Build high-availability clustering
- Add content scanning (malware detection)

These are explicitly out of scope for i1 but the architecture supports them.

## Documentation Plan

### Code Documentation

- **Rustdoc:** All public APIs documented
- **Module-level docs:** Architecture overview in each module
- **Examples:** Configuration examples in `config/examples/`

### User Documentation

- **README.md:** Quick start guide
- **docs/DEPLOYMENT.md:** Tailscale exit node setup
- **docs/CONFIGURATION.md:** Filter rules, categories, threat feeds
- **docs/HITRUST-CONTROLS.md:** Control mapping with evidence requirements
- **docs/TROUBLESHOOTING.md:** Common issues and solutions

### Operational Documentation

- **docs/OPERATIONS.md:** Monitoring, log aggregation, alerting
- **docs/SECURITY.md:** Key management, certificate rotation
- **docs/COMPLIANCE.md:** Preparing for HITRUST assessor

## HITRUST i1 Control Mapping

This implementation provides evidence for the following HITRUST i1 Category 10 controls:

### 10.c.1 - URL Filtering

**Control:** Implement pattern-based URL filtering to block access to unauthorized or malicious websites

**Implementation:**
- `src/filtering/url.rs` - Regex-based pattern matching
- `config/filters.yaml` - Block/allow patterns
- Audit logs show matched patterns and actions

**Evidence:**
- Configuration file with URL patterns
- Audit logs showing blocked URLs with rule references
- Test cases demonstrating pattern matching

### 10.c.2 - DNS Filtering

**Control:** Integration with DNS filtering services to prevent resolution of known malicious domains

**Implementation:**
- `src/filtering/dns.rs` - Threat intelligence feed integration
- Periodic feed updates (configurable)
- Audit logs show DNS block decisions

**Evidence:**
- Configuration showing feed URLs
- Audit logs with DNS block events
- Documentation of feed update procedures

### 10.c.3 - TLS Inspection

**Control:** Decryption and analysis of TLS-encrypted traffic to inspect content and validate certificates

**Implementation:**
- `src/tls/mitm.rs` - TLS MITM with dynamic certificates
- `src/ca/` - Certificate authority management
- `src/tls/config.rs` - HITRUST-compliant TLS settings (TLS 1.2+, strong ciphers)

**Evidence:**
- TLS configuration enforcing minimum version
- Certificate chain validation logs
- Bypass list for pinned domains (documented)

### 10.c.4 - Content Categorization

**Control:** Classification and filtering of web content by category to enforce organizational policies

**Implementation:**
- `src/filtering/categories.rs` - Content classification
- `config/categories.yaml` - Category definitions
- Block responses include category explanation

**Evidence:**
- Category configuration file
- Audit logs with category classifications
- Test cases for category matching

(see brainstorm: docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md - Section "HITRUST i1 Control Mapping")

## Sources & References

### Origin

- **Brainstorm document:** [docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md](docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md)
  - Key decisions carried forward:
    - HITRUST i1 Category 10 focus (Approach A selected)
    - Forward proxy at Tailscale exit node deployment
    - Rust with hyper, rustls, rcgen, moka stack
    - Four filtering engines: URL, DNS, content, TLS
    - Structured JSON audit logging

### Internal References

- **CLAUDE.md:** TDD requirements, SOLID principles, git workflow
- **Repository:** Greenfield project - no existing patterns

### External References

- **Rustls Documentation:** https://docs.rs/rustls/latest/rustls/
- **Hyper HTTP/2 Guide:** https://hyper.rs/guides/1/server/hello-world/
- **rcgen Certificate Generation:** https://docs.rs/rcgen/latest/rcgen/
- **HITRUST i1 Requirements:** https://hitrustalliance.net/assessments-and-certifications/i1
- **Production Example:** http-mitm-proxy by hatoo (https://github.com/hatoo/http-mitm-proxy)

### Related Work

- None - this is a new implementation

---

**Plan written to:** `docs/plans/2026-02-25-feat-hitrust-i1-http2-web-filter-plan.md`

**Next Steps:**
1. Review and refine plan
2. Run `/deepen-plan` for additional research
3. Run `/technical_review` for code-focused feedback
4. Begin implementation with `/workflows:work`
