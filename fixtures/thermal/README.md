# Thermal Reference Fixtures

This directory contains the first end-to-end vertical slice defined by the Core Simulation Ontology v0.1 implementation plan.

## Active fixture

- `thermal-reference.json` — Phase 1 solver-independent semantic model used for JSON serialization round-trip testing.

The fixture targets the semantic Core only. Until M0.1 is stable, it must not encode MOOSE, COMSOL, or Ansys native objects and must not depend on a real backend adapter.
