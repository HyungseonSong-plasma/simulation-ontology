# SOL Project Session Initialization

**Status:** Project operating convention  
**Date:** 2026-08-21  
**Scope:** Deterministic bootstrap of logical roles and current repository state in a new or uncertain chat/session

## Purpose

Conversation history and model memory are useful hints, but they are not authoritative project state. A new chat may not retain the logical Agent roles, accepted decisions, current milestone, open pull request, CI result, or real execution gate.

The project-level work mode `init` restores that context from canonical repository documents and current GitHub evidence before any role-specific work begins.

`init` is a project-session bootstrap, not a runtime SOL concept and not a substitute for `meeting`, `resume`, or `update`.

## Trigger

Run `init` when:

- the user sends `init`;
- a new chat begins and project execution state is needed;
- conversational context was compacted, lost, or may be stale;
- the repository or active milestone is uncertain;
- the user asks to restore, reload, or verify the project operating context.

Re-running `init` is safe because it is read-only.

## Canonical entry sources

Read these entry sources in order:

1. root `AGENTS.md`;
2. `docs/operations/project-session-init.md` (this document);
3. `docs/operations/logical-agent-workflow.md`;
4. `docs/operations/github-milestone-convention.md`;
5. current GitHub repository evidence;
6. only the governing ADRs, contracts, plans, and implementation records relevant to the current gate.

Do not load the entire repository by default. Use progressive disclosure after the current milestone/Phase or decision topic is known.

## Bootstrap procedure

### 1. Resolve repository identity

Confirm:

- repository: `HyungseonSong-plasma/simulation-ontology`;
- default branch;
- latest default-branch commit SHA;
- whether the available GitHub connection can read the private repository.

Do not infer current state from an old local checkout or previous chat when fresher GitHub evidence is available.

### 2. Reload the logical operating model

Re-establish:

- `Manager` and `meeting`;
- `Planner`;
- `Researcher`;
- `Validator`;
- `Operator` and `resume` / `update`;
- authority hierarchy and real-gate rules;
- bounded auto-merge limits.

`init` itself remains project-level and read-only. It does not become a sixth logical Agent.

### 3. Inspect current execution state

Use current GitHub evidence to identify:

- current or most recently completed roadmap/GitHub Milestone;
- normative parent tracker;
- open eligible Phase or milestone-blocker issues and dependency order;
- open pull request, exact head SHA, CI/check state, mergeability, review threads, and permissions when applicable;
- latest verified `main` state;
- unresolved milestone closure or documentation-handoff requirements.

Issue bodies and guide documents may be stale. Cross-check them against issue state, PR state, commits, CI, and `main`.

### 4. Read the minimum relevant governing material

After identifying the current gate, read only the applicable:

- ADRs;
- Public Contract / Adapter Protocol documents and schemas;
- roadmap/versioning plans;
- implementation or exit-audit records;
- Phase acceptance criteria.

A guide or README may explain state, but it must not override a normative ADR, contract, schema, parent tracker, Validator verdict, or verified repository evidence.

### 5. Classify the next mode

Recommend exactly one next mode when evidence permits:

- `meeting`: a material architecture, ontology, Public Contract, Adapter Protocol, roadmap, compatibility, or unresolved decision gate exists;
- `resume`: accepted executable work is eligible and no real gate blocks it;
- `update`: accepted implementation state needs guide/documentation synchronization and its required predecessor closure is complete.

If evidence is insufficient, report the missing evidence instead of guessing.

### 6. Stop after the initialization report

`init` ends after reporting the restored context. It does not automatically invoke `meeting`, `resume`, or `update`. The user selects or confirms the next mode.

## Required output

Keep the initialization report concise but include:

1. **Repository snapshot** — repository, default branch, latest verified SHA.
2. **Role snapshot** — canonical roles and available modes.
3. **Execution snapshot** — milestone, parent tracker, current Phase/blocker, open PR/CI when present.
4. **Real gate** — the first unresolved decision, permission, milestone, CI, review, or dependency gate.
5. **Recommended next mode** — `meeting`, `resume`, or `update`, with one-line justification.
6. **Evidence uncertainty** — any state that could not be verified.

## Read-only boundary

During `init`, do not:

- create, update, close, assign, or comment on issues;
- create, update, review, merge, or close pull requests;
- create or update branches, commits, tags, releases, or milestones;
- edit repository or Library files;
- rerun CI;
- treat chat memory as acceptance evidence;
- make a semantic or compatibility decision;
- silently continue into another work mode.

Discovery, inspection, and reporting are allowed.

## Counterexamples

The following are invalid:

```text
new chat + remembered summary -> resume without GitHub inspection
README says Phase N -> assume Phase N is still current
Milestone says 100% -> assume Validator/parent/main gates passed
init -> automatically modify or merge repository state
init -> silently choose a new protocol or architecture meaning
init -> load every repository file before identifying the current gate
```

## Persistent ChatGPT Project entry hook

A repository document cannot guarantee that a completely new chat knows where the repository rules live. The ChatGPT Project instructions should therefore retain only this small stable entry hook:

```text
For the Simulation Ontology project, when the user sends "init", use GitHub to open
HyungseonSong-plasma/simulation-ontology and follow
docs/operations/project-session-init.md as a read-only bootstrap.
Do not rely on prior chat memory and do not mutate project state until the user
selects or confirms meeting, resume, or update.
```

Keep detailed and changeable workflow rules in this repository rather than duplicating them in Project instructions.

## Relationship to other modes

```text
init
  -> meeting  # decide
  -> resume   # execute accepted work
  -> update   # synchronize accepted documentation state
```

`meeting`, `resume`, and `update` retain the definitions and gates in `logical-agent-workflow.md`. `init` only restores enough authoritative context to choose among them safely.
