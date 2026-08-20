# SOL v0.1 Model-Side Tree Semantic Classification — Independent Review v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Evaluation inputs

- ADR-0008 — explicit inheritance / Interface composition
- ADR-0014 — no implicit `children` taxonomy
- ADR-0015 — explicit SimulationTask application semantics
- current architecture/entity/relation registries
- `docs/research/sol-v0.1-model-side-tree-semantic-classification-v0.1.md` as the proposal under test
- official backend documentation only as counterexample/reference evidence

Validation asks whether each displayed tree edge can safely be interpreted as `is_a` under the strict substitutability contract.

## 2. Executive verdict

**Classification proposal: ACCEPT.**

The current model-side trees are decomposition/grouping diagrams, not taxonomic hierarchies. The only Core tree edges currently proven and explicitly encoded as `is_a` are the Analysis specializations.

However, the classification exposes one existing relation-domain defect that should become the next Research input:

```text
REL-01: applied_to uses aggregate ConditionModel as domain
while its semantic description targets individual conditions.
```

This does not invalidate the classification; it is precisely the kind of hidden dependency on old tree semantics the classification was intended to reveal.

## 3. Substitutability counterexamples

### V-MT-01 — Field is not MathematicalModel

A Field has independent identity and participates in equations/operators. It cannot satisfy the complete semantic contract of a MathematicalModel by itself.

**Classification: composition/association — confirmed.**

### V-MT-02 — Boundary is not SpatialModel

A Boundary is one spatial entity within a spatial context and cannot substitute for geometry/domain/scope structure as a whole.

**Classification: composition/association — confirmed.**

### V-MT-03 — MaterialProperty is not MaterialModel

A thermal conductivity property or permittivity property can be reused independently and does not constitute a complete MaterialModel.

**Classification: composition/association — confirmed.**

### V-MT-04 — Mesh is not NumericalModel

A mesh supplies geometric/topological discretization data but does not by itself define all numerical representation/approximation semantics.

**Classification: composition/association — confirmed.**

### V-MT-05 — BoundaryCondition is not ConditionModel

A BoundaryCondition is an individual condition construct. A ConditionModel is the model-side context/aggregate of conditions and forcing. Treating BC as an implicit ConditionModel subtype would make aggregate-level constraints silently apply to one condition object.

MOOSE official examples support the distinction: BC objects independently bind a variable and boundary inside a wider problem/input model.

**Classification: composition/association — confirmed.**

### V-MT-06 — ClosureRelation / PropertyModel

These terms could superficially be read as specialized constitutive models, but the accepted architecture defines `ConstitutiveModel` as the context that **defines closure relations and property models required to close the mathematical system**. Without an explicit subtype decision, the strict ADR-0008 rule forbids inferring inheritance from that grouping.

**Classification for v0.1 baseline: composition/association — confirmed.**

A later domain ontology may define taxonomic specializations beneath these concepts without changing this generic aggregate relation.

### V-MT-07 — Analysis specializations

`StationaryAnalysis`, `TransientAnalysis`, `FrequencyDomainAnalysis`, `EigenvalueAnalysis`, `ParametricAnalysis`, and `OptimizationAnalysis` are specialized computational questions and satisfy the Analysis contract.

These edges are already explicit `is_a` in `entities.yaml`.

**Classification: inheritance — confirmed.**

## 4. Aggregate classification matrix

The following displayed parent→child groups are **not `is_a`** in the current Core baseline:

- SimulationModel → PhysicsModel / MathematicalModel / ConstitutiveModel / SpatialModel / MaterialModel / ConditionModel / NumericalModel / ObservationModel
- PhysicsModel → Phenomenon / Process / Interaction
- MathematicalModel → Formulation / Equation / Field / Operator / MathematicalParameter
- ConstitutiveModel → ClosureRelation / PropertyModel
- SpatialModel → Geometry / Domain / Boundary / SpatialInterface / Scope
- MaterialModel → Material / Species / MaterialProperty
- ConditionModel → BoundaryCondition / InitialCondition / Source / Load
- NumericalModel → Discretization / Mesh / NumericalApproximation
- ObservationModel → Quantity / Probe / Integral / Dataset / Output
- SolverConfiguration → NonlinearSolver / LinearSolver / Preconditioner / ConvergenceCriterion

`SimulationTask` is governed separately by ADR-0015 and has no Analysis/SolverConfiguration inheritance edge.

## 5. Newly exposed REL-01 defect

Current `relations.yaml` states:

```text
applied_to
  domain: ConditionModel
  range: Field | Equation | Scope
```

while its description says it associates **a condition** with what it constrains/forces.

Because:

```text
BoundaryCondition !is_a ConditionModel
InitialCondition  !is_a ConditionModel
Source            !is_a ConditionModel
Load              !is_a ConditionModel
```

an independent validator has two incompatible choices:

1. accept `applied_to` only from the aggregate ConditionModel, contradicting the relation's individual-condition wording and reference-model usage; or
2. implicitly treat the condition leaf types as ConditionModel subtypes, violating ADR-0014/0008.

**Classification: Architecture relation-domain defect.**

The next Research step must repair this without restoring implicit inheritance. Candidate solution families include:

- introduce a justified common `Condition` semantic Entity Type with explicit `is_a` edges from BoundaryCondition/InitialCondition/Source/Load;
- use an Interface/capability contract as relation domain;
- use an explicit domain union of concrete condition types.

Validation does not select a solution here.

## 6. Other broad relation domains

`represented_by`, `closed_by`, `parameterized_by`, `defined_on`, and `discretized_by` can still be read consistently as relations between the aggregate model contexts named in their explicit domains/ranges. They do not automatically apply to leaf components.

Therefore no defect is established merely because leaf concepts cannot use those relations. Their exact component-level relations remain future design work.

## 7. Final verdict

| Question | Verdict |
|---|---|
| Model-side tree = implicit taxonomy | **Reject** |
| Model-side tree = semantic decomposition/grouping | **Accept** |
| Analysis specialization `is_a` edges | **Accept** |
| Current flat entity registry strategy | **Accept** |
| Remaining cardinality design ready immediately | **No** |
| New architecture defect found | **Yes — REL-01 `applied_to` domain** |
| Backend runtime/install needed to resolve | **No** |

**Final verdict: ACCEPT classification; route REL-01 plus explicit model-component relation design to Research before completing the remaining cardinality matrix.**
