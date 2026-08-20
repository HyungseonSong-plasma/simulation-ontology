# Core Simulation Ontology v0.1 — Implementation Development Plan

**Status:** Baseline  
**Ontology baseline:** Core Simulation Ontology v0.1  
**Architecture:** Rust semantic Core + TypeScript/React primary client + polyglot backend adapters  
**Development policy:** CI-first, counterexample-driven  
**Initial vertical slice:** Thermal reference model  
**Initial backend:** Mock Adapter only

## 1. Purpose

This document defines the implementation plan for turning Core Simulation Ontology v0.1 into an executable, solver-independent semantic system.

The implementation must preserve the ontology-level separation:

```text
Simulation
├── SimulationModel
│   ├── PhysicsModel
│   ├── MathematicalModel
│   ├── ConstitutiveModel
│   ├── SpatialModel
│   ├── MaterialModel
│   ├── ConditionModel
│   ├── NumericalModel
│   └── ObservationModel
└── SimulationTask
    ├── Analysis
    └── SolverConfiguration
```

The Rust Core is the canonical authority for simulation meaning. Backend adapters realize that meaning in solver-native ecosystems. The GUI is a client of the semantic Core and must not define backend semantics.

## 2. Architectural Invariants

1. Physics and Mathematics remain separate.
2. Mathematical formulation and constitutive closure remain separate.
3. SimulationModel and SimulationTask remain separate.
4. Analysis and SolverConfiguration remain separate.
5. Physics and numerics remain separate.
6. Spatial Scope is a first-class semantic concept.
7. MOOSE, COMSOL, and Ansys native objects never become Core ontology concepts.
8. Semantic relations are first-class and machine-checkable.
9. One SimulationModel can be reused by multiple SimulationTasks.
10. Backend-specific alias, rename, deprecation, and compatibility behavior is adapter-defined unless a minimal Core contract is required.
11. Core and adapters do not use direct FFI as the architectural boundary.
12. Backend realizability must not redefine ontology truth.

## 3. Target Architecture

```text
TypeScript / React GUI
        │
        ▼
Primary TypeScript Client
        │
        ▼
JSON-RPC / IPC Protocol Boundary
        │
        ▼
Rust Semantic Core
├── Data Model / Serialization
├── Canonical Identity / Resolver
├── Constraint / QRC Engine
├── MappingRule / MappingClaim
├── RealizationEffect / Comparator
├── PlanAction / MappingPlan DAG
├── Evaluation Lifecycle
└── BackendTarget Resolver
        │
        ▼
Adapter Contract
   ┌────┼──────────────┐
   ▼    ▼              ▼
Mock  MOOSE          COMSOL          Ansys
      Python/C++     Java            Python
```

Initial implementation stops at Mock Adapter. Real backend adapters and TypeScript binding are gated behind M0.1 stability.

## 4. Semantic Relation Vocabulary

The initial Core relation vocabulary is:

```text
represented_by
closed_by
parameterized_by
defined_on
discretized_by
applied_to
analyzed_by
solved_by
produces
observed_by
```

These relations are represented as semantic graph edges, not inferred solely from Rust type hierarchy.

Representative graph:

```text
PhysicsModel
   │ represented_by
   ▼
MathematicalModel
   │ closed_by
   ▼
ConstitutiveModel
   │ parameterized_by
   ▼
MaterialModel

MathematicalModel ──defined_on────► SpatialModel
MathematicalModel ──discretized_by► NumericalModel
SimulationModel   ──analyzed_by───► Analysis
Analysis          ──solved_by─────► SolverConfiguration
```

## 5. Canonical Identity

Every semantic entity must have solver-independent identity.

Conceptual shape:

```text
CanonicalEntity
├── id
├── kind
├── namespace
├── ontology_version
└── semantic_payload
```

Example canonical IDs:

```text
thermal.temperature
thermal.energy_equation
material.copper
scope.wall
analysis.stationary
```

Backend-native identity is attached through mapping/realization metadata and must not become the canonical identifier.

## 6. Constraint / QRC Engine

The Constraint/QRC engine determines semantic validity independently of backend realizability.

Initial constraint categories:

- Type Constraint
- Relation Constraint
- Cardinality Constraint
- Reference Constraint
- Required-Relation Constraint
- Compatibility Constraint

Example:

