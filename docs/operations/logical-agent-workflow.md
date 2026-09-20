# SOL Logical Agent Workflow

**Status:** Project operating convention  
**Date:** 2026-09-20  
**Scope:** Session initialization, meeting orchestration, planning, semantic decision-making, validation, implementation, and documentation synchronization

## Purpose

This document defines the logical agents used to operate the SOL project. These are project reasoning/execution roles, not runtime SOL agents or product concepts.

The canonical agent names are:

- `Manager`
- `Planner`
- `Researcher`
- `Validator`
- `Operator`

The role boundary is intentional: Manager orchestrates discussion, Planner structures work, Researcher owns semantic investigation, Validator challenges decisions, and Operator executes accepted work.

GitHub Milestones are the repository execution projection of accepted roadmap milestones. Their naming, membership, progress, closure, and backfill convention is defined in `docs/operations/github-milestone-convention.md`; GitHub progress never replaces parent-tracker or Validator acceptance authority.

## 0. Project session bootstrap

`init` is the project-level entry mode.

Generic initialization mechanics are owned by the exact pinned Paul `session-bootstrap` skill. This repository supplies SOL-specific authority sources, roles, current-state locators, and next-mode interpretation through `project-session-init.md`.

```text
init
  -> exact Paul binding
  -> Paul essential rules
  -> session-bootstrap + state-refresh
  -> SOL-local roles / authority / current GitHub evidence
  -> first real gate
  -> meeting | resume | update recommendation
  -> stop read-only
```

Paul essential rules govern pinning, memory/current-evidence boundaries, evidence-class boundaries, real gates, and interruption recovery. Do not restate those generic rules here.

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
- track unresolved questions and rejected/deferred alternatives;
- prevent premature convergence when architecture/public-contract/protocol consequences remain unclear;
- request user confirmation before important compatibility-sensitive decisions become accepted unless that authority has already been explicitly delegated;
- reopen a meeting when Operator reports a newly discovered semantic/compatibility contradiction during `resume` or `update`;
- hand accepted decisions to Operator for implementation or documentation synchronization.

Manager SHOULD make the decision state explicit using the following vocabulary where useful:

```text
PROPOSED
   -> CHALLENGED
   -> REVISED
   -> VALIDATED
        |\
        | -> USER_APPROVED  # when required
        |       |
        +-------+
                v
             ACCEPTED
                |
                v
      HANDOFF_TO_OPERATOR
```

`USER_APPROVED` is conditional rather than mandatory for every meeting. It is required for compatibility-sensitive or otherwise important decisions unless the user has already delegated acceptance authority. A low-risk or delegated decision may move from `VALIDATED` directly to `ACCEPTED`.

Other terminal states may include:

```text
REJECTED
DEFERRED
```

Manager must not convert a proposal into semantic truth merely because discussion appears converged. If Planner, Researcher, and Validator still disagree on a material semantic or compatibility question, Manager escalates the unresolved choice to the user instead of deciding the technical substance itself.

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

- inspect the current GitHub Milestone, parent tracker, Phase/blocker issues, PRs, and CI state before acting;
- execute Phase issues in dependency order;
- create/update branches, commits, PRs, issues, fixtures, tests, code, and documentation;
- apply the smallest root-cause fix when CI fails;
- record executable evidence before completing checklist items;
- merge a pre-authorized Phase PR when all bounded auto-merge conditions below are satisfied;
- verify the merge on `main`, close/update the completed Phase issue, update the parent tracker, and continue to the next eligible Phase without waiting for another user turn;
- keep GitHub Milestone membership/progress synchronized with accepted Phase structure without treating percentage as acceptance authority;
- escalate newly discovered unresolved semantic/compatibility decisions to Manager rather than resolving them implicitly during execution.

Operator has two named work modes: `resume` and `update`.

### 5.1 `resume`

`resume` means continue implementation/execution from the current repository state until a real gate is reached. A CI-green merge of an already accepted milestone Phase is not by itself a stop gate.

Expected behavior:

