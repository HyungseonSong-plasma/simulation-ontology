# Thermal Reference Fixtures

This directory contains the first end-to-end semantic vertical slice defined by the Core Simulation Ontology v0.1 implementation plan.

## Active fixture

- `thermal-reference.json` — solver-independent Thermal reference model used for canonical serialization, validation, identity/resolution, mapping, and related Core contract tests as those implementation stages become available.

## Fixture boundary

The canonical Thermal fixture represents SOL semantics, not a solver-native input model. It SHALL NOT encode MOOSE, COMSOL, Ansys, or other backend-native objects merely to make a mapping convenient.

Core tests may combine this fixture with MockAdapter, BackendTarget, MappingPlan, RealizationEffect, and evaluation-lifecycle fixtures to exercise deterministic contract behavior without installing a real solver. Such tests demonstrate Core/adapter-protocol semantics; they do not constitute solver-native or physical validation.

Real backend realization belongs to independent adapter projects. The reference roadmap starts with the external MOOSE adapter in v0.1, followed by Zapdos/CRANE work in v0.2. Backend-specific expected artifacts or integration evidence should therefore live with the relevant adapter project unless a solver-independent artifact is intentionally retained here as a Core conformance fixture.
