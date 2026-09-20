# SOL Agent Entry Point

This repository uses logical project roles and explicit work modes. They are project operating concepts, not runtime SOL Agents.

## New or uncertain session

When the user sends `init`, follow the consumer binding in `docs/operations/chatgpt-operation-binding.json`.

Initialization must:

1. resolve the exact pinned `chatgpt-operation` revision;
2. verify that the central OS is **Paul**;
3. read `docs/operating_system/ESSENTIAL_RULES.md`;
4. load the bound `session-bootstrap` and `state-refresh` skills;
5. apply the local additions in `docs/operations/project-session-init.md`;
6. report the restored repository/role/execution snapshot and first real gate;
7. stop read-only.

Generic authority, pinning, current-evidence, evidence-boundary, and interruption rules are owned by Paul and are not duplicated here.

## Canonical modes

- `init` — project-level, read-only session bootstrap;
- `meeting` — Manager-led decision process;
- `resume` — Operator execution of accepted work until a real gate;
- `update` — Operator synchronization of documentation with accepted state.

Planner, Researcher, and Validator responsibilities are defined in `docs/operations/logical-agent-workflow.md`.

## Authority

Paul essential rules and reusable operating mechanics are owned by the exact central revision in the binding.

This repository remains authoritative for SOL domain semantics, ADRs, published contracts/schemas, compatibility/versioning policy, roles, roadmap state, and acceptance criteria.

Within SOL meaning, use this hierarchy:

1. accepted ADRs and published contracts/schemas;
2. accepted plans and compatibility/versioning policy;
3. local operations conventions;
4. README, guides, examples, and summaries.

## Scope discipline

Read only the governing material needed for the current gate. Do not silently change ontology, architecture, Public Contract, Adapter Protocol, roadmap, or compatibility meaning during implementation or documentation work.
