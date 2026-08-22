# Public Contract 0.2 — Canonical RealizationSpec

**Status:** M0.8 realization-contract baseline  
**Issues:** #110, #124, #126  
**Architecture:** `docs/adr/ADR-004-realization-spec-boundary.md`  
**Date:** 2026-08-22

## Purpose

Public Contract 0.2 introduces the canonical solver-independent realization payload required by real external adapters. It does not replace the complete Public Contract 0.1 model/validation/evaluation surface. Instead, 0.2 initially publishes the realization-boundary documents needed by Adapter Protocol 0.2:

- `MappingPlanDtoV02`;
- `BackendTargetDtoV02`;
- `RealizationSpecDtoV02`.

Public Contract 0.1 remains a published and behaviorally unchanged contract. A 0.1 parser does not silently accept a 0.2 document.

## 1. Separation of responsibilities

```text
MappingPlan
  -> action identity
  -> dependency DAG
  -> deterministic planning representation

RealizationSpec
  -> explicit solver-independent realization intent
  -> semantic entities/types
  -> scalar values + canonical units
  -> spatial scopes
  -> semantic relations
  -> action-to-subject/scope bindings
```

Action-ID spelling has no physics or realization meaning. An adapter MUST NOT interpret `thermal.material`, `thermal.solve`, or any other action ID as a hidden mapping instruction.

Opaque PlanAction extensions likewise do not acquire canonical realization meaning. Known attempts to place fields such as `physics`, `parameters`, `quantity`, `unit`, `material`, `boundary_condition`, `equation`, `field`, `analysis`, `observation`, `subjects`, `scopes`, or `realization_spec` into PlanAction extensions are rejected by the 0.2 realization path.

## 2. Version boundary

Top-level 0.2 realization documents declare:

```json
{
  "public_contract_version": "0.2"
}
```

The 0.2 path is explicit. It does not reinterpret a 0.1 document and it does not change the meaning of any Public Contract 0.1 DTO.

The initial 0.2 realization subset deliberately reuses the existing nested PlanAction shape (`id`, `dependencies[]`) while moving realization meaning into a separate top-level document.

## 3. MappingPlan 0.2

```text
MappingPlanDtoV02
  public_contract_version: "0.2"
  actions: PlanActionDto[]
```

Normative semantics are preserved from 0.1:

- action IDs are stable symbols and unique;
- dependencies resolve within the same plan;
- dependency arrays are set-like, sorted, and duplicate-free after normalization;
- cycles are rejected;
- actions are emitted in deterministic lexical action-ID order;
- deterministic topological ordering is planning/reproducibility information, not a mandatory backend physical schedule.

No action-ID string token carries realization semantics.

## 4. BackendTarget 0.2

```text
BackendTargetDtoV02
  public_contract_version: "0.2"
  target: stable symbol
  required_capabilities: stable symbol[]
```

The target remains logical solver/backend intent, not adapter process identity or backend-native object identity. Required capabilities are sorted and duplicate-free after normalization.

## 5. RealizationSpec 0.2

```text
RealizationSpecDtoV02
  public_contract_version: "0.2"
  ontology_version: string
  source_model: canonical reference
  entities: RealizationEntityDtoV02[]
  scopes: SpatialScopeDto[]
  relations: SemanticRelationDto[]
  action_bindings: ActionRealizationBindingDtoV02[]
```

`source_model` identifies the canonical model from which the realization projection was produced. It is semantic provenance for the projection, not a backend artifact handle.

### 5.1 Realization entity

```text
RealizationEntityDtoV02
  id: canonical reference
  kind: EntityKindDto
  semantic_type: stable symbol
  parameters: RealizationParameterDtoV02[]
```

The entity preserves canonical identity and enough semantic classification for an adapter to map the entity into its backend-local typed IR. Backend-native object names/types are not part of this DTO.

### 5.2 Scalar quantity and parameter

```text
RealizationParameterDtoV02
  semantic_parameter: canonical reference
  quantity:
    value: JSON number
    unit: canonical reference
```

Examples of canonical unit references include:

```text
unit.kelvin
unit.meter
unit.watt_per_meter_kelvin
```

The 0.2 realization baseline supports scalar JSON-number quantities. Vector/tensor/function/table values and a complete dimensional conversion algebra are deferred.

Parameter arrays are canonical ordered sets keyed by `semantic_parameter`; duplicate parameter identities on one realization entity are rejected.

