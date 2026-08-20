# Core Simulation Ontology Architecture

**Version:** 0.1  
**Status:** **DESIGN-STAGE CLOSED by ADR-0030.** Core architecture is frozen through ADR-0017; focused language/schema/package/model-snapshot representation is accepted through ADR-0029; Thermal and Plasma/QRC reference gates are accepted.

## 1. Purpose

The Core Simulation Ontology provides a solver-independent and domain-independent semantic framework for simulation models. Software-specific concepts such as a MOOSE `Kernel`, COMSOL `Physics Feature`, or Ansys analysis object are backend representations rather than Core ontology concepts.

The Core answers what is modeled, how it is represented mathematically, what constitutive/material/spatial context applies, what computational task is performed, and how the semantic model can be validated and mapped without making backend object trees the source of truth.

## 2. Layering and dependency rule

```text
Application / Product
        │
        ▼
Simulation Profile
      ╱     ╲
     ▼       ▼
Domain     Backend
Ontology   Ontology
     ╲       ╱
      ▼     ▼
simulation-ontology
   Core Framework
```

Specialized layers depend inward toward Core. Core MUST NOT depend on domain ontologies, backend ontologies, profiles, applications, backend installation state, or commercial license state. Domain/backend composition belongs to a Profile and executable lowering belongs to MappingPlan/BackendAdapter contracts under ADR-0010/0011.

## 3. Top-level Simulation semantics

`SimulationModel` defines **what is modeled**. `Analysis` defines **what computational question is asked**. `SimulationTask` reifies the identifiable application of one Analysis to one model. `SolverConfiguration` defines **how the Analysis is solved**.

```text
Simulation
   ├── has_model ───────> exactly 1 SimulationModel
   └── has_task ────────> 1..* SimulationTask
                                ├── uses_model ───> exactly 1 SimulationModel
                                ├── has_analysis ─> exactly 1 Analysis
                                └── produces ─────> 0..* Result

Analysis ── solved_by (generic Core 0..*) ──> SolverConfiguration
```

For every `Simulation S`, if `S has_model M` and `S has_task T`, then `T uses_model M`. `has_model`, `has_task`, and `includes_component` are non-owning semantic references. Analysis and SolverConfiguration do not inherit from SimulationTask.

`SimulationModel -> analyzed_by -> Analysis` is derived only:

```text
M analyzed_by A
IFF
exists T:
  T uses_model M
  AND T has_analysis A
```

Task bindings are authoritative.

## 4. Direct model-component membership

ADR-0016 defines `includes_component` as direct-only, non-owning, non-transitive, unordered structural membership with generic source cardinality `0..*` and subtype-aware endpoint matching.

The allowed direct component matrix is:

```text
SimulationModel -> PhysicsModel | MathematicalModel | ConstitutiveModel |
                   SpatialModel | MaterialModel | ConditionModel |
                   NumericalModel | ObservationModel

PhysicsModel -> Phenomenon | Process | Interaction
MathematicalModel -> Formulation | Equation | Field | Operator | MathematicalParameter
ConstitutiveModel -> ClosureRelation | PropertyModel
SpatialModel -> Geometry | Domain | Boundary | SpatialInterface | Scope
MaterialModel -> Material | Species | MaterialProperty
ConditionModel -> BoundaryCondition | InitialCondition | Source | Load
NumericalModel -> Discretization | Mesh | NumericalApproximation
ObservationModel -> Quantity | Probe | Integral | Dataset | Output
SolverConfiguration -> NonlinearSolver | LinearSolver | Preconditioner | ConvergenceCriterion
```

No transitive stored membership is inferred.

## 5. Semantic inter-model relations

Structural membership does not replace semantic relations:

```text
PhysicsModel       ─ represented_by ─> MathematicalModel
MathematicalModel  ─ closed_by ─────> ConstitutiveModel
ConstitutiveModel  ─ parameterized_by > MaterialModel
MathematicalModel  ─ defined_on ────> SpatialModel
MathematicalModel  ─ discretized_by -> NumericalModel
Analysis           ─ solved_by ─────> SolverConfiguration
Result             ─ observed_by ───> ObservationModel
```

ADR-0017 assigns generic Core source cardinality `0..*` to these seven relations. Cardinality is normatively a Constraint; relation-side cardinality is a matching projection/cache only.

## 6. Conditions and spatial scope

`ConditionModel` is an aggregate. Individual condition/forcing entities target semantic fields/equations/scopes:

```text
BoundaryCondition | InitialCondition | Source | Load
             │
         applied_to (1..*)
             ▼
       Field | Equation | Scope
```

Endpoint matching uses canonical equal-or-subtype semantics. Backend selection IDs, sidesets, feature tags, and geometry handles are not Core identities.

`Scope` is first-class and abstracts backend scoping concepts without copying backend object identity into Core.

## 7. Entity taxonomy and Interface composition

Taxonomic inheritance is explicit only through `is_a`; diagram indentation or grouping is never inheritance. Entity Types have at most one direct `is_a` parent in v0.1. Orthogonal reusable capabilities use `Interface`.

`Interface` is the abstract capability-contract construct from ADR-0008/0025. `SpatialInterface` is the spatial Entity Type from ADR-0014.

Interface definitions may contain:

- Property requirements;
- Relation requirements;
- reusable targeted Constraint applications;
- extension of other Interfaces.

