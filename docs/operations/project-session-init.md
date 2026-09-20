# SOL Project Session Initialization

**Status:** Project operating convention  
**Date:** 2026-09-20  
**Scope:** Deterministic bootstrap of pinned central operating skills, logical roles, and current repository state in a new or uncertain chat/session

## Purpose

Conversation history and model memory are useful hints, but they are not authoritative project state. A new chat may not retain the logical Agent roles, accepted decisions, current milestone, open pull request, CI result, or real execution gate.

The project-level work mode `init` restores that context from the consumer-pinned `chatgpt-operation` operating-system/skill source, canonical SOL repository documents, and current GitHub evidence before any role-specific work begins.

Central operating mechanics and SOL project/domain authority remain separate: `chatgpt-operation` owns reusable operating-system identity and portable mechanics, while this repository owns SOL semantics, repository-specific policy, roles, roadmap state, and acceptance criteria.

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

1. `docs/operations/chatgpt-operation-binding.json` from the current verified SOL revision;
2. the exact pinned `HyungseonSong-plasma/chatgpt-operation` revision named by that binding:
   - `docs/operating_system/README.md`;
   - central root `README.md` as the skill catalog;
   - the skill contracts listed for the `init` working set;
3. root `AGENTS.md`;
4. `docs/operations/project-session-init.md` (this document);
5. `docs/operations/logical-agent-workflow.md`;
6. `docs/operations/github-milestone-convention.md`;
7. current GitHub repository evidence;
8. only the governing ADRs, contracts, plans, and implementation records relevant to the current gate.

Do not follow central `main`, `latest`, or another floating ref once the binding has selected an exact revision. Do not load the entire central or consumer repository by default. Follow the central OS working-set principle and use progressive disclosure after the current milestone/Phase, mode, or decision topic is known.

## Bootstrap procedure

### 1. Resolve repository identity and consumer binding

Confirm:

- repository: `HyungseonSong-plasma/simulation-ontology`;
- default branch;
- latest default-branch commit SHA;
- whether the available GitHub connection can read the repository;
- `docs/operations/chatgpt-operation-binding.json` from that verified SOL revision.

The binding is consumer-owned operating metadata. It must identify an exact immutable central revision, not a floating branch/tag alias.

Do not infer current state from an old local checkout or previous chat when fresher GitHub evidence is available.

### 2. Initialize the central operating-skill working set

From the exact revision in `chatgpt-operation-binding.json`:

1. verify the central repository/revision is readable;
2. read `docs/operating_system/README.md` and verify the expected OS version when one is declared;
3. read the central skill catalog;
4. load only the skill contracts listed under the binding's `init` working set;
5. record the exact central revision and loaded skill names for the initialization report.

The current SOL binding initializes `state-refresh` during `init`. Other portable skills stay dormant until their declared mode/trigger becomes active. In particular, mutation skills do not grant mutation authority during `init`.

If the pinned central revision or required init skill cannot be verified, do not fall back to conversation memory, a different central revision, or floating `main`. Report the operating-source uncertainty as the real gate and stop read-only.

### 3. Reload the logical operating model

Re-establish:

- `Manager` and `meeting`;
- `Planner`;
- `Researcher`;
- `Validator`;
- `Operator` and `resume` / `update`;
- authority hierarchy and real-gate rules;
- bounded auto-merge limits.

`init` itself remains project-level and read-only. It does not become a sixth logical Agent.

### 4. Inspect current execution state

Use current GitHub evidence to identify:

- current or most recently completed roadmap/GitHub Milestone;
- normative parent tracker;
- open eligible Phase or milestone-blocker issues and dependency order;
- open pull request, exact head SHA, CI/check state, mergeability, review threads, and permissions when applicable;
- latest verified `main` state;
- unresolved milestone closure or documentation-handoff requirements.

Issue bodies and guide documents may be stale. Cross-check them against issue state, PR state, commits, CI, and `main`.

