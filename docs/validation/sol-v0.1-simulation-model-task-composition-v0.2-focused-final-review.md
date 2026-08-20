# SOL v0.1 Simulation–Model–Task Composition v0.2 — Focused Final Review

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Scope

This review evaluates only the delta introduced by:

- `docs/research/sol-v0.1-simulation-model-task-composition-semantics-proposal-v0.2.md`

against the prior independent findings. Previously accepted top-level decisions (`has_model` exactly one, `has_task` one-or-more, non-owning model reuse, zero-result pre-execution validity) are not reopened without a new counterexample.

## 2. Evaluation criteria

The revision passes only if:

1. `SimulationTask`, `Analysis`, `SimulationModel`, and `SolverConfiguration` remain semantically distinct;
2. a reusable Analysis can be applied to multiple models without Analysis duplication;
3. task identity can carry independent provenance/result relationships;
4. task-to-model and task-to-Analysis bindings are single-valued and deterministic;
5. `analyzed_by` does not create a second conflicting source of truth;
6. a Simulation's tasks cannot silently bind a different model;
7. no backend-native or Adapter-runtime semantics enter Core;
8. the revision does not expand into multi-model/co-simulation or solver override design.

## 3. Counterexample replay

### F-ST-01 — reusable Analysis across two models

```text
T1 uses_model M1
T1 has_analysis A

T2 uses_model M2
T2 has_analysis A
```

The same `A` is reused without duplication. The tasks remain independently identifiable.

**Result: PASS.**

### F-ST-02 — independent task result identity

```text
T1 has_analysis A
T2 has_analysis A
T1 produces R1
T2 produces R2
```

Result identity belongs to task application, not to the reusable Analysis definition.

**Result: PASS.**

### F-ST-03 — task/model ambiguity

`uses_model` requires exactly one SimulationModel per task. Zero or multiple model bindings are cardinality conflicts.

**Result: PASS.**

### F-ST-04 — task/Analysis ambiguity

`has_analysis` requires exactly one Analysis per task. Zero or multiple Analysis bindings are cardinality conflicts.

**Result: PASS.**

### F-ST-05 — duplicate authoritative `analyzed_by`

The proposal defines:

```text
M analyzed_by A
IFF
exists T: T uses_model M AND T has_analysis A
```

Thus `analyzed_by` is derived and cannot independently contradict task bindings. Multiple tasks with the same `(M,A)` pair yield one set-valued derived relation edge.

**Result: PASS.**

### F-ST-06 — Simulation/task model mismatch

```text
S has_model M1
S has_task T
T uses_model M2
```

with `M1 != M2` is an explicit Simulation consistency conflict.

**Result: PASS.**

### F-ST-07 — SolverConfiguration role

No inheritance between SolverConfiguration and SimulationTask is introduced. Existing `Analysis -> solved_by -> SolverConfiguration` remains orthogonal.

**Result: PASS.**

### F-ST-08 — backend realization differences

MOOSE may serialize separate model+Executioner artifacts per task; COMSOL may lower several task contexts to Studies in one model; Ansys may use multiple analysis systems sharing model data. None changes the Core task relation contract.

**Result: PASS.**

## 4. New ambiguity search

### Task shared by multiple Simulation contexts

Because `has_task` is explicitly non-owning, the same task may be referenced by multiple Simulation contexts. The invariant requires every such Simulation to select `T.uses_model` as its `has_model` target. This yields one deterministic result and does not require inverse ownership semantics.

**No defect.**

### Analysis with SolverConfiguration reused across models

An Analysis reused across tasks also reuses its explicitly associated SolverConfiguration under the current contract. If future evidence requires task-specific solver overrides, that is a separate solver-configuration relation decision. It does not make the current model/task binding ambiguous.

**Deferred, not a defect in current scope.**

### Multi-step backend studies

A backend Study containing multiple native steps can lower from several SOL task identities or a later workflow construct. v0.1 does not need to identify one backend Study object with one Core SimulationTask.

**Backend transformation, not a Core defect.**

## 5. Scope and ownership

No new:

- backend vocabulary;
- Adapter runtime state;
- Result subtype;
- MappingPlan contract;
- QRC rule;
- multi-model Simulation contract;
- solver override mechanism

is introduced.

The relation set is Core semantic structure and is therefore correctly owned by the SOL language/architecture layer.

## 6. Final verdict

| Item | Verdict |
|---|---|
| `SimulationTask` reification | **Accept** |
| `uses_model` exactly 1 | **Accept** |
| `has_analysis` exactly 1 | **Accept** |
| Analysis reuse across tasks/models | **Accept** |
| derived `analyzed_by` | **Accept** |
| Simulation task-model consistency invariant | **Accept** |
| SolverConfiguration separation | **Accept** |
| backend independence | **Accept** |
| focused revision required | **No** |

**Final verdict: ACCEPT.**

The v0.2 proposal is ready for a focused ADR and machine-readable transcription. The frozen baseline need only be amended for these explicit top-level/task relation semantics; ADR-0010/0011/0012 remain unaffected.
