# Public Contract 0.1 — Model, Validation, and Diagnostic DTOs

**Status:** M0.2 Phase 2 normative baseline  
**Issue:** #31  
**Date:** 2026-08-21

## Purpose

This document defines the Public Contract 0.1 shapes for model-facing semantic data and validation output. The contract preserves M0.1 semantic meaning without exposing resolver graphs, Rust ownership/storage choices, or backend-specific objects.

The generic JSON/version rules in `public-contract-0.1-json-rules.md` remain governing. In particular, top-level canonical documents require `public_contract_version: "0.1"`, declared extensible objects preserve unknown optional fields, object-key order is not semantic, and arrays preserve order unless a DTO explicitly defines a different collection rule.

## Model-facing DTO

The top-level model document is `SimulationDto`:

```text
SimulationDto
  public_contract_version: string = "0.1"
  ontology_version: string
  model: SimulationModelDto
  tasks: SimulationTaskDto[]
  relations: SemanticRelationDto[]
```

`ontology_version` and `public_contract_version` remain independent version axes.

### Model entity DTOs

`SimulationModelDto` preserves the M0.1 categories:

- `physics`
- `mathematical`
- `constitutive`
- `spatial`
- `scopes`
- `material`
- `conditions`
- `numerical`
- `observations`

`OntologyEntityDto` carries:

```text
id
kind
semantic_type
label
```

`SimulationTaskDto` carries:

```text
id
analyses[]
solver_configurations[]
```

`SemanticRelationDto` carries:

```text
kind
source
 target
```

The serialized key is `target`; the spacing above is explanatory only.

### SpatialScope

`SpatialScopeDto` is first-class and carries:

```text
id
members[]
```

Members are canonical references to spatial-model entities. Backend selections, mesh-set handles, or solver-native object IDs are not canonical scope identity.

## Canonical reference semantics

Public references remain language-neutral strings. A canonical reference contains at least one namespace separator (`.`), and each namespace/local segment contains only ASCII alphanumeric characters, `_`, or `-`.

Examples:

```text
thermal.energy_conservation
scope.main_domain
ontology.thermal.temperature
```

The public contract does not expose `CanonicalId`, `Namespace`, `ResolvedGraph`, `ResolvedNode`, or resolver storage types.

## ValidationReport

A top-level validation result is:

```text
ValidationReport
  public_contract_version: "0.1"
  valid: boolean
  diagnostics: Diagnostic[]
```

`valid` is validation validity, not the evaluation lifecycle status `PASS/FAIL/BLOCKED/INDETERMINATE`.

`valid` is `false` when at least one `error` diagnostic exists. Warning and informational diagnostics do not by themselves make the report invalid.

## Diagnostic

The normative Diagnostic fields are:

```text
code: string
severity: error | warning | info
subject?: canonical reference
detail: string
```

The four fields above are the stable semantic diagnostic surface for Public Contract 0.1. Objects remain extensible according to the Phase 1 JSON rules.

- `code` is machine-stable and dot-separated.
- `severity` is a closed enum in Public Contract 0.1.
- `subject` is optional and, when present, is a canonical semantic reference.
- `detail` is human-readable evidence and must not be used as the machine identity of the condition.

Unknown severity tokens are malformed; they are not mapped to lifecycle states. Diagnostic codes may be extended additively, but the meaning of an existing code must not be silently redefined under Public Contract 0.1.

## Diagnostic ordering and duplicates

`diagnostics` is an explicitly canonicalized collection, which is an exception to the generic Phase 1 rule that array order is preserved.

Canonical ordering is:

1. severity (`error`, then `warning`, then `info`);
2. code;
3. subject;
4. detail;
5. opaque extension payload for deterministic tie-breaking.

Exact duplicate diagnostic objects collapse to one entry. Diagnostics that differ in any normative field or preserved extension remain distinct.

This rule makes validation output deterministic without making discovery/evaluation order part of the public semantics.

## Phase 2 model-integrity diagnostic codes

The initial Public Contract 0.1 model-integrity codes are:

| Code | Meaning |
|---|---|
| `model.invalid_canonical_id` | An ID/reference does not satisfy canonical reference syntax. |
| `model.duplicate_id` | Two canonical nodes use the same canonical ID. |
| `model.empty_scope` | A SpatialScope has no members. |
| `model.duplicate_scope_member` | A SpatialScope repeats a member reference. |
| `model.unresolved_scope_member` | A SpatialScope member does not resolve. |
| `model.non_spatial_scope_member` | A SpatialScope member resolves to a non-spatial node. |
| `model.unresolved_relation_endpoint` | A relation source or target does not resolve. |

These codes preserve the M0.1 identity/scope/reference semantics while decoupling the public report from `ResolveError` and other Rust enums.

## Extensibility and round-trip behavior

The model DTOs, reports, and diagnostics preserve unknown optional object members via opaque extension storage. Unknown fields cannot redefine known 0.1 fields or their normative meaning.

Typed model/report round trips must preserve known semantic fields and unknown optional fields and must emit JSON using the canonical JSON rules from Phase 1.

## Validation scope in Phase 2

The executable `validate_simulation` reference path covers the model-integrity semantics needed for the Phase 2 contract:

- canonical ID syntax;
- duplicate canonical IDs;
- SpatialScope non-empty/duplicate/resolved/spatial-member rules;
- relation endpoint resolution.

This does not make the public-contract crate a replacement for all Core semantic evaluation. QRC-specific validation, mapping, planning, target resolution, evaluation, and adapter semantics remain owned by their existing or later M0.2/M0.3 layers. Later conversion/orchestration may depend on both internal Core and this public contract, but the public contract must not depend on incidental internal Core crate layout.

## Counterexamples

Phase 2 fixtures enforce at least these failures:

```text
unresolved relation reference -> model.unresolved_relation_endpoint
non-spatial scope member       -> model.non_spatial_scope_member
duplicate canonical ID         -> model.duplicate_id
malformed Diagnostic           -> report parse/shape rejection
```

The Thermal reference model is the positive golden model and produces a valid report with no diagnostics.

## Compatibility rule

Under Public Contract 0.1, changing any of the following incompatibly is a breaking semantic change:

- canonical reference meaning;
- the meaning of an existing diagnostic code;
- severity semantics;
- validity derivation;
- diagnostic canonical ordering/duplicate rules;
- SpatialScope membership semantics.

Such a change must not be introduced silently under the same declared contract version.