### 5. Read the minimum relevant governing material

After identifying the current gate, read only the applicable:

- ADRs;
- Public Contract / Adapter Protocol documents and schemas;
- roadmap/versioning plans;
- implementation or exit-audit records;
- Phase acceptance criteria.

A guide or README may explain state, but it must not override a normative ADR, contract, schema, parent tracker, Validator verdict, or verified repository evidence.

### 6. Classify the next mode

Recommend exactly one next mode when evidence permits:

- `meeting`: a material architecture, ontology, Public Contract, Adapter Protocol, roadmap, compatibility, or unresolved decision gate exists;
- `resume`: accepted executable work is eligible and no real gate blocks it;
- `update`: accepted implementation state needs guide/documentation synchronization and its required predecessor closure is complete.

If evidence is insufficient, report the missing evidence instead of guessing.

After the user selects or confirms the next mode, load the mode-specific skill contracts declared by the same pinned binding before any governed execution or mutation. Conditional skills are activated only when their trigger applies; central skill presence does not imply permanent activation.

### 7. Stop after the initialization report

`init` ends after reporting the restored context. It does not automatically invoke `meeting`, `resume`, or `update`. The user selects or confirms the next mode.

## Required output

Keep the initialization report concise but include:

1. **Operating-skill snapshot** — central repository, exact pinned revision, OS version, init skills loaded, and relevant deferred/mode skills.
2. **Repository snapshot** — repository, default branch, latest verified SHA.
3. **Role snapshot** — canonical roles and available modes.
4. **Execution snapshot** — milestone, parent tracker, current Phase/blocker, open PR/CI when present.
5. **Real gate** — the first unresolved operating-source, decision, permission, milestone, CI, review, or dependency gate.
6. **Recommended next mode** — `meeting`, `resume`, or `update`, with one-line justification.
7. **Evidence uncertainty** — any state that could not be verified.

## Read-only boundary

During `init`, do not:

- create, update, close, assign, or comment on issues;
- create, update, review, merge, or close pull requests;
- create or update branches, commits, tags, releases, or milestones;
- edit repository or Library files;
- rerun CI;
- treat chat memory as acceptance evidence;
- replace the pinned central revision with a floating ref or silently upgrade the OS/skill source;
- activate a mutation/execution skill as authority to write during `init`;
- make a semantic or compatibility decision;
- silently continue into another work mode.

Discovery, inspection, and reporting are allowed.

## Counterexamples

The following are invalid:

```text
new chat + remembered summary -> resume without GitHub inspection
consumer binding pins SHA A -> load central skill from floating main/SHA B
central skill exists -> keep every skill permanently active
README says Phase N -> assume Phase N is still current
Milestone says 100% -> assume Validator/parent/main gates passed
init -> automatically modify or merge repository state
init -> silently choose a new protocol or architecture meaning
init -> load every central or consumer repository file before identifying the current gate
```

## Persistent ChatGPT Project entry hook

A repository document cannot guarantee that a completely new chat knows where the repository rules live. The ChatGPT Project instructions should therefore retain only this small stable entry hook:

```text
For the Simulation Ontology project, when the user sends "init", use GitHub to open
HyungseonSong-plasma/simulation-ontology and follow
docs/operations/project-session-init.md as a read-only bootstrap. That procedure
loads the consumer-pinned chatgpt-operation OS/skill working set before SOL-specific
state. Do not rely on prior chat memory and do not mutate project state until the
user selects or confirms meeting, resume, or update.
```

Keep detailed and changeable workflow rules in this repository rather than duplicating them in Project instructions.

## Relationship to other modes

```text
init
  -> pinned central OS + init skill working set
  -> SOL roles + current repository evidence
  -> meeting  # decide
  -> resume   # execute accepted work
  -> update   # synchronize accepted documentation state
```

`meeting`, `resume`, and `update` retain the definitions and gates in `logical-agent-workflow.md`. `init` only restores enough authoritative context to choose among them safely.
