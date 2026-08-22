# GitHub Milestone Convention

**Status:** Project operating convention  
**Date:** 2026-08-22  
**Scope:** GitHub Milestone naming, creation, membership, progress interpretation, closure, and Operator integration

## Purpose

GitHub Milestones are the repository-level execution projection of the accepted SOL roadmap. They are used to show milestone scope and progress in GitHub without redefining SOL semantic/version authority.

A GitHub Milestone is not a SOL Runtime release, Public Contract version, Adapter Protocol version, ontology package version, or adapter implementation version.

## 1. Canonical structure

The accepted repository structure is:

```text
GitHub Milestone
  = M0.x roadmap execution container

Parent issue
  = normative milestone specification / architecture and acceptance tracker

Phase issue
  = canonical milestone progress unit

Milestone-blocking corrective issue
  = optional progress unit only when required for milestone exit

PR
  = implementation evidence, not a progress unit

CI
  = executable evidence

Validator exit audit
  = milestone acceptance authority

Operator update
  = documentation synchronization with accepted repository state
```

The parent issue is deliberately not assigned to its GitHub Milestone. Phase PRs are also deliberately not assigned when a corresponding Phase issue already exists.

## 2. Naming

Milestone titles use:

```text
M<roadmap-number> — <Canonical Milestone Name>
```

Examples:

```text
M0.1 — Semantic Core Bootstrap
M0.2 — Canonical Public Contract 0.1
M0.3 — Adapter Protocol 0.1
M0.4 — MockAdapter Protocol Conformance
M0.5 — JSON-RPC / stdio Transport
M0.6 — Adapter Conformance Tooling
M0.7 — Adapter Runtime & Registry
```

Do not use `v0.x` for roadmap milestones because `M0.x` is not a release/version axis.

## 3. Creation rule

Create a GitHub Milestone when both are true:

1. the roadmap milestone objective has been accepted; and
2. a parent milestone tracker issue exists.

A future roadmap name without an accepted objective or parent tracker does not require a GitHub Milestone yet.

Planned milestones may be created before implementation eligibility when their objective and decomposition are accepted. Predecessor dependencies still govern execution order.

## 4. Milestone description

Keep the GitHub Milestone description concise. It should identify the roadmap meaning and link the repository trackers rather than duplicate the parent issue.

Recommended shape:

```text
Roadmap milestone identifier.
This is not a SOL Runtime, Public Contract, Adapter Protocol,
ontology package, or adapter implementation version.

Parent tracker: #<parent>
Exit gate: #<final-phase>

Objective:
<short accepted objective>

Canonical progress units:
Phase issues #<start>–#<end>.

Out of scope:
- <major exclusion>
```

Detailed invariants and acceptance criteria remain in the parent issue and governing repository documents.

## 5. Membership rule

Canonical progress units are Phase issues.

Assign to the GitHub Milestone:

- each Phase issue belonging to that M0.x milestone;
- a corrective issue only when it independently blocks milestone exit and cannot be treated as an ordinary child task inside an existing Phase.

Do not assign by default:

- the parent tracker issue;
- Phase implementation PRs;
- ordinary child/subtask issues whose completion is already represented by an open Phase issue;
- post-milestone Operator `update` PRs.

This keeps the GitHub progress denominator aligned with roadmap Phase completion rather than commit/PR count or task granularity.

## 6. Corrective blockers

An unexpected issue may be added to a milestone when all of the following are true:

- it is required to complete the milestone safely;
- it is not merely implementation detail already owned by an existing Phase issue;
- milestone closure is blocked until it is resolved.

Such an issue becomes a canonical progress unit for that milestone.

## 7. Cross-milestone issues

One issue should not be duplicated across milestones merely because it affects later work.

If an issue blocks multiple milestones, assign it to the earliest milestone that cannot close without it. Later milestones reference it as a prerequisite.

## 8. Due dates

A GitHub Milestone due date is optional.

Set a due date only when there is a real external or organizational commitment, such as a release commitment, conference/demo date, customer/internal delivery, or compatibility freeze.

Do not invent due dates from an informal desire to finish quickly. Operator does not create or change due dates without an accepted planning decision.

## 9. Progress semantics

GitHub Milestone progress percentage is informational, not acceptance authority.

A milestone showing 100% closed progress is not automatically accepted if the parent tracker or Validator exit gate is incomplete.

Authority order is:

```text
GitHub progress                 informational
        |
        v
Parent milestone tracker        normative gates
        |
        v
Validator exit audit            milestone acceptance
        |
        v
main verification               executable repository evidence
```

## 10. Closure rule

Preferred closure sequence:

