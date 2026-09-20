# SOL Agent Entry Point

This repository uses logical project roles and explicit work modes. They are project operating concepts, not runtime SOL Agents.

## New or uncertain session

When the user sends `init`, or when session context may be missing or stale:

1. read `docs/operations/chatgpt-operation-binding.json` from the current verified SOL revision;
2. resolve the exact pinned `HyungseonSong-plasma/chatgpt-operation` revision from that binding;
3. read the pinned central OS index, skill catalog, and the `init` working-set skills before loading project-specific operating state;
4. read `docs/operations/project-session-init.md` and follow its read-only bootstrap procedure exactly;
5. read `docs/operations/logical-agent-workflow.md` and `docs/operations/github-milestone-convention.md`;
6. report the restored operating-skill/repository/role/execution snapshot;
7. stop before mutation until the user selects or confirms the next mode.

Do not reconstruct project state from conversation memory alone. Do not follow a floating central `main`/latest ref after the consumer binding has selected an exact revision.

## Canonical modes

- `init` — project-level, read-only session bootstrap;
- `meeting` — Manager-led decision process;
- `resume` — Operator execution of accepted work until a real gate;
- `update` — Operator synchronization of documentation with accepted state.

Planner, Researcher, and Validator responsibilities are defined in `docs/operations/logical-agent-workflow.md`.

## Authority

Authority is scope-based:

- the exact revision pinned in `docs/operations/chatgpt-operation-binding.json` owns reusable ChatGPT operating-system identity and portable operating mechanics;
- this repository owns SOL domain semantics, repository-specific policy, roles, roadmap state, acceptance criteria, and local bootstrap additions.

Within SOL domain/project meaning, use this repository hierarchy:

1. accepted ADRs and published contracts/schemas;
2. accepted plans and compatibility/versioning policy;
3. `docs/operations/` conventions;
4. README, guides, examples, and chat summaries.

Central skills must not override SOL semantic authority. Current GitHub issue, PR, CI, and verified `main` evidence must be inspected before execution. GitHub Milestone percentage and prior chat memory are not acceptance authority.

## Scope discipline

Read only the governing material needed for the current gate. Do not silently change ontology, architecture, Public Contract, Adapter Protocol, roadmap, or compatibility meaning during implementation or documentation work.
