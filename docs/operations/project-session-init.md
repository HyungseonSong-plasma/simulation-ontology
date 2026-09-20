# SOL Project Session Initialization

**Status:** consumer-local Paul bootstrap configuration  
**Command:** `init`  
**Scope:** Simulation Ontology repository additions to the central Paul `session-bootstrap` skill

## Central authority

The exact central source is defined in:

```text
docs/operations/chatgpt-operation-binding.json
```

A valid initialization requires:

```text
OS = Paul
Paul essential rules loaded
session-bootstrap loaded
state-refresh loaded
```

The generic bootstrap algorithm, fail-closed behavior, exact-pin rule, memory/current-evidence rules, and read-only boundary are owned by the pinned Paul OS and `skills/session-bootstrap/README.md`.

Do not duplicate those algorithms locally.

## Local authority sources

After the central bootstrap layer is established, restore the minimum SOL-specific context from:

1. root `AGENTS.md`;
2. this file;
3. `docs/operations/logical-agent-workflow.md`;
4. `docs/operations/github-milestone-convention.md`;
5. current GitHub repository evidence;
6. only ADRs, contracts, plans, and implementation records relevant to the first real gate.

SOL semantics and project acceptance remain local even when a central skill performs the mechanics.

## Consumer inputs to session-bootstrap

Use:

```text
consumer repository:
  HyungseonSong-plasma/simulation-ontology

local semantic authority:
  accepted ADRs
  Public Contract / Adapter Protocol / schemas
  accepted versioning and roadmap plans

durable work state:
  current milestone / parent tracker / phase issue / PR / main evidence

local roles/modes:
  Manager / meeting
  Planner
  Researcher
  Validator
  Operator / resume / update
```

## Current-state restoration

Inspect current GitHub evidence sufficient to identify:

- latest verified default-branch SHA;
- current or most recently completed roadmap/GitHub Milestone;
- normative parent tracker;
- eligible Phase or blocker issue;
- open PR and exact-head CI/review/mergeability state when applicable;
- unresolved milestone closure or documentation handoff.

Read only the governing ADR/contract/plan material needed for the current gate.

## Next-mode selection

When evidence permits, recommend exactly one:

- `meeting` — unresolved architecture, ontology, Public Contract, Adapter Protocol, roadmap, compatibility, or other material decision;
- `resume` — accepted executable work is eligible and no real gate blocks it;
- `update` — accepted implementation state needs documentation synchronization.

The recommendation does not itself execute that mode.

## Initialization report

Report:

1. Paul central revision and loaded init skills;
2. repository/ref;
3. local roles/modes;
4. current milestone/parent/Phase/PR/CI state;
5. first real gate;
6. recommended next mode;
7. unresolved evidence uncertainty.

No operating metrics context is required.

## Persistent ChatGPT Project hook

A minimal stable hook is sufficient:

```text
For the Simulation Ontology project, when the user sends "init", use GitHub to open
HyungseonSong-plasma/simulation-ontology, read the exact Paul binding in
docs/operations/chatgpt-operation-binding.json, follow the pinned central
session-bootstrap skill, then apply docs/operations/project-session-init.md
for SOL-specific additions. Stop read-only after the initialization report.
```

Detailed generic bootstrap rules belong in `chatgpt-operation`; SOL-specific semantics remain here.
