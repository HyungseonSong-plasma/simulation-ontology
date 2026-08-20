# SOL v0.1 Machine-Readable Core Consolidation — Readback Validation v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Validation inputs

This readback used only the accepted contract and current durable artifacts:

- ADR-0008 — Inheritance and Interface Composition
- ADR-0014 — Interface Disambiguation and Machine-Readable Taxonomy Semantics
- `docs/architecture.md`
- `docs/ontology-language.md`
- `ontology/core/entities.yaml`
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`

The prior Research editing process and prior PASS conclusions were not evaluation inputs.

## 2. Evaluation criteria

The transcription passes only if independent readers are forced to the same interpretation for the ADR-0014 scope:

1. unqualified `Interface` has only capability-contract meaning;
2. the spatial entity is `SpatialInterface`;
3. no machine-readable `children` field carries inheritance semantics;
4. every taxonomic inheritance edge is explicit `is_a` and obeys single-direct-parent semantics;
5. `Result` resolves as a Core Entity Type;
6. every current Core relation domain/range endpoint resolves to the current entity registry;
7. `produces` is consistently `SimulationTask -> Result`;
8. accepted versus open machine-readable work is separated;
9. no backend-native semantics are introduced into Core;
10. open composition/cardinality questions are not silently decided.

## 3. Findings

### RB-01 — Interface identity

`ontology/core/entities.yaml` contains `SpatialInterface` and does not export a spatial Entity Type named `Interface`. `docs/architecture.md` explicitly reserves unqualified `Interface` for the ADR-0008 capability contract and names the spatial concept `SpatialInterface`. `docs/ontology-language.md` does the same.

**Verdict: PASS.**

### RB-02 — Taxonomy representation

The entity registry no longer has a `children` structural field. It explicitly states that file order/grouping is non-semantic and that normative inheritance requires `is_a`.

The only current explicit `is_a` edges are:

```text
StationaryAnalysis -> Analysis
TransientAnalysis -> Analysis
FrequencyDomainAnalysis -> Analysis
EigenvalueAnalysis -> Analysis
ParametricAnalysis -> Analysis
OptimizationAnalysis -> Analysis
```

Each has one direct parent. No diagram grouping for `Simulation`, `SimulationModel`, `SimulationTask`, model categories, solver categories, or observation categories has been silently converted to inheritance.

**Verdict: PASS.**

### RB-03 — Result referential completeness

`Result` is present in the entity registry. `relations.yaml` resolves:

```text
SimulationTask -> produces -> Result
Result -> observed_by -> ObservationModel
```

The same semantics appear in `docs/architecture.md` and `docs/ontology-language.md`.

**Verdict: PASS.**

### RB-04 — Relation endpoint resolution

Every domain/range identifier currently used in `ontology/core/relations.yaml` resolves to an Entity Type declared in `ontology/core/entities.yaml`:

```text
PhysicsModel
MathematicalModel
ConstitutiveModel
MaterialModel
SpatialModel
NumericalModel
ConditionModel
Field
Equation
Scope
SimulationModel
Analysis
SolverConfiguration
SimulationTask
Result
ObservationModel
```

No relation endpoint requires a backend-local type or an undeclared Core type.

**Verdict: PASS.**

### RB-05 — `produces` consistency

The machine-readable relation registry and the architecture document both use:

```text
SimulationTask -> produces -> Result
```

The former misleading diagram placement under `SolverConfiguration` has been removed.

**Verdict: PASS.**

### RB-06 — Interface status

`docs/ontology-language.md` now treats `Interface` as normative under ADR-0008/0014. It no longer labels Interface provisional. Method/action requirements remain deferred, preserving ADR-0008 scope.

**Verdict: PASS.**

### RB-07 — State separation

The machine-readable files use `status: consolidating` and distinguish:

```text
accepted_semantics_not_yet_transcribed
```

from:

```text
open_language_schema_decisions
```

Already accepted identity, unit/dimension, Interface, and QRC semantics are not incorrectly presented as unresolved architecture decisions.

**Verdict: PASS.**

### RB-08 — Scope preservation

No backend-native MOOSE, COMSOL, or Ansys object type was added to the Core registry. No new MappingRule/MappingClaim/MappingPlan semantics were introduced. No Result subtype taxonomy, new top-level composition relation, or global cardinality matrix was invented during transcription.

**Verdict: PASS.**

## 4. Residual open items

The following remain intentionally open and are not readback failures:

- explicit composition relations/cardinalities connecting `Simulation`, `SimulationModel`, and `SimulationTask`;
- complete Core relation cardinalities and required/optional matrix;
- additional `is_a` edges only where taxonomic meaning is independently justified;
- final language-level serialization of Interface definitions/implementation mappings;
- final representation of accepted Value/ValueDefinition/PhysicalDimension/Unit semantics.

These require focused Research/Validation cycles rather than transcription inference.

## 5. Final verdict

| Question | Verdict |
|---|---|
| ADR-0014 transcription fidelity | **Accept** |
| Canonical Interface collision removed | **Yes** |
| `children` ambiguity removed | **Yes** |
| Result relation endpoints resolvable | **Yes** |
| `produces` domain consistent | **Yes** |
| Backend semantics leaked into Core | **No** |
| Open design questions silently resolved | **No** |
| New revision required | **No** |

**Final verdict: ACCEPT.**

The ADR-0014 transcription pass is complete. The next focused language-design question should be the explicit top-level composition semantics and cardinalities connecting `Simulation`, `SimulationModel`, and `SimulationTask`, because ADR-0014 intentionally removed the ambiguous `children` shorthand without replacing it by guessed relations.
