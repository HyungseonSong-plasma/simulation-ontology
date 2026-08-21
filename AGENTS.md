# SOL Agent Entry Point

This repository uses logical project roles and explicit work modes. They are project operating concepts, not runtime SOL Agents.

## New or uncertain session

When the user sends `init`, or when session context may be missing or stale:

1. read `docs/operations/project-session-init.md`;
2. follow its read-only bootstrap procedure exactly;
3. read `docs/operations/logical-agent-workflow.md` and `docs/operations/github-milestone-convention.md`;
4. report the restored repository/role/execution snapshot;
5. stop before mutation until the user selects or confirms the next mode.

Do not reconstruct project state from conversation memory alone.

## Canonical modes

- `init` — project-level, read-only session bootstrap;
- `meeting` — Manager-led decision process;
- `resume` — Operator execution of accepted work until a real gate;
- `update` — Operator synchronization of documentation with accepted state.

Planner, Researcher, and Validator responsibilities are defined in `docs/operations/logical-agent-workflow.md`.

## Authority

Use the repository hierarchy defined by the operating convention:

1. accepted ADRs and published contracts/schemas;
2. accepted plans and compatibility/versioning policy;
3. `docs/operations/` conventions;
4. README, guides, examples, and chat summaries.

Current GitHub issue, PR, CI, and verified `main` evidence must be inspected before execution. GitHub Milestone percentage and prior chat memory are not acceptance authority.

## Scope discipline

Read only the governing material needed for the current gate. Do not silently change ontology, architecture, Public Contract, Adapter Protocol, roadmap, or compatibility meaning during implementation or documentation work.
