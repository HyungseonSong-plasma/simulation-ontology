# SOL v0.1 ADR-0015 Transcription — Readback Validation v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Inputs

- ADR-0015 — Simulation–Model–Task Composition Semantics
- current `ontology/core/entities.yaml`
- current `ontology/core/relations.yaml`
- current `ontology/core/constraints.yaml`
- current `docs/architecture.md`
- current `docs/ontology-language.md`

Prior Research narrative was not used as a correctness assumption.

## 2. Evaluation criteria

The transcription must preserve:

```text
Simulation has_model exactly 1 SimulationModel
Simulation has_task 1..* SimulationTask
SimulationTask uses_model exactly 1 SimulationModel
SimulationTask has_analysis exactly 1 Analysis
SimulationTask produces 0..* Result
```

plus:

- Simulation task-model consistency;
- non-owning `has_model` / `has_task` semantics;
- Analysis and SolverConfiguration are not SimulationTask subtypes;
- `analyzed_by` is derived only from task bindings;
- no multi-model/co-simulation semantics are introduced;
- YAML serialization choices are not silently promoted into broader normative schema rules.

## 3. Readback findings

### RB15-01 — Entity identities and inheritance

`Simulation`, `SimulationModel`, `SimulationTask`, `Analysis`, `SolverConfiguration`, and `Result` all resolve in the entity registry.

There is no:

```text
Analysis is_a SimulationTask
```

or:

```text
SolverConfiguration is_a SimulationTask
```

edge. The existing Analysis specializations continue to inherit only from `Analysis`.

**PASS.**

### RB15-02 — Top-level relation cardinalities

`relations.yaml` transcribes:

- `has_model`: min 1, max 1;
- `has_task`: min 1, max unbounded;
- `uses_model`: min 1, max 1;
- `has_analysis`: min 1, max 1;
- `produces`: min 0, max unbounded.

Domain/range identifiers resolve to the entity registry and match ADR-0015.

**PASS.**

### RB15-03 — Non-owning semantics

`has_model` and `has_task` are explicitly recorded as non-owning references, and architecture/language prose states that no deletion/lifecycle cascade is implied.

**PASS.**

### RB15-04 — Simulation consistency

`constraints.yaml`, `docs/architecture.md`, and `docs/ontology-language.md` all state the same invariant:

```text
S has_model M
AND S has_task T
=> T uses_model M
```

No artifact permits container position to override task binding.

**PASS.**

### RB15-05 — Derived `analyzed_by`

`relations.yaml` marks `analyzed_by` as `derived: true` with:

```text
M analyzed_by A iff exists T:
  T uses_model M
  and T has_analysis A
```

The same authority rule appears in the constraint and human-readable language artifacts. No independent authored `analyzed_by` truth is introduced.

**PASS.**

### RB15-06 — Analysis / SolverConfiguration separation

Architecture and language artifacts keep:

```text
Analysis -> solved_by -> SolverConfiguration
```

separate from task identity. Solver cardinality and task-specific override behavior remain explicitly open.

**PASS.**

### RB15-07 — Scope preservation

No:

- backend-native object type;
- production Adapter lifecycle;
- multi-model/co-simulation contract;
- Result subtype;
- task workflow/dependency model;
- solver override contract

was introduced.

**PASS.**

### RB15-08 — Machine serialization boundary

`source_cardinality`, `ownership`, `order_semantics`, and derived-relation YAML fields are being used as current consolidation representations. The files explicitly keep the **final canonical YAML/JSON Schema representation** open.

Therefore these field spellings are not silently elevated to a universal SOL serialization contract by the transcription itself.

**PASS with consolidation note.**

## 4. Final verdict

| Item | Verdict |
|---|---|
| ADR-0015 transcription fidelity | **Accept** |
| relation endpoint completeness | **Accept** |
| cardinality fidelity | **Accept** |
| task/model consistency | **Accept** |
| derived `analyzed_by` authority | **Accept** |
| Analysis/Solver separation | **Accept** |
| scope leakage | **None** |
| revision required | **No** |

**Final verdict: ACCEPT.**

ADR-0015 transcription is complete at the current consolidation level. The next language/schema work should not revisit these relations unless a new semantic counterexample appears.
