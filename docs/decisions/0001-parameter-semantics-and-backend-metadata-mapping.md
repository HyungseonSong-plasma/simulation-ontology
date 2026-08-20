# ADR-0001: Parameter Semantics and Backend Metadata Mapping

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Simulation Ontology Language v0.1

## Context

Simulation frameworks expose generic parameter/property metadata, but the native metadata category does not uniquely determine semantic meaning.

Cross-framework review of MOOSE, COMSOL, and Ansys shows the same general pattern:

- MOOSE `InputParameters` may represent scalar coefficients, field references, block/boundary scope references, enums, solver/execution settings, and other object inputs.
- COMSOL parameters and feature properties may represent physical values, expressions, material/constitutive values, configuration choices, geometry/mesh/solver settings, while selections provide spatial scoping. A global model parameter may also be reused in different modeling or study contexts.
- Ansys object properties and analysis settings may represent quantities, locations/scopes, configuration choices, convergence controls, time/step-dependent settings, and other solver or model inputs.

Therefore, backend-native terms such as `parameter` and `property` are implementation metadata categories and cannot be copied directly into the Core Simulation Ontology as if they were universal semantic types.

## Decision

SOL v0.1 adopts the following normative principles.

### P1 — Native Parameter Non-Equivalence

A backend-native parameter/property classification does not by itself determine its SOL semantic type.

A MOOSE `InputParameter`, COMSOL parameter/feature property, or Ansys object property MUST be semantically interpreted before being mapped into the core model.

### P2 — Contextual Parameterization

Parameterization is modeled as a contextual assignment or reference binding between semantic concepts, typed values or referenced entities, and the model context in which the binding applies.

The same semantic quantity may play different parameterization roles in different contexts. For example, a frequency quantity may be used as a model input in one context and as a parametric-study variable in another.

### P3 — Semantic Mapping Requirement

Backend metadata MUST undergo semantic classification before representation in the Core Simulation Ontology.

The conceptual mapping pipeline is:

```text
Backend-native metadata
        │
        ▼
Metadata extraction
        │
        ▼
Semantic classification
        │
        ▼
SOL semantic representation
        │
        ▼
Profile / backend binding
```

Direct mappings such as the following are invalid unless semantic equivalence has independently been established:

```text
MOOSE InputParameter  ≠ automatically SOL Property
COMSOL Parameter      ≠ automatically SOL Parameter
COMSOL FeatureProperty ≠ automatically SOL Property
Ansys ObjectProperty  ≠ automatically SOL Property
```

## Semantic classification examples

```text
Native input/property       Possible SOL interpretation
────────────────────────────────────────────────────────
scalar physical value       ValueAssignment / Quantity
field or variable name      FieldReference
block/boundary/selection    ScopeReference
material/property reference MaterialReference / MaterialPropertyReference
mode/enum choice            ConfigurationChoice
solver tolerance            Solver/Convergence configuration
execution setting           Execution/Numerical configuration
```

The classification depends on semantic context rather than the backend metadata container alone.

## ParameterBinding status

This ADR establishes the need for a contextual binding concept, but intentionally does **not** decide whether `ParameterBinding` is:

1. a Core Entity,
2. a Relation or specialized relation,
3. a dedicated language-level `Binding` construct, or
4. a lightweight relation that is reified only when contextual metadata is required.

That representation decision requires separate evaluation against the SOL Entity/Property/Relation boundary rules and reference models.

## Consequences

### Positive

- Core semantics remain independent of MOOSE, COMSOL, Ansys, and future backend terminology.
- Backend metadata extractors can preserve native information without forcing native categories into the core taxonomy.
- The same semantic concept can be mapped consistently across different simulation packages.
- Context-dependent uses such as sweeps, scoped values, and solver settings can be represented without redefining the underlying physical quantity.

### Cost

- Backend integration requires a semantic-classification layer rather than a direct metadata copy.
- Some mappings cannot be inferred from type information alone and may require documentation, rules, tags, or curated mappings.
- The language needs a precise representation for contextual bindings.

## Validation basis

This decision was reached after comparing metadata concepts in three reference backend families: MOOSE, COMSOL, and Ansys. The comparison is architectural evidence for the principles above; detailed backend mappings remain the responsibility of their respective backend ontologies and profiles.

## Follow-up decision

Determine the normative representation of contextual parameterization: `ParameterBinding` as Core Entity versus Relation/reification pattern versus a dedicated language-level `Binding` construct.