```text
PhysicsModel --represented_by--> MathematicalModel   => valid candidate
PhysicsModel --solved_by-------> MaterialModel       => invalid
```

Core semantic validity and backend realizability are separate evaluations.

## 7. Mapping Layer

The mapping layer separates ontology truth from claims about backend realization.

```text
Core Entity / Relation
        │
        ▼
MappingRule
        │
        ▼
MappingClaim
        │
        ▼
RealizationEffect
        │
        ▼
Comparator
```

### MappingRule

Defines when a semantic pattern can be realized by a backend capability.

### MappingClaim

Represents a concrete claim that a rule applies to a specific semantic entity or relation.

### RealizationEffect

Represents the semantic effect of realization, including exact, compatible, degraded, unsupported, or unknown outcomes as the taxonomy matures.

### Comparator

Compares semantic intent against proposed or observed realization effects.

## 8. MappingPlan / PlanAction DAG

Backend realization order is represented as a DAG.

```text
MappingPlan
├── PlanAction
├── dependency edges
├── topological ordering
└── cycle detection
```

Representative action ordering:

```text
CreateGeometry
  ↓
CreateMaterial
  ↓
CreateField
  ↓
CreateEquation
  ↓
ApplyBoundaryCondition
  ↓
ConfigureAnalysis
```

PlanAction is operational and is not a Core ontology entity.

## 9. Evaluation Lifecycle

All validation and mapping evaluations use four states:

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

Definitions:

- **PASS:** sufficient evidence confirms the requirement.
- **FAIL:** sufficient evidence confirms violation or mismatch.
- **BLOCKED:** evaluation/execution cannot proceed because a prerequisite or capability is unavailable.
- **INDETERMINATE:** evaluation ran, but evidence is insufficient to decide PASS or FAIL.

Invariant:

```text
BLOCKED != FAIL
INDETERMINATE != FAIL
```

## 10. BackendTarget Resolver

Backend selection is modeled separately from semantic truth.

Conceptual structure:

```text
BackendTarget
├── backend_family
├── backend_version
├── capabilities
└── adapter_identity
```

Resolution flow:

```text
SimulationModel
+
SimulationTask
+
Target requirements
        │
        ▼
BackendTarget Resolver
        │
        ▼
Candidate Adapter
```

Backend metadata may constrain realization, but must not alter the canonical ontology graph.

## 11. Recommended Repository Structure

```text
simulation-ontology/
├── crates/
│   ├── sol-core-model/
│   ├── sol-core-identity/
│   ├── sol-core-constraint/
│   ├── sol-core-mapping/
│   ├── sol-core-effect/
│   ├── sol-core-plan/
│   ├── sol-core-lifecycle/
│   ├── sol-adapter-contract/
│   ├── sol-adapter-mock/
│   ├── sol-target-resolver/
│   └── sol-cli/
├── fixtures/
│   ├── thermal/
│   ├── counterexamples/
│   └── golden/
├── tests/
│   ├── architecture/
│   ├── contract/
│   └── property/
├── schemas/
├── docs/
│   ├── adr/
│   └── plans/
└── .github/
    └── workflows/
```

Crates may be consolidated if implementation overhead is excessive, but semantic dependency boundaries must remain explicit.

## 12. CI-First Policy

GitHub Actions is the initial architecture authority.

Minimum CI gates:

```text
cargo build
cargo fmt --check
cargo clippy
cargo test
architecture counterexample fixtures
golden tests
contract tests
property tests
```

A feature is not considered integrated until its architecture fixtures run in CI.

## 13. Counterexample-Driven Validation

Validation Lab findings become executable architecture specifications.

```text
Validation Lab
    │
    ▼
Positive / Negative Counterexample
    │
    ▼
Fixture
    ├── Golden Test
    ├── Contract Test
    └── Property Test
```

Positive fixtures prove allowed structures remain accepted. Negative fixtures prove forbidden structures remain rejected.

## 14. First Vertical Slice: Thermal Reference Model

The first vertical slice must exercise the entire semantic pipeline with one small model.

