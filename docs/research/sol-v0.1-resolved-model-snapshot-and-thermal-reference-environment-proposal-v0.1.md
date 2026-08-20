# SOL v0.1 Resolved Model Snapshot + Thermal Reference Environment Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Input findings:** THV-01, THV-02  
**Scope:** design-stage reference-model serialization only

## 1. Scope preservation

This proposal does not reopen the accepted thermal semantics or ADR-0028 package architecture.

Preserved decisions:

- the minimal thermal semantic graph is contract-level representable;
- backend projection remains official-document sanity validation only;
- backend install/license/runtime state is out of scope;
- ontology package resources and model instances remain separate identity spaces;
- package/namespace/canonical identity separation remains ADR-0009/0028 authority;
- `Value`, `UnitReference`, `DimensionVector`, and inline ValueDefinition remain ADR-0026/0027 authority;
- relation instances are graph statements by default, not schema resources.

The proposal adds only the smallest normalized model input needed by Thermal and later Plasma/QRC reference fixtures.

## 2. THV-01 — focused `ResolvedModelSnapshot`

A design-stage normalized reference model SHALL use a closed immutable snapshot envelope:

```yaml
snapshot_state: closed
ontology_environment:
  - package: "@simulation-ontology/core-reference-fixture"
    version: "0.1.0"
  - package: "@simulation-ontology/thermal-reference"
    version: "0.1.0"

entities:
  - id: "model:thermal-simulation"
    type: <canonical EntityType id>
  - id: "model:left-temperature"
    type: <canonical FixedTemperatureCondition type id>
    value_definition:
      mechanism: literal
      value: <ADR-0026 normalized Value>

relations:
  - relation: <canonical has_model RelationDefinition id>
    source: "model:thermal-simulation"
    target: "model:thermal-model"
```

### 2.1 Snapshot closure

`snapshot_state` is exactly `closed` in this focused v0.1 schema.

The `entities` and `relations` collections are the complete model graph used for semantic validation. Validators SHALL NOT search external model fragments for additional relation occurrences when evaluating Cardinality/QRC against this snapshot.

This explicit closure also supplies the closed-snapshot precondition required by ADR-0012.

### 2.2 Model-instance identity

A model-instance `id` is a nonempty opaque string unique within the resolved snapshot.

The exact globally unique URI format remains deferred under ADR-0009. Validators compare model-instance identity by exact normalized string equality inside one snapshot.

Model-instance IDs SHALL NOT be replaced by backend-local tags/handles.

### 2.3 Entity instance

Focused normalized entity instance:

```text
{
  id,
  type,
  value_definition?
}
```

Rules:

1. `type` is a canonical EntityTypeDefinition ID resolved in the declared ontology environment.
2. `value_definition`, when present, is an ADR-0026/0027 normalized `InlineValueDefinition`.
3. The field is optional and is used only where the Entity instance itself is the value-bearing semantic concept in the reference fixture.
4. Reified ValueDefinition mechanism payload serialization is not introduced by this focused fixture schema; reference cases that require it must justify a later extension.
5. Arbitrary backend/configuration fields are forbidden.

### 2.4 Relation edge

Focused normalized relation edge:

```text
{
  relation: canonical RelationDefinition ID,
  source: model-instance ID,
  target: model-instance ID
}
```

Rules:

1. relation/source/target all resolve exactly once;
2. source/target instance types satisfy the RelationDefinition endpoint contract using canonical type + subtype closure;
3. for `allowed_pairs`, the resolved source/target type pair must satisfy the authoritative allowed-pair contract;
4. identical `(relation, source, target)` triples SHALL NOT appear more than once in a normalized snapshot;
5. relation order is semantically irrelevant;
6. no edge-level identity/provenance/reification is introduced in the focused reference schema.

### 2.5 Cardinality and QRC evaluation

Because the graph is closed, semantic validation MAY evaluate accepted Cardinality/QRC ConstraintDefinitions against the complete relation set.

For ordinary source cardinality projection:

```text
for every applicable source instance
count distinct matching target instance identities
compare to authoritative Cardinality Constraint
```

QRC additionally filters by target semantic type/subtype under ADR-0012.

Duplicate syntactic relation triples are rejected before counting rather than silently influencing graph multiplicity.

## 3. Ontology environment binding

`ontology_environment` is a nonempty array of exact package identities:

```text
(package name, exact SemVer version)
```

Rules:

1. every declared package must exist exactly once in the supplied resolved package environment;
2. package ranges are forbidden;
3. the semantic validator uses ADR-0028 namespace/resource rules on those exact packages;
4. every model EntityType and RelationDefinition canonical reference resolves from that environment;
5. extra active package providers outside the snapshot environment do not silently affect model semantics.

For focused reference validation, the supplied package set SHALL match the snapshot's declared ontology environment exactly.

## 4. THV-02 — minimal committed reference package environment

A full production Core compiler/package is not required for the current stage.

Instead commit two normalized ADR-0028-conforming design fixtures:

### 4.1 Core reference fixture

```text
package: @simulation-ontology/core-reference-fixture
namespace provided: sol
```

It contains the Core EntityType/RelationDefinition/ConstraintDefinition resources required to evaluate the Thermal and planned Plasma/QRC reference fixtures.

Important rules:

- it is explicitly a design-validation fixture, not a production Core distribution;
- canonical IDs minted for included SOL resources are durable committed identity bindings;
- future production packages MAY provide namespace `sol` in another resolved environment while reusing the same canonical IDs;
- package identity does not become semantic identity.