1. Identify the current roadmap/GitHub Milestone and inspect its parent tracker plus open Phase/milestone-blocker issues before selecting work.
2. Inspect current PR/issue/CI state rather than assuming the previous state is still current.
3. If an existing Phase PR is open, verify its head SHA, CI state, mergeability, unresolved review/permission state, and whether its work remains inside the already accepted Phase scope.
4. Continue the next eligible Phase/task when the current gate is clear. A planned successor GitHub Milestone does not bypass predecessor dependencies merely because it already exists.
5. Use counterexample-driven implementation and the established CI-first loop.
6. On CI failure, inspect logs, apply the smallest root-cause fix, and restart verification.
7. On CI success, record evidence and update issue checklists before merge.
8. When bounded auto-merge conditions are satisfied, merge the Phase PR, verify the merge on `main`, close/update the Phase issue and parent tracker, keep milestone progress metadata synchronized, then continue immediately to the next eligible Phase.
9. If implementation exposes a new unresolved architecture/Public Contract/Protocol/compatibility choice, stop that decision path and hand it to Manager `meeting`.
10. Stop only at a real decision, Validator rejection/revision gate, milestone exit audit, permission/branch-protection or merge-conflict gate, explicit user stop request, or other architecture/compatibility gate.

The normal inspection order is:

```text
current GitHub Milestone
  -> parent tracker
  -> milestone Phase / blocker issues
  -> current eligible Phase
  -> current PR / exact-head CI
```

#### Bounded auto-merge authorization

The user has delegated merge authority to Operator for already accepted milestone Phase implementation. Operator MAY merge without a separate per-PR user confirmation only when **all** of the following are true:

- the Phase/milestone scope and acceptance criteria were already accepted before implementation;
- the PR does not introduce a newly unresolved semantic, architecture, Public Contract, Adapter Protocol, roadmap, or compatibility decision;
- required fixtures/tests and issue evidence are complete;
- the required CI sequence is green on the exact current PR head;
- the PR is mergeable with no unresolved merge conflict, blocking review, branch-protection, or permission problem;
- the merge uses the exact verified head SHA when the merge API supports that guard;
- the Phase is not itself a milestone exit-audit/closure decision that requires Validator or Manager/user review.

After such a merge, Operator MUST verify the resulting `main` state before closing the Phase issue or starting work that depends on the merge.

Bounded auto-merge does **not** authorize Operator to merge through a real decision gate. Operator must stop and escalate instead when any of these occur:

- a new semantic/architecture/Public Contract/Adapter Protocol choice is required;
- compatibility impact is unclear or potentially breaking beyond the accepted Phase decision;
- Validator returns `REJECT/REVISE` or an unmet execution gate;
- a milestone exit audit is due;
- required CI is not green on the exact head;
- merge conflict, branch protection, review, or permission blocks safe merge;
- the user explicitly requests a stop or per-PR review.

The intent is:

```text
resume
  -> inspect current milestone / parent / Phase
  -> implement accepted Phase N
  -> PR
  -> CI/fix loop
  -> evidence
  -> bounded auto-merge when eligible
  -> verify main
  -> close Phase N / update parent / synchronize milestone progress
  -> continue Phase N+1
  -> stop only at a real gate
```

The normal CI polling loop is conceptually:

```text
push / PR update
   -> detect workflow run
   -> wait/check in ~30-second intervals
   -> failure: inspect -> minimal fix -> restart
   -> success: record evidence -> merge/next task when authorized
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
- roadmap/status sections that have become stale;
- project operating conventions when an accepted workflow decision changes repository operation.

Expected behavior:

1. Read the current implementation, accepted ADRs, governing plans, active/completed issues, GitHub Milestone state, and relevant public contracts.
2. Identify stale, incomplete, contradictory, or missing guide/operations content.
3. Update documentation to describe accepted behavior and current capability accurately.
4. Prefer links to normative ADR/contract/policy documents rather than duplicating large normative definitions.
5. Update examples and command/output snippets when supported behavior changes.
6. Distinguish implemented capability from planned capability clearly.
7. Run applicable documentation/build/CI checks and record evidence in the PR.
8. If documentation exposes an unresolved semantic or compatibility contradiction, stop that decision path and escalate it to Manager `meeting`.

`update` is documentation synchronization, not semantic authority. Operator should not bypass Manager by silently choosing among unresolved Planner/Researcher/Validator alternatives in guide text.

For milestone handoff, `update` normally runs after the GitHub Milestone has been accepted and closed. Its PR is not assigned back to the closed milestone and does not reopen completed progress merely to record documentation synchronization.

A typical `update` scope is:

```text
accepted repository state
       |
       v
