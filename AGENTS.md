# AGENTS.md

## Validation policy

All agent-produced changes must comply with:

`docs/validation/agent-first-validation-principles.md`

A change is not complete until its intent, scope, semantics, and observable behaviour are supported by reproducible validation evidence.

Before changing code or ontology definitions, identify:

- the requested intent and acceptance criteria;
- invariants that must remain true;
- the allowed and forbidden change scope;
- valid, boundary, and expected-failure cases;
- the evidence required for acceptance.

Do not silently infer ambiguous scientific meaning, units, references, identity mappings, or backend compatibility. Report unsupported, ambiguous, or lossy transformations explicitly.

Tests authored with an implementation are necessary but may not be sufficient. High-risk changes require an independent oracle such as a predeclared acceptance test, separate validation pass, analytical solution, benchmark, manufactured solution, cross-backend comparison, or explicit human approval.

When reporting completion, include the changed scope, validation commands and results, assumptions and defaults, unresolved limitations, and any semantic loss.