Concrete Entity Types satisfy Interface requirements through explicit canonical property/relation mappings. Effective Interface guarantees are inherited through Entity specialization and overlapping mappings must converge deterministically.

## 8. Constraint architecture

ADR-0007 defines six minimum Constraint families:

```text
Cardinality / QRC
Type
Value
Dimension
Compatibility
Conditional
```

Focused machine-readable schemas and semantic-validation boundaries are accepted through ADR-0018..0024.

Common design-stage validation state precedence is:

```text
FAIL > INDETERMINATE > PASS
```

`BLOCKED` or operational/resource interruption remains a separate invocation/precondition axis and is not silently collapsed into semantic PASS/FAIL.

QRC uses closed snapshot, distinct target identity, canonical target-type/subtype matching, and deterministic interval validation under ADR-0012/0029.

## 9. Value, Unit, Dimension, and ValueDefinition

The accepted representation boundary is:

```text
DimensionVector
  -> typed 7-axis canonical dimension payload

UnitReference
  -> typed reference resolved by external metrology context

Value
  -> evaluated typed datum; scalar/vector/tensor × scalar kind

InlineValueDefinition
  -> dependency-free local definition

ValueDefinition Entity
  -> reified only when independent identity, semantic dependency,
     reuse, or provenance requires graph participation
```

`Value`, `UnitReference`, and `DimensionVector` are not Core Entity Types. Missing unit does not imply dimensionless or backend default. Nonliteral inline definitions use an explicit format-provider contract under ADR-0027; semantic dependencies are extracted by that provider rather than guessed from raw expression text.

Reified `ValueDefinition` participates in `has_value_definition` and `depends_on` graph relations under ADR-0003/0026.

## 10. Canonical ontology package

ADR-0009/0028 separate:

```text
Distribution package
Semantic namespace / authoring names
Canonical persistent semantic identity
```

A resolved normalized ontology package contains exact package dependencies, explicit namespace export tables, canonical-ID resource collections, Interface definitions/implementations, and reusable ConstraintDefinitions. File paths, package names, versions, declaration order, and backend handles do not determine canonical semantic identity.

One active provider per visible semantic namespace is required in the v0.1 resolved environment. Namespace federation/augmentation is deferred.

## 11. Resolved model snapshot

ADR-0029 defines the design-stage machine-readable model-instance boundary used by reference validation:

```text
ResolvedModelSnapshot
├── snapshot_state = closed
├── exact ontology_environment
├── entities[]
│   ├── model-instance id
│   ├── canonical EntityType id
│   └── properties[]
│       ├── canonical PropertyDefinition id
│       └── InlineValueDefinition
└── relations[]
    ├── canonical RelationDefinition id
    ├── source model-instance id
    └── target model-instance id
```

The snapshot is a focused reference-validation boundary, not a claim that SOL v0.1 already defines every future application/model document feature.

## 12. Reference-model validation

### Minimal Thermal — PASS

The accepted Thermal fixture exercises:

- Simulation / SimulationModel / SimulationTask / StationaryAnalysis;
- direct component membership and semantic inter-model relations;
- fixed-temperature BoundaryConditions applied to Field/Scope;
- canonical PropertyDefinition assignment;
- Value / UnitReference;
- Interface-targeted Dimension Constraints;
- metrology PASS / INDETERMINATE / mismatch boundaries.

Official MOOSE, COMSOL, and Ansys thermal concepts provide no obvious cross-backend semantic counterexample to this structure.

### Minimal Plasma/QRC — PASS

The accepted dissociative-attachment fixture exercises:

- subtype-specialized Reaction and Species concepts;
- explicit `reactants` / `products` relations;
- Interface relation requirement mapping;
- independent QRC obligations for exactly one negative-ion product and exactly one neutral product;
- closed-snapshot, subtype-qualified, distinct-identity counting;
- missing/extra/generic-type/duplicate/unresolved/open-snapshot/order counterexamples.

## 13. Backend boundary

```text
Core Ontology / Domain Extension
           │
           ▼
Simulation IR / MappingPlan
           │
      ┌────┼────┐
      ▼    ▼    ▼
    MOOSE COMSOL Ansys
```

Backend-native ownership, solver trees, feature tags, installation, licenses, runtime APIs, and executable Adapter details remain outside Core semantics. Backend execution V&V belongs to the Adapter/integration stage unless execution reveals a genuine architecture counterexample.

## 14. Closed design-stage boundary

ADR-0030 closes SOL v0.1 design-stage architecture/language/package/reference-model design. Reopen requires evidence of a genuine architecture defect, normative contradiction, independent-validator nondeterminism, cross-backend semantic counterexample, or required reference-model impossibility not attributable to an explicitly deferred/Profile/Adapter/backend limitation.

The following remain post-design/deferred and do not reopen the baseline by default:

- production Adapter implementation and backend runtime execution V&V;
- installation/license availability;
- namespace federation/augmentation;
- multi-model/co-simulation semantics;
- richer future PropertyDefinition and Result sub-taxonomies;
- production Profile/backend package authoring beyond accepted mapping contracts;
- complete general application/model-document syntax beyond the ADR-0029 reference snapshot.

The next project stage is implementation, Adapter integration, and executable V&V against this closed semantic baseline.
