# SOL v0.1 Minimal Thermal Reference Model — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Input artifact:** `docs/research/sol-v0.1-minimal-thermal-reference-model-v0.1.md`  
**Normative basis:** Accepted ADR-0001..0028, with emphasis on ADR-0009, ADR-0015/0016, ADR-0020/0021, ADR-0024, ADR-0026/0027, ADR-0028  
**Backend evidence:** official MOOSE, COMSOL, and Ansys documentation only

## 1. Validation posture

The Research conclusion is not assumed correct. This review asks independently whether:

1. the thermal semantics can be represented without backend-specific Core leakage;
2. the same semantic obligations are supported by MOOSE, COMSOL, and Ansys official documentation;
3. the accepted SOL contracts are sufficient for a deterministic machine-readable reference input;
4. any missing runtime/install/license capability is actually relevant at this design stage.

## 2. Thermal semantic architecture verdict

**ACCEPT.**

The proposed reference problem is intentionally minimal and is representable with accepted SOL concepts:

```text
Simulation / SimulationModel / SimulationTask
PhysicsModel / MathematicalModel / ConstitutiveModel
SpatialModel / MaterialModel / ConditionModel
Field / Equation / MaterialProperty / Scope / BoundaryCondition
StationaryAnalysis
```

The relation graph uses already accepted contracts:

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

No new backend-shaped Core entity is required.

## 3. Official backend sanity verdict

**ACCEPT — semantic support exists in all three reference backends.**

### MOOSE

Official heat-transfer examples/documentation expose:

- temperature variable;
- HeatConduction kernel;
- thermal conductivity material property;
- Dirichlet temperature boundary condition scoped to a named boundary;
- Steady executioner.

References:

- https://mooseframework.inl.gov/moose/source/executioners/Steady.html
- https://mooseframework.inl.gov/moose/modules/heat_transfer/tutorials/introduction/therm_step02.html

### COMSOL

Official documentation exposes:

- Heat Transfer in Solids / Solid heat equation;
- thermal conductivity `k`;
- steady-state form of the heat equation;
- Temperature boundary feature applied to selected boundaries;
- Stationary study realization.

References:

- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.10.html
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.25.html
- https://doc.comsol.com/6.3/doc/com.comsol.help.comsol/comsol_ref_heattransfer.30.36.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html

### Ansys Mechanical

Official documentation exposes:

- Steady-State Thermal analysis;
- required Thermal Conductivity Engineering Data;
- Temperature boundary conditions;
- geometry/named-selection scoping plus temperature magnitude.

References:

- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/wb_sim/ds_static_thermal_analysis_type.html
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v242/en/wb_sim/ds_Given_Temperatures.html

The backend object decompositions differ, so the expected aggregate projection judgment remains `transformed` with no demonstrated semantic loss.

## 4. Counterexample review

No Architecture defect was found in the thermal case for:

- model/task identity;
- model-component composition;
- condition targeting;
- physics/math/constitutive separation;
- Value/Unit/Dimension separation;
- schema/model/backend identity separation;
- `exact / transformed / lossy / unsupported` representability vocabulary.

However the reference model cannot yet be made into one canonical machine-readable validator input without two focused readiness fixes.

## 5. THV-01 — no normalized model-instance snapshot contract

The accepted package schemas describe ontology **schema resources**, not concrete simulation model instances.

There is no accepted normalized machine contract for the minimum graph data needed by this reference case:

```text
model-instance id
model-instance EntityType reference
relation edge relation/source/target
optional local InlineValueDefinition
exact resolved ontology environment
closed-snapshot status
```

Without that contract, two independent validators can invent different fixture conventions while both claiming to load the same SOL model.

**Classification:** language/schema contract gap exposed by reference-model validation.  
**Architecture impact:** none if remediated as a serialization envelope only.  
**Verdict:** Revise reference-fixture readiness.

### Minimum required properties

A focused v0.1 `ResolvedModelSnapshot` should establish only:

1. exact ontology package environment;
2. a closed finite set of model entities;
3. unique opaque model-instance identity within the snapshot;
4. canonical EntityType reference for each model entity;
5. relation triples using canonical RelationDefinition identity plus model-instance endpoints;
6. optional inline ValueDefinition where ADR-0026/0027 permit it;
7. duplicate relation triples are not silently counted twice;
8. no backend-local identifiers or runtime state.

A normalized resolved snapshot should be closed by definition (or carry an explicit closed marker) so later QRC validation has deterministic graph closure evidence.

## 6. THV-02 — no committed canonical-ID reference package environment

ADR-0028 defines the normalized package/resource schema and canonical-ID rules, but the repository has not yet committed a normalized Core/reference package fixture with durable canonical resource IDs for the thermal model to reference.

Current `ontology/core/*.yaml` files remain source/consolidation fragments and do not contain persisted `canonical_id` bindings. Code search finds no committed `canonical_id` field in the source fragments.

Therefore a normalized thermal model cannot yet reference stable concrete IDs for types/relations from an actual committed reference environment without inventing IDs ad hoc in the model fixture.

**Classification:** machine-transcription / validation-fixture readiness defect, not Architecture defect.  
**Architecture impact:** none.  
**Verdict:** Revise reference-fixture readiness.

### Minimum remediation

Do **not** require full production package compilation. For design-stage closure, it is sufficient to commit a minimal normalized reference ontology package/environment containing only the Core/reference resources required by the Thermal and subsequent Plasma/QRC fixtures, provided:

- IDs are durable and committed;
- the fixture conforms to ADR-0028 package/resource schemas;
- namespace export names resolve deterministically to those IDs;
- future fixture updates reuse the same IDs when semantic identity is unchanged.

This avoids turning the current stage into a full compiler implementation project.

## 7. Installation/license/runtime boundary

MOOSE installation, COMSOL license availability, Ansys installation, solver execution, API access, and production Adapter implementation are **not findings** in this review.

They are intentionally outside the current design-stage gate. Their absence does not change the thermal semantic or package/model-contract verdict.

## 8. Final verdict

| Dimension | Verdict |
|---|---|
| Thermal Core semantic expressibility | **Accept** |
| MOOSE official-doc compatibility | **Accept** |
| COMSOL official-doc compatibility | **Accept** |
| Ansys official-doc compatibility | **Accept** |
| Architecture defect exposed | **No** |
| Canonical machine-readable reference fixture readiness | **Revise** |

**Overall workflow verdict: Revise**, limited to THV-01 and THV-02.

## 9. Next state

Return only THV-01/THV-02 to Research.

Recommended focused sequence:

```text
Research
  -> minimal ResolvedModelSnapshot contract
  -> minimal committed normalized reference ontology environment
Validation
  -> deterministic readback
Decision
  -> focused ADR if needed
  -> create Thermal machine-readable fixture
  -> validate fixture semantically
  -> proceed to Plasma/QRC stress model
```
