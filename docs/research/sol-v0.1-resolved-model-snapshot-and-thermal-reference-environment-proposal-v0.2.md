# SOL v0.1 Resolved Model Snapshot + Thermal Reference Environment Proposal v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision input:** RSV-01  
**Base:** `docs/research/sol-v0.1-resolved-model-snapshot-and-thermal-reference-environment-proposal-v0.1.md`

## 1. Preserved decisions

Unchanged from v0.1:

- `snapshot_state = closed`;
- exact ontology package environment;
- opaque unique model-instance IDs within the snapshot;
- one canonical EntityType reference per focused Entity instance;
- canonical relation triples with source/target model-instance IDs;
- duplicate identical relation triples are invalid in normalized input;
- endpoint/subtype/allowed-pair validation against ADR-0028 package resources;
- ordinary cardinality and later QRC evaluate against the closed graph;
- two small normalized reference packages rather than a production compiler;
- backend runtime/install/license state is excluded.

Only Entity-local value-bearing serialization changes.

## 2. RSV-01 — explicit property assignments

The focused Entity instance is revised to:

```yaml
id: "model:left-temperature"
type: <canonical EntityTypeDefinition id>
properties:
  - property: <canonical PropertyDefinition id>
    value_definition:
      mechanism: literal
      value: <ADR-0026 normalized Value>
```

### 2.1 EntityInstance normalized shape

```text
EntityInstance = {
  id,
  type,
  properties[]
}
```

`properties` is required in the normalized focused schema and may be empty. This distinguishes deliberate no-property content from an omitted/incomplete normalized field.

### 2.2 PropertyAssignment normalized shape

```text
PropertyAssignment = {
  property,
  value_definition
}
```

Rules:

1. `property` is a canonical PropertyDefinition ID;
2. `property` must resolve exactly once in the snapshot's exact ontology environment;
3. `value_definition` is a normalized InlineValueDefinition under ADR-0026/0027;
4. the assignment itself has no independent model-instance identity;
5. order of property assignments is semantically irrelevant;
6. one Entity instance may not contain two assignments with the same canonical PropertyDefinition ID;
7. validators SHALL NOT infer PropertyDefinition identity from display labels, backend parameter names, or declaration order;
8. this focused schema does not invent general property applicability/domain/lifecycle/provenance semantics.

Unknown PropertyDefinition identity fails before value evaluation:

```text
MODEL_PROPERTY_DEFINITION_UNRESOLVED
```

Duplicate normalized assignment fails:

```text
MODEL_PROPERTY_ASSIGNMENT_DUPLICATE
```

## 3. Thermal extension PropertyDefinitions

The normalized Thermal reference package SHALL define/export two minimal PropertyDefinitions:

```text
thermal-ref:thermal_conductivity_value
thermal-ref:prescribed_temperature
```

They carry durable canonical PropertyDefinition IDs under ADR-0028.

The package's authoring/export names communicate the semantic role used by the reference fixture; canonical identity remains authoritative. No backend-native names such as `k`, `value`, `Magnitude`, or `T0` become SOL PropertyDefinition identity.

## 4. Thermal property assignments

### Thermal conductivity

```yaml
entity: ThermalConductivity_1
property: thermal-ref:thermal_conductivity_value
value_definition:
  mechanism: literal
  value:
    shape: scalar
    scalar_kind: number
    data: {coefficient: "1", exponent10: 1}
    unit: {namespace: "si-ref", id: "W_per_m_K"}
```

### Left fixed temperature

```yaml
entity: LeftFixedTemperature
property: thermal-ref:prescribed_temperature
value_definition:
  mechanism: literal
  value:
    shape: scalar
    scalar_kind: number
    data: {coefficient: "3", exponent10: 2}
    unit: {namespace: "si-ref", id: "K"}
```

### Right fixed temperature

Same PropertyDefinition identity, value `400 K`.

This preserves Property/ValueDefinition/Value separation while keeping the fixture small.

## 5. Revised focused snapshot contract

```yaml
snapshot_state: closed
ontology_environment:
  - package: "@simulation-ontology/core-reference-fixture"
    version: "0.1.0"
  - package: "@simulation-ontology/thermal-reference"
    version: "0.1.0"
entities:
  - id: <opaque model-instance id>
    type: <canonical EntityTypeDefinition id>
    properties:
      - property: <canonical PropertyDefinition id>
        value_definition: <InlineValueDefinition>
relations:
  - relation: <canonical RelationDefinition id>
    source: <model-instance id>
    target: <model-instance id>
```

No arbitrary additional fields are permitted in the normalized focused schema.

## 6. Semantic validation order delta

The v0.1 sequence is refined to:

```text
1. validate exact ontology environment
2. build canonical package resource index
3. validate closed snapshot structure
4. resolve unique model Entity identities/types
5. resolve each PropertyDefinition canonical ID
6. reject duplicate property assignments per Entity
7. validate each InlineValueDefinition structurally/semantically
8. resolve relation triples
9. validate endpoint/subtype/allowed-pair semantics
10. evaluate cardinality/QRC
11. evaluate focused metrology evidence when supplied
```

Property resolution occurs before Value evaluation so a valid value cannot rescue an unknown semantic property.

## 7. Boundary cases added for RSV-01

1. Entity with empty `properties: []` -> valid;
2. property assignment uses unknown canonical PropertyDefinition -> FAIL;
3. two assignments of same PropertyDefinition on one Entity -> FAIL;
4. same PropertyDefinition assigned once to two different Entity instances -> valid;
5. backend parameter token `value` used instead of canonical PropertyDefinition ID -> FAIL;
6. thermal conductivity and prescribed temperature use different PropertyDefinition IDs even though both values are numeric;
7. property assignment order permutation -> same semantic result;
8. property assignment has extra lifecycle/provenance fields -> structural FAIL in focused v0.1 snapshot.

## 8. Finding closure

**RSV-01 proposed closed.**

The model snapshot now uses existing SOL `Property` semantics instead of an unlabeled Entity value field.

## 9. Research verdict

**Ready for final focused Validation.**
