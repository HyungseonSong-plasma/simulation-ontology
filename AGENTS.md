# SOL Agent Entry Point

This repository uses logical project roles and explicit work modes. They are project operating concepts, not runtime SOL Agents.

## New or uncertain session

When the user sends `init`, follow the consumer binding in `docs/operations/chatgpt-operation-binding.json`.

Initialization must:

1. resolve the exact pinned `chatgpt-operation` revision;
2. verify that the central OS is **Samuel**;
3. read `docs/operating_system/ESSENTIAL_RULES.md`;
4. load `session-bootstrap`, `state-refresh`, and the mandatory `problem-oriented-response` skill plus machine contract;
5. index the pinned skill catalog and capability registry;
6. apply the local additions in `docs/operations/project-session-init.md`;
7. report the restored repository/role/execution snapshot, response contract, and first real gate;
8. stop read-only.

Generic authority, pinning, current-evidence, evidence-boundary, and interruption rules are owned by Samuel and are not duplicated here.

## Canonical modes

- `init` — project-level, read-only session bootstrap;
- `meeting` — Manager-led decision process;
- `resume` — Operator execution of accepted work until a real gate;
- `update` — Operator synchronization of documentation with accepted state.

Planner, Researcher, and Validator responsibilities are defined in `docs/operations/logical-agent-workflow.md`.

Samuel's mandatory post-init response contract is `samuel.problem-first-response.v1`: unresolved problems, human-review boundaries, and explicit decision questions come first; detailed evidence is available on request. This does not alter SOL's local authority or approval gates.

## Authority

Samuel essential rules and reusable operating mechanics are owned by the exact central revision in the binding.

This repository remains authoritative for SOL domain semantics, ADRs, published contracts/schemas, compatibility/versioning policy, roles, roadmap state, and acceptance criteria.

Within SOL meaning, use this hierarchy:

1. accepted ADRs and published contracts/schemas;
2. accepted plans and compatibility/versioning policy;
3. local operations conventions;
4. README, guides, examples, and summaries.

## Scope discipline

Read only the governing material needed for the current gate. Do not silently change ontology, architecture, Public Contract, Adapter Protocol, roadmap, or compatibility meaning during implementation or documentation work.
