# Phase 1 — Data Model / Serialization

**Status:** Implementation submitted; CI evidence pending.

## Implemented scope

- `Simulation` root document.
- `SimulationModel` with the eight Core Ontology v0.1 model domains.
- `SimulationTask` with Analysis and SolverConfiguration.
- Solver-independent `OntologyEntity` representation.
- First-class `SemanticRelation` edges.
- Relation vocabulary: `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `applied_to`, `analyzed_by`, `solved_by`, `produces`, `observed_by`.
- JSON serialization/deserialization using pinned `serde` / `serde_json` dependencies.
- Thermal reference fixture and JSON round-trip contract test.

## Boundary decisions

- Phase 1 entity `id` is a document-level reference key only. Canonical identity and namespace semantics remain Phase 2 work.
- Phase 1 does not enforce relation cardinality, endpoint type compatibility, or reference integrity. Those remain Phase 3 Constraint/QRC responsibilities.
- No MOOSE, COMSOL, Ansys, or real adapter representation is introduced into the semantic Core.

## Exit criterion

Phase 1 is complete only when GitHub Actions passes build, test, fmt, clippy, architecture counterexamples, and the Thermal serialization round-trip test.
