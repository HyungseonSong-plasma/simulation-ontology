# Public Contract 0.1 — Mapping, Plan, Target, and Evaluation DTOs

**Status:** M0.2 Phase 3 normative baseline  
**Issue:** #32  
**Date:** 2026-08-21

## Purpose

This document promotes the M0.1 mapping, planning, target-selection, realization-effect, and evaluation semantics into language-neutral Public Contract 0.1 DTOs. It does not define Adapter Protocol operations or transport framing.

The generic version/envelope rules in `public-contract-0.1-json-rules.md` remain governing. Nested DTOs inherit the top-level `public_contract_version` and do not repeat it.

## 1. Mapping subject

A mapping subject is either a canonical entity reference or a canonical semantic relation.

Entity form:

```json
{
  "subject_kind": "entity",
  "id": "thermal.energy_conservation"
}
```

Relation form:

```json
{
  "subject_kind": "relation",
  "source": "thermal.transport",
  "relation_kind": "represented_by",
  "target": "thermal.energy_conservation"
}
```

Unknown optional extension fields are preserved but do not change semantic subject identity. Entity identity is the canonical `id`; relation identity is `(source, relation_kind, target)`.

## 2. Mapping claims, evidence, and provenance

The top-level mapping-claim document is:

```text
MappingClaimsDto
  public_contract_version: "0.1"
  claims: MappingClaimDto[]
```

Each claim carries:

```text
rule_id
subject
evidence[]
provenance?
```

Evidence carries stable `source` and human-readable `detail`. Provenance carries `producer` and optional `revision`.

Claims are canonicalized deterministically by rule ID and semantic subject, with exact duplicates removed. Evidence is likewise canonicalized deterministically by source/detail/extension payload with exact duplicates removed. These arrays are explicit set-like exceptions to the generic Phase 1 array-order rule.

An opaque provenance extension such as:

```json
{
  "backend_artifact_ref": "backend:job/123"
}
```

may be preserved as evidence/provenance metadata. Such a reference is not canonical semantic identity and must not participate in semantic subject comparison.

## 3. MappingPlan and PlanAction

The top-level plan document is:

```text
MappingPlanDto
  public_contract_version: "0.1"
  actions: PlanActionDto[]
```

A PlanAction carries:

```text
id
dependencies[]
```

Dependencies name actions that must precede the action. Public Contract 0.1 preserves the M0.1 DAG semantics:

- action IDs are unique;
- every dependency resolves to an action in the same plan;
- cycles are rejected;
- actions are emitted in deterministic lexical action-ID order;
- dependency arrays are set-like, lexically sorted, and duplicate-free after normalization;
- deterministic topological ordering is available for canonical planning behavior.

The deterministic canonical order is a representation/planning rule, not a future Adapter Protocol execution-scheduling mandate. A later adapter may schedule independent actions differently only if dependency semantics remain preserved.

## 4. BackendTarget

The top-level target requirement document is:

```text
BackendTargetDto
  public_contract_version: "0.1"
  target: string
  required_capabilities: string[]
```

`target` is a logical solver/backend target class such as `mock`; it is not an adapter process ID, solver object handle, mesh selection ID, pointer, or other backend-native identity.

`required_capabilities` is a set-like collection: it is lexically sorted and duplicate-free after normalization.

Adapter implementation identity, protocol version negotiation, dynamic adapter description, and transport connection state are M0.3+ concerns and do not belong in this M0.2 payload.

## 5. Backend-native leakage boundary

Canonical semantic DTOs do not expose backend-native objects or native semantic identities. Reserved extension fields that explicitly attempt to place objects/identities into canonical semantic payloads are rejected, including:

```text
adapter_native
backend_native
backend_native_id
backend_object
native_object
solver_object
```

This rule does not prohibit opaque artifact references used only as provenance/evidence metadata. Opaque references do not redefine canonical identity, equality, target selection, or evaluation semantics.

## 6. RealizationEffect

`RealizationEffectDto` carries:

```text
subject
quality: exact | compatible | degraded | unsupported | unknown
detail?
```

The quality enum is closed in Public Contract 0.1 and preserves the M0.1 meanings. A top-level effect document wraps this nested shape with `public_contract_version`.

Realization effects report how a realization relates to the intended semantic subject. They do not redefine the original intent.

## 7. EvaluationResult

The top-level evaluation result is:

```text
EvaluationResultDto
  public_contract_version: "0.1"
  intended_subject
  effect
  comparison
  status
```

Comparison is one of:

```text
exact
compatible
degraded
unsupported
unknown
subject_mismatch
```

Lifecycle status is one of:

```text
pass
fail
blocked
indeterminate
```

The normative mapping is unchanged from M0.1:

| Comparison | Evaluation status |
|---|---|
| `exact` | `pass` |
| `compatible` | `pass` |
| `degraded` | `fail` |
| `subject_mismatch` | `fail` |
| `unsupported` | `blocked` |
| `unknown` | `indeterminate` |

A parsed EvaluationResult must be internally consistent. In particular:

- a different effect subject yields `subject_mismatch`, regardless of effect quality;
- `unsupported` must not be declared `fail`;
- `unknown` must not be declared `fail`;
- subject mismatch must not be declared `exact/pass`;
- unknown optional subject extensions do not change subject identity.

## 8. Deterministic normalization

Phase 3 declares the following DTO-specific collection semantics:

- `MappingClaimsDto.claims`: canonical ordered set with exact-duplicate collapse;
- `MappingClaimDto.evidence`: canonical ordered set with exact-duplicate collapse;
- `MappingPlanDto.actions`: unique-by-ID, lexically ordered;
- `PlanActionDto.dependencies`: set-like, lexically ordered, duplicate-free;
- `BackendTargetDto.required_capabilities`: set-like, lexically ordered, duplicate-free.

Other arrays continue to follow the generic Phase 1 rule unless a later adopted DTO says otherwise.

Unknown optional fields are preserved opaquely but do not become semantic identity merely because they round-trip.

## 9. Executable fixtures

Positive Thermal goldens:

- `fixtures/public-contract/0.1/thermal-mapping-claims.json`
- `fixtures/public-contract/0.1/thermal-mapping-plan.json`
- `fixtures/public-contract/0.1/thermal-backend-target.json`
- `fixtures/public-contract/0.1/thermal-evaluation.json`

Counterexamples:

- plan cycle rejection;
- unsupported realization -> `blocked`, not `fail`;
- unknown realization -> `indeterminate`, not `fail`;
- subject mismatch -> `fail`;
- mismatched subject falsely declared `exact/pass` -> rejected;
- backend-native object leakage -> rejected.

## 10. Non-goals

Phase 3 does not define:

- Adapter Protocol methods such as description, validation, or execution;
- protocol negotiation;
- JSON-RPC request IDs or error codes;
- stdio/process framing;
- adapter execution scheduling;
- retry semantics;
- backend-native realization object schemas.

Those concerns belong to M0.3 and later milestones.

## 11. Compatibility rule

Under Public Contract 0.1, silently changing any of the following is a semantic breaking change:

- mapping subject identity;
- claim/evidence canonical ordering semantics;
- MappingPlan dependency/cycle semantics;
- BackendTarget requirement meaning;
- RealizationQuality meaning;
- comparison-to-lifecycle mapping;
- the separation of `unsupported`/`unknown` from `fail`;
- the rule that opaque extensions do not become semantic identity.
