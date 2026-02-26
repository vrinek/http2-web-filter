---
review_agents: [code-simplicity-reviewer, security-sentinel, performance-oracle, architecture-strategist, pattern-recognition-specialist]
plan_review_agents: [code-simplicity-reviewer]
---

# Review Context

**Project Type:** HTTP/2 Web Filter with TLS Inspection in Rust

**Security Focus:** This is a security-critical project for HITRUST i1 Category 10 compliance. Extra scrutiny required on:
- TLS/SSL handling and certificate management
- Input validation and sanitization
- PII handling and redaction
- Certificate authority operations
- Network security boundaries

**Architecture:** Modular design with 7 components (ca, tls, proxy, filtering, audit, security, config). Each module should have clear interfaces and be testable in isolation.

**Testing:** TDD is mandatory. 80% unit tests (using mockall for mocks), 20% integration tests. Look for:
- Missing test coverage
- Mock abuse (too many mocks = design smell)
- Test organization and clarity

**Performance:** Network proxy serving multiple concurrent connections. Check for:
- Resource leaks (connections, file handles)
- Memory allocations in hot paths
- Concurrency issues (deadlocks, race conditions)
- Async/await usage patterns

**Code Quality:** Follow Rust idioms:
- Proper error handling (Result, ? operator, thiserror)
- Ownership and borrowing clarity
- Trait-based design for testability
- Avoid unsafe code unless absolutely necessary

**Documentation:** Each public API should have doc comments. Complex logic needs inline comments explaining the "why" not the "what".

**Dependencies:** Review dependency choices carefully - each dependency adds to the attack surface and compile time. Prefer small, focused crates.
