# Cross-Backend Reference Validation Plan v0.1

**Owner:** Validation Lab  
**Status:** Ready for execution  
**Date:** 2026-08-20

## Purpose

Independently validate the SOL v0.1 architecture by projecting the same semantic reference model to MOOSE, COMSOL, and Ansys without modifying Core semantics.

## Architecture under test

Primary decisions:

- ADR-0006 — Semantic preservation across backends
- ADR-0007 — Constraint architecture and composition
- ADR-0008 — Inheritance and Interface composition
- ADR-0009 — Identity, namespace, package, and versioning
- ADR-0010 — Profile and Backend Mapping Contract

Validation evidence SHALL be treated as independent verification. If a projection fails, Validation Lab SHALL first classify the failure before proposing an architecture change.

## Reference model

Start with a small thermal model containing at least:

- Material
- ThermalConductivity
- TemperatureField
- BoundaryCondition
- Study
- Scope/selection
- at least one constant ValueDefinition
- at least one non-constant ValueDefinition if supported by the backend profile

After the thermal model, repeat with a plasma-oriented model containing at least:

- Species
- ElectronDensity / relevant Field
- Transport coefficient
- Reaction
- BoundaryCondition / surface reaction
- Study

## Backend targets

Validate independent projections to:

1. MOOSE
2. COMSOL
3. Ansys

Backend-native structure MUST NOT be copied back into SOL Core solely to make a projection easier.

## Validation pipeline

```text
SOL reference model
  -> semantic validation
  -> Profile mapping-rule evaluation
  -> MappingClaim composition
  -> MappingPlan generation
  -> representability preflight
  -> BackendAdapter realization
  -> backend artifact/model inspection
  -> semantic preservation assessment
```

## Required checks

### V1 — Core independence

The same SOL reference model SHALL be used for all three backends. Backend switching SHALL NOT require changing Core semantic meaning.

### V2 — MappingRule sufficiency

The five-field MappingRule contract SHALL be sufficient:

```text
source
applicability
realization
bindings
capabilities
```

Any missing information SHALL be recorded as evidence before extending the schema.

### V3 — MappingClaim composition

Applicable rules SHALL produce MappingClaims with:

```text
obligation
source
realization
provenance
```

Compatible claims SHALL compose. Same-source + same-obligation + incompatible realization SHALL be reported as a MappingConflict.

### V4 — MappingPlan inspectability

A backend-independent MappingPlan SHALL be generated before backend mutation or artifact creation. Dependency/order information SHALL form an acyclic realization graph or produce an explicit diagnostic.

### V5 — Representability classification

Each realized mapping SHALL be classified as one of:

```text
exact
transformed
lossy
unsupported
```

`transformed` SHALL be treated as successful when semantic meaning is preserved.

### V6 — Constraint preservation

Constraints from Core/domain ontology SHALL remain valid through mapping. Backend limitations SHALL NOT silently weaken semantic constraints.

### V7 — Identity preservation

Schema and SOL model-instance identities SHALL remain stable across backend projections. Only backend-local identity may vary.

### V8 — Backend representability boundary

A valid SOL model that cannot be represented faithfully SHALL remain semantically valid and be reported as lossy/unsupported rather than being rewritten to satisfy the backend.

### V9 — Diagnostic provenance

Any schema, configuration, mapping, capability, or representability failure SHALL identify the contributing rule/profile/adapter source where possible.

## Failure classification

Every failure SHALL be classified before remediation:

```text
Architecture defect
Profile defect
Adapter defect
Backend limitation
Reference-model defect
Validation-tooling defect
```

Only an Architecture defect should trigger reconsideration of accepted SOL ADRs.

## Pass criteria for SOL v0.1 architecture freeze

Architecture freeze is supported when:

1. the same reference model projects to MOOSE, COMSOL, and Ansys without Core semantic changes;
2. exact/transformed/lossy/unsupported outcomes are explainable before realization;
3. mapping conflicts are deterministic and do not rely on declaration priority;
4. backend-local identities do not leak into schema identity;
5. backend limitations are reported rather than silently normalized into Core;
6. no new Core primitive is required by either the thermal or plasma reference validation.

## Feedback path

Validation Lab findings SHOULD be recorded under `docs/validation/`.

If an architecture defect is found, create a focused research note describing the counterexample and return it to Research Lab for ADR review. Implementation/profile/adapter defects SHOULD remain in Validation/Implementation workstreams and SHOULD NOT reopen Core architecture by default.