```text
Simulation
├── SimulationModel
│   ├── PhysicsModel
│   │   └── ThermalTransport
│   ├── MathematicalModel
│   │   ├── EnergyConservation
│   │   └── TemperatureField
│   ├── ConstitutiveModel
│   │   └── FourierLaw
│   ├── MaterialModel
│   │   ├── Material
│   │   └── ThermalConductivity
│   ├── SpatialModel
│   │   ├── Domain
│   │   └── Boundary
│   ├── ConditionModel
│   │   └── TemperatureBoundaryCondition
│   ├── NumericalModel
│   │   └── FiniteElement
│   └── ObservationModel
│       └── MaximumTemperature
└── SimulationTask
    ├── StationaryAnalysis
    └── SolverConfiguration
```

Relations:

```text
ThermalTransport --represented_by----> EnergyConservation
EnergyConservation --closed_by-------> FourierLaw
FourierLaw --parameterized_by--------> ThermalConductivity
EnergyConservation --defined_on------> Domain
EnergyConservation --discretized_by--> FiniteElement
TemperatureBoundaryCondition --applied_to--> TemperatureField / Boundary
SimulationModel --analyzed_by--------> StationaryAnalysis
StationaryAnalysis --solved_by-------> SolverConfiguration
```

The objective is not to solve heat transfer. The objective is to validate the semantic translation architecture end-to-end.

## 15. Vertical Slice Execution Pipeline

```text
thermal.json
   ↓
Deserialize
   ↓
Canonical Identity Resolver
   ↓
Constraint / QRC Validation
   ↓
MappingRule Resolution
   ↓
MappingClaims
   ↓
RealizationEffect / Comparator
   ↓
MappingPlan DAG
   ↓
Mock Adapter
   ↓
PASS / FAIL / BLOCKED / INDETERMINATE
   ↓
sol-cli output
```

## 16. Implementation Sequence

The implementation order is intentionally strict.

### Phase 0 — Repository / CI Bootstrap

Deliverables:

- Rust workspace
- minimum CI workflow
- fixture directories
- architecture test entry point

Exit gate: build/test/fmt/clippy are executable in CI.

### Phase 1 — Data Model / Serialization

Deliverables:

- Core entities
- Core relations
- Simulation / SimulationModel / SimulationTask
- serialization DTOs

Exit gate: Thermal model serialization round-trip passes.

### Phase 2 — Canonical Identity & Resolver

Deliverables:

- canonical IDs
- namespaces
- entity references
- relation endpoint resolution

Exit gate: all Thermal graph references resolve deterministically.

### Phase 3 — Constraint / QRC Engine

Deliverables:

- type constraints
- relation constraints
- required relations
- cardinality
- reference integrity

Exit gate: positive fixtures PASS and negative fixtures FAIL.

### Phase 4 — MappingRule / MappingClaim

Deliverables:

- MappingRule
- MappingClaim
- applicability evaluation
- evidence representation

Exit gate: Thermal entities produce deterministic mapping claims.

### Phase 5 — RealizationEffect / Comparator

Deliverables:

- effect model
- semantic comparison
- effect aggregation

Exit gate: exact and degraded realization counterexamples are distinguishable.

### Phase 6 — PlanAction / MappingPlan DAG

Deliverables:

- PlanAction
- dependency edges
- topological ordering
- cycle detection

Exit gate: valid plan is ordered; cyclic plan is rejected.

### Phase 7 — Evaluation Lifecycle

Deliverables:

- PASS
- FAIL
- BLOCKED
- INDETERMINATE

Exit gate: all relevant evaluation paths terminate deterministically in one lifecycle state.

### Phase 8 — Mock Adapter

Deliverables:

- capability declaration
- plan acceptance/rejection
- simulated realization result
- failure injection
- blocked capability behavior
- indeterminate response behavior

Exit gate: end-to-end Core↔Adapter contract can be tested without solver installation.

### Phase 9 — BackendTarget Resolver

Deliverables:

- BackendTarget
- BackendCapability
- AdapterDescriptor
- resolver

Exit gate: Thermal MappingPlan deterministically selects an eligible Mock Adapter.

### Phase 10 — `sol-cli`

Minimum commands:

```bash
sol-cli validate thermal.json
sol-cli plan thermal.json --target mock
```

Required output includes canonical validation status and MappingPlan summary.

## 17. M0.1 — Semantic Core Bootstrap

M0.1 is complete only when all three gates pass:

1. **Rust Core builds successfully in CI.**
2. **Architecture counterexample tests execute in CI.**
3. **`sol-cli` outputs canonical validation and MappingPlan results for the Thermal reference model.**