```text
final Phase implementation
  -> exact-head CI green
  -> Validator milestone exit audit
  -> final Phase merge
  -> verify main
  -> close final Phase issue
  -> update parent tracker: all phases/gates complete
  -> close parent tracker
  -> verify no required milestone work remains open
  -> close GitHub Milestone
  -> Operator update
```

GitHub Milestone closure is therefore a repository-management completion step, not a substitute for the Validator or parent tracker.

A milestone exit audit remains a real gate and is not covered by bounded auto-merge authorization.

## 11. Operator `resume`

`resume` begins by identifying the current roadmap/GitHub Milestone before selecting the next Phase.

Expected inspection order:

```text
current GitHub Milestone
  -> parent tracker
  -> milestone Phase/blocker issues
  -> current eligible Phase
  -> current PR / exact-head CI
```

Operator then executes the accepted Phase through the existing CI-first/bounded-merge workflow. When one Phase closes, Operator continues to the next eligible Phase in the same milestone until a real gate is reached.

A planned successor milestone does not become implementation-eligible merely because its GitHub Milestone already exists. Accepted dependency order remains authoritative.

If repository tooling cannot create or mutate an intended GitHub Milestone container, Operator uses the accepted parent tracker and Phase issues as the authoritative execution projection while reporting the administrative milestone action as pending. Tooling limitations do not change milestone semantics.

## 12. Operator `update`

`update` synchronizes user/developer-facing documentation with accepted repository state. It does not promote unmerged PR state into accepted implementation.

A milestone-handoff `update` normally runs after milestone closure, but an in-progress synchronization update may also be used when durable repository metadata has materially changed—for example after accepted Phase merges or manual GitHub Milestone administration.

The update PR is not assigned to the active or closed roadmap milestone because documentation synchronization is evidence/handoff work rather than a canonical Phase progress unit.

## 13. Historical backfill

Historical milestones may be backfilled so GitHub presents a coherent roadmap history.

For historical backfill:

- create the M0.x GitHub Milestone using the canonical title;
- assign completed Phase issues only;
- do not assign the historical parent issue or PRs merely to reconstruct history;
- verify the progress becomes 100%;
- close the historical GitHub Milestone.

Historical backfill does not change the original semantic or acceptance evidence.

## 14. Reopening a closed milestone

Do not reopen a historical milestone merely because a later bug or compatibility issue is discovered.

Normally create/assign the corrective work to the current or next appropriate milestone. Reopening is reserved for evidence that the original milestone completion itself was invalid—for example, required exit evidence was materially wrong or an accepted closure condition was never actually satisfied.

## 15. Current roadmap projection

Current repository projection after M0.7 Phase 0–1 acceptance and Milestone #7 administration:

```text
M0.1 — Semantic Core Bootstrap            historical / complete
M0.2 — Canonical Public Contract 0.1      historical / complete
M0.3 — Adapter Protocol 0.1               historical / complete
M0.4 — MockAdapter Protocol Conformance   historical / complete
M0.5 — JSON-RPC / stdio Transport         historical / complete
M0.6 — Adapter Conformance Tooling        accepted complete; Milestone #6 UI close pending
M0.7 — Adapter Runtime & Registry         ACTIVE; Milestone #7; 2/6 Phase issues complete
```

Canonical issue groupings are:

```text
M0.1: parent #2,   progress units #6–#16
M0.2: parent #28,  progress units #29–#35
M0.3: parent #38,  progress units #39–#44
M0.4: parent #65,  progress units #66–#71
M0.5: parent #74,  progress units #75–#80
M0.6: parent #81,  progress units #82–#87
M0.7: parent #111, progress units #112–#117, GitHub Milestone #7
```

For M0.6, Phase #82–#87 and parent #81 are closed completed, exact-head CI and Validator exit acceptance are complete, and the final merge is verified on `main`. GitHub Milestone #6 reports `0` open and `6` closed Phase issues; the available GitHub connector does not expose a milestone-state write action, so its final UI **Close milestone** action remains administrative.

For M0.7, GitHub Milestone #7 exists with canonical title `M0.7 — Adapter Runtime & Registry`. Phase issues #112–#117 are assigned to it; #112 and #113 are closed completed, while #114–#117 are open. Parent #111 remains outside milestone membership by convention. The current GitHub progress is therefore 2/6 Phase issues complete and is informational only.

## 16. Change control

This is an operating convention, not a semantic ADR. Changes that alter roadmap structure, compatibility authority, Public Contract/Adapter Protocol meaning, or milestone acceptance semantics require Manager `meeting` and normal Planner/Researcher/Validator handling as applicable.
