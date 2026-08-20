# SOL Implementation Documentation

This directory contains post-design implementation planning and implementation decisions for the closed SOL v0.1 semantic baseline.

## Boundary

- Semantic design authority remains in ADR-0001..ADR-0030.
- Implementation documents MUST NOT silently change the closed semantic baseline.
- A genuine semantic counterexample must follow the ADR-0030 reopen procedure before changing Core semantics.

## Current implementation baseline

- [Core Implementation Plan v0.1](sol-v0.1-core-implementation-plan-v0.1.md)
- [IDR-0001 — Polyglot Core, Protocol Boundary, and CI-First Bootstrap](decisions/0001-polyglot-core-protocol-and-ci-first-bootstrap.md)

## Current state

```text
Design: SOL v0.1 DESIGN-STAGE CLOSED
Implementation: planning baseline accepted
Next atomic unit: Rust workspace + GitHub Actions CI bootstrap
Primary vertical slice: Thermal reference model
Real backend adapters: deferred
TypeScript protocol/client binding: deferred until CLI milestone is stable
```
