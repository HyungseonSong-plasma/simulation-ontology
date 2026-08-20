# SOL v0.1 Resolved Model Snapshot + Reference Environment — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-resolved-model-snapshot-and-thermal-reference-environment-proposal-v0.1.md`  
**Scope:** THV-01 / THV-02 remediation only

## 1. Overall verdict

**Revise.**

The proposal correctly closes most of THV-01/THV-02 without reopening backend or Core architecture. The closed snapshot, exact package environment, opaque model-instance identity, canonical relation triples, subtype-aware endpoint validation, duplicate-edge rejection, and minimal normalized reference packages are all consistent with accepted ADRs.

One new serialization/semantic gap remains: RSV-01.

## 2. Accepted parts

### 2.1 Closed resolved snapshot

Accepted.

Requiring `snapshot_state: closed` is an appropriate focused contract for deterministic reference validation and provides the closure evidence needed by later QRC evaluation.

### 2.2 Model-instance identity

Accepted.

A nonempty opaque ID unique within one snapshot is compatible with ADR-0009, which separates model-instance identity from schema and backend identity while deferring the final global URI format.

### 2.3 Relation triples

Accepted.

The focused triple:

```text
(relation canonical ID, source model-instance ID, target model-instance ID)
```

preserves the existing lightweight relation-instance model. Exact triple uniqueness is a valid normalized-snapshot cleanliness rule and avoids syntactic duplicates entering cardinality evaluation.

### 2.4 Exact ontology environment

Accepted.

Exact package identities and equality with the supplied resolved package set prevent undeclared packages/import order from silently changing type or relation resolution.

### 2.5 Minimal normalized reference packages

Accepted in principle.

A design-validation package may provide a limited current export/resource set for namespace `sol` and later be replaced by another distribution provider in another environment while durable canonical resource IDs remain unchanged. Package identity is not semantic identity under ADR-0009/0028.

The fixture must preserve the complete accepted semantics of every Core resource it actually includes; it may not narrow `includes_component` or another relation merely because the reference model uses fewer cases.

## 3. RSV-01 — direct Entity `value_definition` bypasses Property semantics

The proposed focused Entity instance shape is:

```text
{
  id,
  type,
  value_definition?
}
```

This is insufficiently explicit for independent semantic interpretation.

SOL already distinguishes:

- Entity — independently identified semantic concept;
- Property — intrinsic characteristic/value-bearing attribute;
- ValueDefinition — how a Value is obtained.

If a generic model Entity may carry an unlabeled `value_definition`, the same structural field could mean very different things:

```text
ThermalConductivity_1.value_definition = 10 W/(m K)
LeftFixedTemperature.value_definition  = 300 K
```

An independent validator can see the values but cannot identify the intrinsic semantic role being assigned except by heuristics over Entity Type names.

That would bypass the accepted Property boundary and would make later extension ambiguous when one Entity has more than one intrinsic value-bearing characteristic.

**Classification:** language/schema contract gap in model-instance serialization; not an Architecture defect.

## 4. Minimum remediation for RSV-01

Do not introduce a general property-object runtime or reified PropertyAssignment Entity.

Use a lightweight normalized property assignment inside the model Entity instance:

```yaml
entities:
  - id: "model:left-temperature"
    type: <FixedTemperatureCondition EntityType ID>
    properties:
      - property: <prescribed-temperature PropertyDefinition ID>
        value_definition:
          mechanism: literal
          value: <normalized Value>
```

Focused rules:

1. `property` is a canonical PropertyDefinition ID resolved from the exact ontology environment;
2. `value_definition` is an ADR-0026/0027 InlineValueDefinition;
3. one normalized Entity instance SHALL NOT contain two assignments for the same PropertyDefinition ID;
4. property assignment order is irrelevant;
5. no independent property-assignment identity is created;
6. PropertyDefinition applicability/domain constraints are not invented by this focused schema; a domain/Interface package may constrain them where already specified;
7. unknown/unresolved PropertyDefinition ID fails before value semantics;
8. backend-local parameter names are not PropertyDefinition identity.

The Thermal extension package can then define stable properties such as:

```text
thermal-ref:thermal_conductivity_value
thermal-ref:prescribed_temperature
```

and the model fixture becomes semantically explicit without adding new Core Entity types.

## 5. THV finding status

| Finding | Status after review |
|---|---|
| THV-01 model snapshot contract | Mostly resolved; revise for RSV-01 |
| THV-02 committed canonical-ID reference environment | Accepted approach |

## 6. No backend finding

No MOOSE, COMSOL, or Ansys installation/license/runtime requirement is introduced by RSV-01. The thermal official-document representability verdict remains accepted.

## 7. Next state

Return only RSV-01 to Research.

Expected delta:

```text
EntityInstance
  id
  type
  properties[]

PropertyAssignment
  property: canonical PropertyDefinition ID
  value_definition: InlineValueDefinition
```

If this delta is deterministic and does not expand into general property-assignment lifecycle/reification semantics, the proposal should be suitable for focused final Validation.
