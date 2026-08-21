# Implementation Notes

Implementation-stage notes live here. They record how accepted architecture and product contracts are realized in code. They complement, but do not replace, the planning/policy documents in `docs/plans/`, architecture decisions, or executable evidence in CI.

## Baseline documents

Before changing an implementation boundary, consult the relevant plan/policy:

- [`../plans/core-simulation-ontology-v0.1-implementation-plan.md`](../plans/core-simulation-ontology-v0.1-implementation-plan.md) — Core v0.1 implementation sequence and validation gates.
- [`../plans/product-boundary-and-adapter-roadmap.md`](../plans/product-boundary-and-adapter-roadmap.md) — Core vs real-adapter ownership, MockAdapter role, protocol transport, and reference-adapter roadmap.
- [`../plans/versioning-and-compatibility-policy.md`](../plans/versioning-and-compatibility-policy.md) — Runtime/Public Contract/Adapter Protocol/ontology version axes and breaking-change rules.

## Current notes

- [`phase1-data-model.md`](phase1-data-model.md) — Core Ontology v0.1 data model and serialization boundary.

## Implementation boundary

Implementation in this repository should preserve the language-neutral public-contract boundary. Internal Rust crate organization is not itself the public API contract. The intended external surface evolves through a Rust facade/CLI first, followed by TypeScript and Python SDKs as they are promoted during v0.x.

Real MOOSE, Zapdos, CRANE, COMSOL, Ansys, or other solver adapters do not belong in this repository. Adapter-facing implementation work here is limited to the versioned language-neutral protocol, MockAdapter reference conformance behavior, conformance tooling, and supporting public schemas/contracts.

Implementation evidence should be executable where practical: unit tests, golden/counterexample fixtures, SDK parity tests, protocol conformance tests, and CI results take precedence over prose claims of completion.
