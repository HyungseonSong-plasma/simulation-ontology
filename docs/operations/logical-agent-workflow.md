# SOL Logical Agent Workflow

**Status:** Project operating convention  
**Date:** 2026-08-21  
**Scope:** Planning, semantic decision-making, validation, implementation, and documentation synchronization

## Purpose

This document defines the logical agents used to operate the SOL project. These are project reasoning/execution roles, not runtime SOL agents or product concepts.

The canonical agent names are:

- `Planner`
- `Researcher`
- `Validator`
- `Operator`

## 1. Planner

Planner prepares the decision space before architecture or implementation work begins.

Responsibilities:

- clarify objectives, constraints, sequencing, and dependencies;
- compare alternative approaches and trade-offs;
- identify which questions require Researcher decisions;
- identify which changes require ADR treatment;
- shape milestones, phases, and acceptance criteria before execution;
- keep roadmap sequencing consistent with product/versioning policy.

Planner does not own implementation state and does not silently establish new semantic truth.

Primary outputs:

- plans;
- milestone/phase decomposition;
- option comparisons;
- decision criteria;
- dependency/order recommendations.

## 2. Researcher

Researcher owns semantic and architecture investigation.

Responsibilities:

- define ontology and architecture boundaries;
- resolve semantic ambiguity;
- investigate alternatives and implications;
- preserve agreed Core/backend separation and versioning invariants;
- produce an ADR when a decision changes or fixes semantic architecture.

Researcher should not treat implementation convenience as sufficient justification for changing canonical semantics.

Primary outputs:

- semantic decision proposals;
- architecture decisions;
- ADRs;
- clarified invariants and non-goals.

## 3. Validator

Validator challenges proposed decisions and implementation boundaries before they are treated as stable.

Evaluation dimensions include:

- semantic connectivity;
- extensibility;
- simplicity;
- consistency;
- compatibility;
- backend leakage;
- deterministic behavior;
- counterexample coverage.

Responsibilities:

- test Researcher/Planner decisions against positive and negative cases;
- construct executable counterexamples where possible;
- distinguish structural validity from semantic validity;
- detect hidden coupling and compatibility breaks;
- issue an explicit verdict such as `APPROVE`, `APPROVE WITH GATES`, or `REJECT/REVISE`;
- define acceptance gates that Operator can execute.

Primary outputs:

- validator verdicts;
- counterexamples;
- compatibility cases;
- acceptance gates;
- missing-evidence findings.

## 4. Operator

Operator owns execution against the approved plan and repository state.

Responsibilities:

- inspect current GitHub/CI state before acting;
- execute Phase issues in dependency order;
- create/update branches, commits, PRs, issues, fixtures, tests, code, and documentation;
- apply the smallest root-cause fix when CI fails;
- record executable evidence before completing checklist items;
- preserve the user-controlled merge gate unless explicit merge authorization is given.

Operator has two named work modes: `resume` and `update`.

### 4.1 `resume`

`resume` means continue implementation/execution from the current repository state until a real gate is reached.

Expected behavior:

1. Inspect current PR/issue/CI state rather than assuming the previous state is still current.
2. If a merge gate exists, verify whether the user merged the PR.
3. Continue the next eligible Phase/task when the gate is clear.
4. Use counterexample-driven implementation and the established CI-first loop.
5. On CI failure, inspect logs, apply the smallest root-cause fix, and restart verification.
6. On CI success, record evidence, update issue checklists, and continue immediately to the next eligible task.
7. Stop only at a real decision, merge, permission, or architecture gate.

The normal CI polling loop is conceptually:

```text
push / PR update
   -> detect workflow run
   -> wait/check in ~30-second intervals
   -> failure: inspect -> minimal fix -> restart
   -> success: record evidence -> next task
```

### 4.2 `update`

`update` means synchronize user-facing and developer-facing guide documentation with the accepted current project state.

Primary targets include:

- root `README.md`;
- API documentation;
- SDK guides;
- adapter authoring guides;
- CLI usage guides;
- schema/public-contract guides;
- architecture overview documents;
- examples and onboarding documentation;
- roadmap/status sections that have become stale.

Expected behavior:

1. Read the current implementation, accepted ADRs, governing plans, active/completed issues, and relevant public contracts.
2. Identify stale, incomplete, contradictory, or missing guide content.
3. Update documentation to describe accepted behavior and current capability accurately.
4. Prefer links to normative ADR/contract/policy documents rather than duplicating large normative definitions.
5. Update examples and command/output snippets when supported behavior changes.
6. Distinguish implemented capability from planned capability clearly.
7. Run applicable documentation/build/CI checks and record evidence in the PR.

`update` is not authority to invent new semantics. If documentation exposes an unresolved semantic contradiction, Operator should surface it to Planner/Researcher rather than silently resolve it in guide text.

A typical `update` scope is:

```text
accepted repository state
       |
       v
README / API docs / guides / examples
       |
       v
consistency review
       |
       v
PR + CI evidence
```

## 5. Normal agent flow

The normal project flow is:

```text
Planner
   |
   v
Researcher
   |
   +--> ADR when semantic architecture is fixed/changed
   |
   v
Validator
   |
   v
counterexample / acceptance gate
   |
   v
Operator: resume
   |
   v
fixture/test -> implementation -> CI -> issue evidence

Operator: update
   |
   v
README / API docs / guides synchronized to accepted state
```

Planner and Researcher may be skipped when a task is purely mechanical and already fully specified. Validator should be used whenever semantics, compatibility, architecture boundaries, or public contracts can be affected.

## 6. Documentation hierarchy rule

Documentation has different authority levels.

- ADRs define accepted semantic architecture decisions.
- Public-contract/protocol/schema documents define normative external contracts when adopted.
- Product/versioning plans define roadmap and compatibility governance.
- README/API guides/examples explain how to understand and use the accepted system.

Guide documentation must follow normative sources; it must not silently override them.

## 7. Naming migration

Previous informal names are superseded as follows:

```text
Consult                         -> Planner
Research Lab                    -> Researcher
Validation Lab / Validator      -> Validator
Operating / Implementation Agent -> Operator
```

New project documentation should use the canonical names above.