### Explicit M0.1 Non-Goals

Do not begin before M0.1 is stable:

- real MOOSE adapter
- real COMSOL adapter
- real Ansys adapter
- TypeScript Node/native binding
- React integration
- solver installation dependency
- backend execution

## 18. M0.1 Architecture Gate

M0.1 requires YES to all of the following.

### Ontology

- Physics and Mathematics are separate.
- MathematicalModel and ConstitutiveModel are separate.
- SimulationModel and Analysis are separate.
- Analysis and SolverConfiguration are separate.
- Scope is first-class.
- backend-native objects do not enter Core.

### Identity

- every semantic entity has canonical identity.
- backend identity is separate.

### Constraints

- invalid relations are rejected.
- missing references are detected.
- cardinality violations are representable and detectable.

### Mapping

- semantic truth is separate from backend mapping claims.
- MappingClaim can carry evidence/provenance.

### Planning

- realization order is represented as a DAG.
- cycles are detectable.

### Lifecycle

- PASS / FAIL / BLOCKED / INDETERMINATE are distinguishable.

### Architecture

- Core imports no backend implementation package.
- Mock Adapter supports the end-to-end vertical slice.

## 19. Post-M0.1 Sequence

```text
M0.1 Semantic Core Stable
        ↓
Protocol Contract Stabilization
        ↓
JSON-RPC / IPC
        ↓
TypeScript Client
        ↓
React Integration
        ↓
Real Backend Adapters
```

The TypeScript layer should consume stable protocol DTOs instead of depending directly on Rust internal types.

## 20. Real Backend Adapter Responsibility

Each real adapter owns:

- backend capability discovery
- MappingRule implementation/evidence
- MappingPlan realization
- backend-native identity
- version compatibility
- backend-specific alias / rename / deprecation
- execution
- result extraction

Adapters do not own Core semantic truth.

## 21. Development Control Loop

```text
Research Lab
    │ architecture / ontology decision
    ▼
ADR
    ▼
Validation Lab
    │ positive / negative counterexample
    ▼
Fixture
    ▼
CI Test
    ▼
Core Implementation
    ▼
sol-cli
    ▼
Architecture Validation
    └──────────────► Research Lab
```

Architecture authority is therefore the combination of:

```text
Ontology + ADR + Counterexample + Executable Test
```

## 22. Definition of Done for a Core Feature

A Core feature is done only when:

1. semantic definition exists;
2. ontology relation is identified;
3. canonical representation exists;
4. constraints are defined;
5. positive fixture exists;
6. negative fixture exists;
7. serialization tests pass;
8. counterexample tests pass;
9. CLI inspection is possible where applicable;
10. no backend-specific dependency enters Core.

For mapping features, additionally:

11. MappingRule exists;
12. MappingClaim evidence exists;
13. RealizationEffect is representable;
14. MappingPlan remains a valid DAG.

## 23. GitHub Planning Policy

GitHub is the execution source of truth for implementation progress.

- `docs/plans/` stores approved development plans.
- `docs/adr/` stores architecture decisions.
- GitHub Issues track implementation units and validation gates.
- Pull requests, when used, must link the relevant issue and include test evidence.
- Validation Lab counterexamples must be converted into repository fixtures before the associated issue is considered complete.
- The M0.1 master issue tracks phase-level progress and gate status.
- Implementation scope does not advance to real adapters or TypeScript integration until the M0.1 issue is complete.

## 24. Canonical Implementation Rule

> The Rust Core determines what a simulation means. An Adapter determines how that meaning is realized in a specific backend. The GUI is only a client for creating, inspecting, and manipulating the semantic model.

Dependency direction must remain:

```text
GUI
 ↓
Semantic Protocol
 ↓
Rust Core
 ↓
Adapter Contract
 ↓
Backend Adapter
 ↓
Simulation Backend
```

Backend-specific semantics must never become the authority for Core Ontology semantics.

## 25. Primary External References

Backend mapping and compatibility claims should be verified against authoritative documentation:

- MOOSE Framework: https://mooseframework.inl.gov/
- COMSOL: https://www.comsol.com/
- Ansys: https://ansys.synopsys.com/
- Palantir Ontology concepts: https://www.palantir.com/uk/

These references support backend/adaptor validation and ontology design comparison; they do not replace the project's own Core semantic contract.