### 5.3 Spatial scopes

A projected `SpatialScopeDto` preserves the accepted first-class scope semantics:

```text
SpatialModel != SpatialScope != backend-native selection
```

Scope IDs are canonical references. Scope members resolve to realization entities whose kind is `spatial_model`. Empty scopes and non-spatial/unresolved members are invalid.

A backend may realize a canonical scope through native blocks, boundaries, selections, mesh sets, or other structures, but those backend-native identities do not become canonical SOL identity.

### 5.4 Semantic relations

The realization projection carries canonical `SemanticRelationDto` values. Relation endpoints may resolve to:

- a projected realization entity;
- a projected SpatialScope;
- the `source_model` reference.

This permits canonical relations such as `defined_on` and spatial `applied_to` to target SpatialScopes while preserving model-level relations such as `analyzed_by` where required by a realization projection.

Exact duplicate relations collapse after deterministic normalization. Unresolved endpoints are invalid.

### 5.5 Action realization binding

```text
ActionRealizationBindingDtoV02
  action_id: stable symbol
  subjects: MappingSubjectDto[]
  scopes: canonical reference[]
```

For a RealizationSpec used with a MappingPlan:

- every PlanAction has exactly one action binding;
- no binding names an unknown action;
- an action binding contains at least one subject or scope;
- entity subjects resolve to a known realization reference;
- relation subjects resolve to an explicitly carried semantic relation;
- scopes resolve to projected SpatialScopes.

This binding is the explicit bridge between orchestration structure and realization meaning. The action ID itself remains non-semantic.

## 6. Referential identity space

Within one RealizationSpec, the canonical reference space used by semantic relations and bound entity subjects is:

```text
realization entity IDs
UNION SpatialScope IDs
UNION {source_model}
```

These identities must not collide. Scope membership is stricter: members must resolve specifically to `spatial_model` realization entities.

## 7. Canonical normalization

The 0.2 realization boundary normalizes:

- MappingPlan actions by lexical action ID;
- action dependencies as sorted duplicate-free sets;
- BackendTarget capabilities as sorted duplicate-free sets;
- realization entities by canonical ID;
- parameters by `semantic_parameter`;
- SpatialScopes by canonical ID and members as sorted duplicate-free sets;
- relations by `(source, relation_kind, target)` plus deterministic extension tie-breaking;
- action bindings by action ID;
- binding subjects/scopes as deterministic duplicate-free sets.

Object-key order is non-semantic and canonical JSON emission sorts object keys recursively.

## 8. Backend-native leakage boundary

Canonical realization payloads reject backend-native semantic object/identity fields including known forms such as:

```text
adapter_native
backend_native
backend_native_id
backend_object
native_object
solver_object
mesh_selection_handle
```

The rejection is recursive through preserved extension values. Backend-local typed IR remains inside the adapter repository/runtime.

## 9. Distinguishability requirement

Two canonical models may share the same MappingPlan:

```text
Plan(A) == Plan(B)
```

while differing in realization values or semantic choices. Public Contract 0.2 requires those differences to remain observable through the realization boundary:

```text
RealizationSpec(A) != RealizationSpec(B)
```

The M0.8 thermal fixtures demonstrate this using unchanged action IDs/dependencies but different thermal conductivity and boundary-temperature quantities.

## 10. Schema boundary

The checked-in Draft 2020-12 realization schemas are:

```text
schemas/public-contract/0.2/shared.schema.json
schemas/public-contract/0.2/mapping-plan.schema.json
schemas/public-contract/0.2/backend-target.schema.json
schemas/public-contract/0.2/realization-spec.schema.json
```

JSON Schema establishes structural validity only. Referential integrity, DAG validity, hidden-realization-extension rejection, semantic relation resolution, and exact plan/spec binding coverage remain semantic validation obligations implemented and tested by the contract facade/conformance tooling.

## 11. Compatibility rule

The following are compatibility-sensitive Public Contract 0.2 semantics:

- separation of MappingPlan from RealizationSpec;
- action-ID non-semantic rule;
- explicit scalar quantity + canonical unit representation;
- first-class SpatialScope distinction;
- action-binding exact coverage against a plan;
- canonical reference resolution rules;
- backend-native leakage prohibition;
- deterministic normalization rules.

An incompatible change to these meanings requires a later explicit Public Contract version boundary. It must not be introduced by changing 0.2 behavior in place.
