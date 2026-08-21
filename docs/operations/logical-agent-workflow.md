# SOL Logical Agent Workflow

**Status:** Project operating convention  
**Date:** 2026-08-21  
**Scope:** Meeting orchestration, planning, semantic decision-making, validation, implementation, and documentation synchronization

## Purpose

This document defines the logical agents used to operate the SOL project. These are project reasoning/execution roles, not runtime SOL agents or product concepts.

The canonical agent names are:

- `Manager`
- `Planner`
- `Researcher`
- `Validator`
- `Operator`

The role boundary is intentional: Manager orchestrates discussion, Planner structures work, Researcher owns semantic investigation, Validator challenges decisions, and Operator executes accepted work.

## 1. Manager

Manager owns discussion orchestration and decision-state management. Manager does not own semantic truth, implementation state, or validation authority.

Manager has one named work mode: `meeting`.

### 1.1 `meeting`

`meeting` means coordinate a structured decision process across Planner, Researcher, Validator, and the user until the proposal is either accepted, rejected, deferred, or returned for revision.

Responsibilities:

- establish the agenda and decision question;
- ask Planner to frame objectives, alternatives, sequencing, and constraints;
- ask Researcher to resolve semantic/architecture questions and identify ADR candidates;
- ask Validator to challenge the proposal using connectivity, extensibility, simplicity, consistency, compatibility, and counterexamples;
- return Validator findings to Planner for critical revision rather than automatic acceptance;
- track unresolved questions and rejected alternatives;
- prevent premature convergence when architecture/public-contract/protocol consequences remain unclear;
- request user confirmation before important architecture/public-contract/protocol decisions become accepted;
- hand accepted decisions to Operator for implementation or documentation synchronization.

Manager SHOULD make the decision state explicit using the following vocabulary where useful:

```text
PROPOSED
   -> CHALLENGED
   -> REVISED
   -> VALIDATED
   -> USER_APPROVED
   -> ACCEPTED
   -> HANDOFF_TO_OPERATOR
```

Other terminal states may include:

```text
REJECTED
DEFERRED
```

Manager must not convert a proposal into semantic truth merely because the discussion appears converged. For architecture, Public Contract, Adapter Protocol, or other compatibility-sensitive decisions, user confirmation is part of acceptance unless the user has already explicitly delegated that authority.

A successful meeting should leave four artifacts clear:

1. the accepted decision;
2. rejected/deferred alternatives and the reason;
3. invariants, counterexamples, or acceptance gates;
4. the execution handoff: ADR, issue, fixture/test, implementation, or documentation update.

Manager is optional for purely mechanical work that is already fully specified.

## 2. Planner

Planner prepares the decision space before architecture or implementation work begins.

Responsibilities:

- clarify objectives, constraints, sequencing, and dependencies;
- compare alternative approaches and trade-offs;
- identify which questions require Researcher decisions;
- identify which changes require ADR treatment;
- shape milestones, phases, and acceptance criteria before execution;
- keep roadmap sequencing consistent with product/versioning policy;
- critically evaluate Validator recommendations instead of accepting them automatically.

Planner does not own implementation state and does not silently establish new semantic truth.

Primary outputs:

- plans;
- milestone/phase decomposition;
- option comparisons;
- decision criteria;
- dependency/order recommendations.

## 3. Researcher

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

## 4. Validator

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

Validator findings inform Planner revision; they are not automatically the final project decision.

Primary outputs:

- validator verdicts;
- counterexamples;
- compatibility cases;
- acceptance gates;
- missing-evidence findings.

## 5. Operator

Operator owns execution against the accepted plan and repository state.

Responsibilities:

- inspect current GitHub/CI state before acting;
- execute Phase issues in dependency order;
- create/update branches, commits, PRs, issues, fixtures, tests, code, and documentation;
- apply the smallest root-cause fix when CI fails;
- record executable evidence before completing checklist items;
- preserve the user-controlled merge gate unless explicit merge authorization is given.

Operator has two named work modes: `resume` and `update`.

### 5.1 `resume`

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

### 5.2 `update`

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

`update` is not authority to invent new semantics. If documentation exposes an unresolved semantic contradiction, Operator should surface it to Manager/Planner/Researcher rather than silently resolve it in guide text.

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

## 6. Normal agent flow

For architecture, Public Contract, Adapter Protocol, roadmap, or other decision-heavy work, the normal flow is:

```text
Manager: meeting
      |
      v
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
Planner critical revision
      |
      v
Manager decision-state check
      |
      v
User confirmation when required
      |
      v
ACCEPTED
      |
      v
Operator: resume
      |
      v
fixture/test -> implementation -> CI -> issue evidence
```

Documentation synchronization then uses:

```text
accepted repository state
      |
      v
Operator: update
      |
      v
README / API docs / guides synchronized to accepted state
```

Manager, Planner, and Researcher may be skipped when a task is purely mechanical and already fully specified. Validator should be used whenever semantics, compatibility, architecture boundaries, or public contracts can be affected.

## 7. When `meeting` is required

Manager `meeting` SHOULD be used for:

- architecture decisions;
- Public Contract changes;
- Adapter Protocol changes;
- milestone/roadmap redesign;
- decisions with multiple reasonable alternatives;
- changes that may affect compatibility;
- conflicts between Researcher and Validator recommendations.

Manager `meeting` MAY be skipped for:

- implementation of an already accepted issue;
- CI/root-cause fixes;
- formatting/mechanical maintenance;
- straightforward refactors that preserve accepted contracts;
- guide synchronization through Operator `update`.

## 8. Milestone handoff rule

For compatibility-sensitive milestones, the preferred handoff is:

```text
implementation complete
        -> Validator exit audit
        -> Operator resume for required fixes
        -> milestone close
        -> Operator update
        -> Manager/Planner preparation of the next milestone
```

This makes milestone closure mean that validation has already passed, while `update` serves as the documentation handoff to the next milestone.

## 9. Documentation hierarchy rule

Documentation has different authority levels.

- ADRs define accepted semantic architecture decisions.
- Public-contract/protocol/schema documents define normative external contracts when adopted.
- Product/versioning plans define roadmap and compatibility governance.
- `docs/operations/` defines project operating conventions.
- README/API guides/examples explain how to understand and use the accepted system.

Guide documentation must follow normative sources; it must not silently override them.

## 10. Naming migration

Previous informal names are superseded as follows:

```text
Consult                          -> Planner
Research Lab                     -> Researcher
Validation Lab / Validator       -> Validator
Operating / Implementation Agent -> Operator
```

`Manager` is a new orchestration role introduced to own `meeting`; it does not replace a previous agent.

New project documentation should use the canonical names above.