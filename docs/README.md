# Documentation Map

This page is the primary navigation entry point for SOL architecture and implementation documentation. Use relative links so references remain valid across branches and local checkouts.

## Normative / current design

- [Architecture](architecture.md) — high-level layering and architectural boundaries.
- [Ontology Language](ontology-language.md) — current SOL language rules and normative semantics.
- [Architecture Decision Records](decisions/README.md) — accepted design decisions and rationale.
- [ADR-0030](decisions/0030-sol-v0.1-design-stage-closure.md) — authoritative SOL v0.1 design-stage closure.

## Post-design implementation

- [Implementation index](implementation/README.md) — implementation plans and post-design implementation decisions.
- [Core Implementation Plan v0.1](implementation/sol-v0.1-core-implementation-plan-v0.1.md) — Rust semantic Core, CI-first sequence, Mock Adapter, CLI and deferred TypeScript/backend gates.
- [IDR-0001](implementation/decisions/0001-polyglot-core-protocol-and-ci-first-bootstrap.md) — polyglot architecture and protocol-boundary decision.

Implementation documents consume the closed semantic baseline. They do not silently redefine it.

## Evidence / research

- [Research index](research/README.md) — cross-backend studies and evidence used to reach decisions.
- [Validation artifacts](validation/) — independent design-stage and reference-model validation evidence.

## Traceability rule

Documentation should follow this direction:

```text
Research evidence
      ↓ supports
ADR / semantic design decision
      ↓ governs
Ontology Language / Architecture
      ↓ closed by ADR-0030
Implementation plan / IDR
      ↓ realized by
Rust Core / protocol / clients / backend adapters
```

If implementation exposes a potential semantic contradiction, classify it first and use the ADR-0030 reopen procedure rather than changing normative semantics from implementation code or documentation.

## Current semantic chains

### Parameter and value semantics

[Cross-backend semantic mapping](research/cross-backend-semantic-mapping-matrix-v0.1.md)
→ [ADR 0001](decisions/0001-parameter-semantics-and-backend-metadata-mapping.md)
→ [ADR 0002](decisions/0002-value-and-value-definition-semantics.md)
→ [ADR 0003](decisions/0003-reference-and-value-dependency-semantics.md)
→ [Ontology Language](ontology-language.md)

### Unit and dimension semantics

[Dimensionless/unit study](research/dimensionless-unit-dimension-study-v0.1.md)
→ [ADR 0004](decisions/0004-unit-and-physical-dimension-semantics.md)
→ [ADR 0005](decisions/0005-dimension-contract-and-metrology-boundary.md)
→ [Unit stress test](research/unit-reference-stress-test-v0.1.md)
→ [UnitReference / metrology contract](research/unit-reference-and-metrology-adapter-contract-v0.1.md)

### Entity identity and backend preservation

[Value identity study](research/value-and-value-definition-identity-study-v0.1.md)
→ [Entity boundary / orthogonal axes study](research/entity-boundary-and-orthogonal-axes-study-v0.1.md)

[Cross-backend semantic mapping](research/cross-backend-semantic-mapping-matrix-v0.1.md)
→ [ADR 0006](decisions/0006-semantic-preservation-across-backends.md)

## Maintenance

When adding a new research, ADR, validation, or implementation document:

1. add it to the appropriate directory index;
2. link its direct predecessors and successors;
3. update this map when it changes a major semantic or implementation chain;
4. prefer relative repository links over copied external GitHub URLs.