README / API docs / guides / examples / operations
       |
       v
consistency review
       |
       +--> unresolved semantic/compatibility question -> Manager: meeting
       |
       v
PR + CI evidence
```

## 6. Normal agent flow

When a session is new or its context may be stale, the normal entry flow is:

```text
init
  -> resolve consumer-pinned chatgpt-operation revision
  -> load minimal init skill working set
  -> canonical role/work-mode reload
  -> current GitHub state and real-gate snapshot
  -> explicit meeting | resume | update selection
  -> load selected-mode skills before execution/mutation
```

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
      +--> unresolved material disagreement -> user escalation
      |
      +--> user confirmation when required
      |
      v
ACCEPTED
      |
      v
Operator: resume
      |
      v
milestone -> Phase -> fixture/test -> implementation -> CI -> evidence -> bounded merge -> next eligible Phase
```

Documentation synchronization then uses:

```text
accepted repository state
      |
      v
Operator: update
      |
      v
README / API docs / guides / operations synchronized to accepted state
```

Manager, Planner, and Researcher may be skipped when a task is purely mechanical and already fully specified. Validator should be used whenever semantics, compatibility, architecture boundaries, or public contracts can be affected.

## 7. When `meeting` is required

Manager `meeting` SHOULD be used for:

- architecture decisions;
- Public Contract changes;
- Adapter Protocol changes;
- milestone/roadmap redesign;
- GitHub Milestone convention changes that alter membership/progress/closure/acceptance semantics;
- decisions with multiple reasonable alternatives;
- changes that may affect compatibility;
- conflicts between Researcher and Validator recommendations;
- unresolved semantic/compatibility contradictions discovered by Operator during `resume` or `update`.

Manager `meeting` MAY be skipped for:

- implementation of an already accepted issue;
- CI/root-cause fixes;
- formatting/mechanical maintenance;
- straightforward refactors that preserve accepted contracts;
- mechanical GitHub Milestone assignment/backfill under the accepted convention;
- guide synchronization through Operator `update` when no unresolved decision is exposed.

## 8. Milestone handoff rule

For compatibility-sensitive milestones, the preferred handoff is:

```text
implementation complete
        -> exact-head CI green
        -> Validator exit audit
        -> final Phase merge
        -> verify main
        -> close final Phase issue
        -> complete/close parent tracker
        -> verify no required GitHub Milestone work remains open
        -> close GitHub Milestone
        -> Operator update
        -> Manager meeting / Planner preparation of the next milestone when new decisions are required
```

Bounded auto-merge applies to accepted Phase implementation PRs inside the milestone, but it does not bypass the Validator exit audit or other milestone closure gates.

GitHub Milestone percentage is informational. The canonical progress units are Phase issues plus explicit milestone-exit corrective blockers; the parent tracker remains outside milestone membership and retains normative gate/evidence responsibility. PRs are implementation evidence rather than duplicate progress units.

This makes milestone closure mean that validation and repository verification have already passed, while `update` serves as the documentation handoff to the next milestone.

Historical backfill, cross-milestone blocker assignment, due-date rules, and closed-milestone reopening policy are defined in `docs/operations/github-milestone-convention.md`.

## 9. Documentation hierarchy rule

Documentation has different authority levels.

- ADRs define accepted semantic architecture decisions.
- Public-contract/protocol/schema documents define normative external contracts when adopted.
- Product/versioning plans define roadmap and compatibility governance.
- `docs/operations/` defines project operating conventions, including logical-agent and GitHub Milestone workflow.
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
