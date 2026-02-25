---
date: 2026-02-25
topic: http2-tls-web-filter-hitrust-i1
---

# HTTP/2 Web Filter with TLS Inspection for HITRUST i1 Compliance

## What We're Building

An HTTP/2 proxy written in Rust that performs TLS inspection (MITM) to filter web traffic for HITRUST i1 Category 10 (Network Security) compliance. The tool will be deployed as a forward proxy at a Tailscale exit node, filtering all outbound web traffic from organizational MacBooks.

**Target Users:** Healthcare organizations and business associates pursuing HITRUST i1 certification who need to demonstrate implementation of network security controls.

**Deployment Model:** Forward proxy running as a Tailscale exit node. Client MacBooks connect via Tailscale VPN, with all internet-bound traffic routing through the exit node where this proxy performs filtering.

**Core Capabilities:**
- HTTP/2 and HTTP/1.1 proxy with TLS MITM using organization-provided CA certificates
- URL filtering to block unauthorized/malicious websites
- DNS filtering integration to block malicious domains
- Content categorization filtering (social media, adult content, gambling, etc.)
- TLS/SSL inspection to analyze encrypted traffic for threats
- Structured audit logging for HITRUST assessor review
- YAML/JSON configuration for allow/deny lists and categories

## Why This Approach

**Selected Approach: HITRUST i1-Focused Implementation**

The original minimal proxy design is well-suited for HITRUST i1 requirements. Rather than building a generic web filter, we're targeting the specific Category 10 controls that HITRUST assessors evaluate. This keeps the scope focused while ensuring compliance needs are met.

**Key Design Decisions:**
1. **Tailscale Integration:** Modern VPN approach provides encrypted tunnels for remote workers while centralizing filtering at the exit node. Audit logs can correlate traffic with Tailscale identities.
2. **Forward Proxy Model:** Aligns with existing brainstorm and works well with Tailscale exit nodes. Simpler certificate management than transparent interception.
3. **Category 10 Focus:** Implements the specific controls HITRUST i1 assessors look for (URL filtering, DNS filtering, TLS inspection, content categorization).

## Key Decisions

### Approach A: HITRUST i1 Focused Core (Recommended)

Build a single-purpose tool targeting HITRUST i1 Category 10 controls:
- HTTP/2 proxy with TLS MITM for traffic inspection
- URL filtering with pattern matching for blocked domains/paths
- DNS filtering service integration (block lists, threat intelligence feeds)
- Content categorization engine with configurable category filters
- Structured audit logging (JSON format) capturing: timestamp, user identity (Tailscale), URL, action (allow/block), category matched, TLS certificate info
- YAML configuration for filter rules, categories, and threat feeds
- Deploy as Tailscale exit node forward proxy

**Pros:**
- Directly addresses HITRUST i1 Category 10 control requirements
- Clear compliance story for assessors
- Simpler to document and demonstrate control implementation
- Audit trail explicitly designed for HITRUST evidence
- Can leverage existing minimal proxy codebase

**Cons:**
- Limited to forward proxy model (transparent gateway would require additional work)
- DNS filtering relies on external threat intelligence feeds
- Content categorization requires maintaining category databases

**Best when:** You need to satisfy specific HITRUST i1 Category 10 requirements with a focused, auditable solution.

### Approach B: Modular HITRUST Framework

Build a plugin-based system supporting multiple HITRUST categories:
- Core proxy engine with plugin architecture
- Plugins for different control categories (network security, malware protection, access control)
- Pluggable threat intelligence sources
- Integration with SIEM for centralized logging
- Support for both forward proxy and transparent gateway modes

**Pros:**
- Can address multiple HITRUST categories beyond just Category 10
- Extensible for future compliance requirements
- Could evolve toward r2 readiness

**Cons:**
- Significantly more complex development effort
- Plugin system overhead reduces performance
- May implement features not needed for current i1 scope (YAGNI violation)

**Best when:** You anticipate needing to address multiple HITRUST control categories and want a platform that grows with your compliance journey.

### Approach C: Enterprise Integrated Solution

Build for immediate enterprise deployment with full HITRUST r2 alignment:
- Active Directory/LDAP integration for user-based policies
- Centralized management API for policy distribution
- High availability clustering
- Integration with endpoint detection and response (EDR)
- Full SIEM integration with alerting

**Pros:**
- Enterprise-ready architecture
- Can scale to r2 requirements later
- Centralized management reduces operational overhead

**Cons:**
- Massive scope increase beyond i1 needs
- Requires additional infrastructure (AD/LDAP, SIEM)
- Much longer development time
- YAGNI - building for hypothetical future requirements

**Best when:** You're a large healthcare organization with existing enterprise infrastructure and need immediate integration.

## Architecture Decisions

