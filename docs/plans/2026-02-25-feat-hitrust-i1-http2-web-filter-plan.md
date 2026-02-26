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

### Implementation Phases

#### Phase 1: Foundation (Week 1-2)

**Goals:** Initialize project, implement core proxy infrastructure, establish CA certificate handling

**Tasks:**
- Initialize Rust project with Cargo.toml and dependencies
- Set up project structure (src/proxy/, src/ca/, src/audit/, src/config/)
- Implement CA certificate loading and validation
- Create dynamic certificate generation with `rcgen`
- Build certificate cache with `moka`
- Implement basic HTTP/2 server with `hyper`

**Deliverables:**
- `Cargo.toml` with all dependencies
- `src/ca/mod.rs` - CA certificate management
- `src/ca/cert_cache.rs` - Certificate caching
- `src/proxy/server.rs` - HTTP/2 server implementation

**Success Criteria:**
- Can load organization-provided CA certificate
- Can generate dynamic certificates for arbitrary domains
- HTTP/2 server accepts connections on configured port

#### Phase 2: Core Proxy Implementation (Week 3-4)

**Goals:** Implement CONNECT tunneling, TLS MITM, and request/response handling

**Tasks:**
- Implement CONNECT method handler for HTTPS proxying
- Add TLS MITM with dynamic certificate generation
- Build HTTP/2 client for upstream connections
- Implement bidirectional data tunneling
- Add timeout and resource management
- Create connection semaphore for resource protection

**Deliverables:**
- `src/proxy/handler.rs` - Request handling with CONNECT support
- `src/proxy/tunnel.rs` - Bidirectional tunneling
- `src/tls/mitm.rs` - MITM interception logic
- `src/tls/config.rs` - HITRUST-compliant TLS configuration

**Success Criteria:**
- Can proxy HTTPS traffic through CONNECT tunnel
- TLS certificates are dynamically generated and cached
- HTTP/2 to HTTP/2 proxying works end-to-end
- Connection timeouts prevent resource exhaustion

#### Phase 3: Filtering Engines (Week 5-6)

**Goals:** Implement all four filtering capabilities

**Tasks:**
- **URL Filtering:** Domain and path pattern matching with regex support
- **DNS Filtering:** Integration with threat intelligence feeds
- **Content Categorization:** Block/allow by category (social media, adult, gambling, file sharing)
- **TLS Certificate Analysis:** Inspect certificates for validity and suspicious indicators
- Implement filtering decision engine with rule precedence
- Add 403 Forbidden responses with category explanations

**Deliverables:**
- `src/filtering/url.rs` - URL pattern matching
- `src/filtering/dns.rs` - DNS threat feed integration
- `src/filtering/categories.rs` - Content categorization
- `src/filtering/engine.rs` - Decision engine
- `config/filters.yaml` - Default filter rules
- `config/categories.yaml` - Content categories

**Success Criteria:**
- URL patterns correctly match domains and paths
- DNS filtering blocks known malicious domains
- Content categories are correctly identified
- TLS certificate validation works
- Blocked requests return 403 with explanation

#### Phase 4: Audit Logging & Configuration (Week 7)

**Goals:** Implement HITRUST-compliant audit logging and configuration system

**Tasks:**
- Implement structured JSON logging with `tracing`
- Add all HITRUST-required fields (timestamp, user identity, URL, action, category, TLS details)
- Create YAML configuration loader with `serde`
- Add Tailscale identity extraction from connection metadata
- Implement log rotation and retention policies
- Add configuration validation

**Deliverables:**
- `src/audit/logger.rs` - Structured audit logging
- `src/config/loader.rs` - YAML configuration loader
- `config/default.yaml` - Default configuration
- `config/example-filters.yaml` - Example filter configurations

**Success Criteria:**
- All proxy decisions logged in structured JSON
- Logs contain all HITRUST-required fields
- Configuration loads from YAML files
- Invalid configurations are rejected with clear errors

#### Phase 5: Testing & Documentation (Week 8)

**Goals:** Comprehensive testing, documentation, and HITRUST evidence preparation

**Tasks:**
- Write unit tests for all modules (following TDD principles from CLAUDE.md)
- Create integration tests for end-to-end proxy scenarios
- Add certificate generation tests
- Test PII redaction
- Test certificate pinning bypass
- Write comprehensive README with HITRUST control mapping
- Create deployment guide for Tailscale exit node
- Document audit log format for assessors

**Deliverables:**
- `tests/unit/` - Unit tests
- `tests/integration/` - Integration tests
- `README.md` - Project documentation
- `docs/HITRUST-CONTROLS.md` - Control mapping documentation
- `docs/DEPLOYMENT.md` - Tailscale deployment guide

**Success Criteria:**
- >80% test coverage
- All HITRUST controls documented with evidence requirements
- Deployment guide enables successful Tailscale exit node setup
- Audit logs are assessor-ready

### Component Architecture

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

- [ ] >80% test coverage
- [ ] All HITRUST Category 10.c controls documented
- [ ] Audit logs validated against HITRUST assessor requirements
- [ ] Deployment guide tested on fresh Tailscale exit node
- [ ] Code review with security focus
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