For any included Core RelationDefinition, the fixture SHALL preserve its full accepted Core endpoint/cardinality semantics rather than narrowing the definition merely because the Thermal model uses a subset.

In particular, if `includes_component` is included, its full ADR-0016 allowed-pair matrix is included.

### 4.2 Thermal reference extension package

```text
package: @simulation-ontology/thermal-reference
namespace provided: thermal-ref
resolved dependency: exact core-reference-fixture version
```

Minimum specialized EntityTypes:

```text
ThermalConductionPhysics    is_a sol:PhysicsModel
SteadyHeatMathModel         is_a sol:MathematicalModel
FourierConductionLaw        is_a sol:ConstitutiveModel
TemperatureField            is_a sol:Field
ThermalConductivityProperty is_a sol:MaterialProperty
FixedTemperatureCondition   is_a sol:BoundaryCondition
```

No backend-native type appears in this package.

## 5. Required Core reference resources

The fixture must include enough Core EntityTypes for complete endpoint/allowed-pair semantics of the relation definitions it exports.

Relations required by Thermal:

```text
has_model
has_task
uses_model
has_analysis
includes_component
represented_by
closed_by
parameterized_by
defined_on
applied_to
```

Their accepted source-cardinality authorities are committed as reusable Cardinality ConstraintDefinitions and referenced by relation-side projections where required.

Because `includes_component` has an authoritative multi-source allowed-pair matrix, all EntityTypes named in that matrix are included in the fixture even if the Thermal model does not instantiate them.

Because `applied_to` has the accepted source/range families from ADR-0016, those endpoint EntityTypes are included as well.

This is still a fixture subset: unrelated Core relations/resources need not be added merely to mimic a production release.

## 6. Minimal thermal model snapshot

The first machine fixture shall instantiate:

```text
ThermalSimulation : Simulation
ThermalModel : SimulationModel
SteadyThermalTask : SimulationTask
ThermalPhysics : thermal-ref:ThermalConductionPhysics
HeatMathModel : thermal-ref:SteadyHeatMathModel
FourierLaw : thermal-ref:FourierConductionLaw
ThermalSpatialModel : SpatialModel
ThermalMaterialModel : MaterialModel
ThermalConditionModel : ConditionModel
TemperatureField_1 : thermal-ref:TemperatureField
HeatEquation_1 : Equation
SolidMaterial_1 : Material
ThermalConductivity_1 : thermal-ref:ThermalConductivityProperty
SolidDomainScope : Scope
LeftBoundaryScope : Scope
RightBoundaryScope : Scope
LeftFixedTemperature : thermal-ref:FixedTemperatureCondition
RightFixedTemperature : thermal-ref:FixedTemperatureCondition
StationaryThermalAnalysis : StationaryAnalysis
```

Relations are the graph accepted in the Thermal Research artifact.

Local inline literal values:

```text
ThermalConductivity_1 = 10 W/(m K)
LeftFixedTemperature  = 300 K
RightFixedTemperature = 400 K
```

A small non-normative reference metrology evidence table MAY be used by tests to resolve these explicit UnitReferences to the ADR-0020 DimensionVectors. This does not create Core Unit entities or a production metrology registry.

## 7. Semantic validation order

Focused reference validator:

```text
1. validate exact package environment under ADR-0028
2. build canonical schema-resource index
3. validate snapshot structural closure
4. resolve unique model-instance identities
5. resolve EntityType canonical references
6. resolve relation triples
7. validate endpoint/subtype/allowed-pair semantics
8. validate authoritative source cardinalities
9. validate InlineValueDefinition structure
10. evaluate focused metrology evidence when supplied
11. later: evaluate QRC on Plasma fixture using same closed-snapshot contract
```

Backend availability is not part of this sequence.

## 8. Required boundary cases

1. duplicate model entity ID -> FAIL;
2. unknown EntityType canonical ID -> FAIL;
3. backend-local type token -> FAIL canonical reference resolution;
4. relation source/target instance unresolved -> FAIL;
5. relation endpoint kind/type mismatch -> FAIL;
6. `includes_component` pair outside ADR-0016 matrix -> FAIL;
7. duplicate identical relation triple -> FAIL normalized snapshot;
8. `has_model` missing for Simulation -> cardinality FAIL;
9. `has_model` two targets -> cardinality FAIL;
10. `applied_to` FixedTemperatureCondition with no target -> cardinality FAIL;
11. package environment range instead of exact version -> structural FAIL;
12. declared package absent from supplied environment -> FAIL;
13. extra supplied package not declared by snapshot -> FAIL focused environment equality;
14. same model snapshot under relation declaration-order permutation -> same semantic result;
15. backend license/runtime absence -> no effect on model semantic verdict.

## 9. Deferred

- production Core package compiler;
- canonical ID generation algorithm beyond committed durable fixture IDs;
- general property-assignment model-instance schema;
- reified relation-instance metadata;
- reified ValueDefinition mechanism-payload schema;
- runtime execution state/results;
- Adapter/backend handles;
- distributed/open-world model fragments.

These are not required by the Thermal or planned Plasma/QRC design-stage stress fixtures unless a later counterexample demonstrates otherwise.

## 10. Finding closure proposal

| Finding | Proposed resolution |
|---|---|
| THV-01 | focused closed `ResolvedModelSnapshot` contract |
| THV-02 | committed normalized core-reference + thermal-reference package fixtures with durable canonical IDs |

## 11. Research verdict

**Ready for independent focused Validation.**