1. **Language:** Rust - Memory safety, high performance, excellent async networking (critical for exit node handling multiple concurrent connections)

2. **Deployment Architecture:** Forward proxy at Tailscale exit node
   - Tailscale provides encrypted mesh VPN from MacBooks
   - Exit node configuration routes all internet traffic through proxy
   - Proxy performs TLS MITM and applies filtering rules
   - Audit logs capture Tailscale user identity for accountability

3. **TLS Inspection:** MITM with organization-provided CA certificates
   - Simpler than auto-generation and trust establishment
   - Centralized certificate management at exit node
   - Required for content inspection and Category 10 compliance

4. **Filtering Capabilities:**
   - **URL Filtering:** Domain and path pattern matching (regex support)
   - **DNS Filtering:** Integration with threat intelligence feeds (malware domains, phishing sites)
   - **Content Categorization:** Block/allow by category (social media, adult, gambling, file sharing, etc.)
   - **TLS Certificate Analysis:** Inspect certificates for validity, CA trust, and suspicious indicators

5. **Configuration:** YAML files
   - Filter rules and patterns
   - Category definitions
   - Threat intelligence feed URLs
   - Audit logging settings
   - HITRUST control mapping documentation

6. **Audit Logging:** Structured JSON with HITRUST-required fields
   - Timestamp (ISO 8601)
   - User identity (Tailscale device/user)
   - Source/destination IP and port
   - URL requested
   - Action taken (allow/block)
   - Category matched (if blocked)
   - TLS certificate details
   - Rule/policy reference

7. **Protocol Support:** HTTP/2 with HTTP/1.1 fallback
   - Modern web requires HTTP/2 support
   - Graceful degradation for legacy clients

## HITRUST i1 Control Mapping

This implementation addresses the following HITRUST i1 Category 10 controls:

### Category 10.0 - Information Systems Acquisition, Development, and Maintenance

**Control 10.c - Network Security Controls**
- **10.c.1 - URL Filtering:** Implementation of pattern-based URL filtering to block access to unauthorized or malicious websites
- **10.c.2 - DNS Filtering:** Integration with DNS filtering services to prevent resolution of known malicious domains
- **10.c.3 - TLS Inspection:** Decryption and analysis of TLS-encrypted traffic to inspect content and validate certificates
- **10.c.4 - Content Categorization:** Classification and filtering of web content by category to enforce organizational policies

**Evidence for Assessors:**
- Configuration files showing filter rules and categories
- Audit logs demonstrating blocking of test malicious URLs
- Documentation of threat intelligence feed sources
- Sample TLS certificate inspection logs

## Open Questions

None - all technical and compliance decisions resolved.

## Resolved Questions

### Core Decisions
- **Language:** Rust (chosen for performance and safety)
- **Target Compliance:** HITRUST i1 Category 10 (Network Security)
- **Deployment:** Forward proxy at Tailscale exit node
- **Target Users:** Healthcare organizations and business associates
- **Architecture:** MITM Proxy with TLS inspection
- **Filtering Scope:** URL, DNS, content categorization, TLS certificate analysis
- **Configuration:** YAML files (version-controllable)
- **TLS Certificates:** Organization-provided CA
- **Audit Logging:** Structured JSON with HITRUST-required fields

### Technical Implementation Decisions
- **HTTP/2 Library:** hyper (supports HTTP/1 & 2 automatically)
- **Protocol Support:** HTTP/2 with HTTP/1.1 fallback
- **Block Response:** 403 Forbidden with optional category explanation
- **Logging Format:** Structured JSON (HITRUST assessor-friendly)
- **Concurrency:** Multi-threaded with Tokio async runtime
- **Network Support:** IPv4 + IPv6 (dual-stack)
- **DNS Filtering:** Integration with external threat feeds (configurable)
- **Content Categories:** Predefined categories with custom extension capability

### HITRUST-Specific Decisions
- **Control Category:** 10.0 - Information Systems Acquisition, Development, and Maintenance
- **Specific Controls:** 10.c (Network Security) - URL filtering, DNS filtering, TLS inspection, content categorization
- **Evidence Collection:** Audit logs designed for assessor review
- **Control Mapping:** Explicit documentation linking features to HITRUST requirements

## Next Steps

→ `/workflows:plan` for implementation details and technical planning

Document: docs/brainstorms/2026-02-25-http2-tls-web-filter-hitrust-i1-brainstorm.md

Key decisions:
- HITRUST i1 Category 10 compliance focus (URL filtering, DNS filtering, TLS inspection, content categorization)
- Forward proxy deployment at Tailscale exit node
- Rust implementation with hyper for HTTP/2 support
- Structured JSON audit logging for HITRUST evidence
- YAML configuration for filter rules and threat intelligence feeds

Next: Run `/workflows:plan` when ready to implement.
