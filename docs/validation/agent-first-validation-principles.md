# Agent-First Validation Principles

**Status:** Draft  
**Scope:** Simulation Ontology and its backend adapters  
**Primary concern:** Detect errors introduced by coding agents while preserving established software and scientific V&V practices.

## Governing principle

> No agent-produced change is trusted by authorship or plausibility. It is accepted only when its intent, scope, semantics, and observable behaviour are supported by independent, reproducible evidence.

Agent-first validation does not replace conventional software testing or scientific verification and validation. It adds controls for characteristic agent failure modes: plausible but incorrect implementations, misunderstood intent, unsupported assumptions, self-confirming tests, silent semantic loss, and unnecessary scope expansion.

## Principles

### 1. Validate intent before implementation

A syntactically correct and well-tested implementation can still solve the wrong problem. Each change must be checked against its stated intent, invariants, allowed scope, forbidden changes, expected behaviours, and expected failures.

### 2. Treat agent statements as claims, not evidence

Statements such as "implemented", "compatible", "tested", or "no regression" are not validation results. Every material claim must link to inspectable evidence such as a diff, executable test, trace, backend output, benchmark, or documented review decision.

### 3. Separate generation from judgement

The implementation and its acceptance oracle must be logically independent. Tests written from the same misunderstanding as the implementation can create a self-confirming error. High-risk changes therefore require at least one independent mechanism: a predeclared acceptance test, separate validator or review pass, analytical reference, benchmark, manufactured solution, or cross-backend comparison.

### 4. Define acceptance before or with implementation

Each task should establish the following contract before acceptance:

- intent;
- invariants;
- allowed and forbidden changes;
- valid, boundary, and expected-failure cases;
- required evidence;
- acceptance authority.

Tests may evolve during implementation, but the implementation must not be allowed to redefine its own success criteria without an explicit decision.

### 5. Validate every semantic boundary

Validation must cover the full transformation chain:

1. authoring model to canonical ontology;
2. canonical ontology to backend adapter;
3. adapter output to backend-native model;
4. backend-native model to solver result.

At each boundary, stable identity, references, units, equations, domains, conditions, and supported semantics must be preserved or explicitly diagnosed.

### 6. Prefer explicit failure over silent inference

Unsupported, ambiguous, or lossy transformations must never pass silently. Missing units, ambiguous references, undefined mappings, alias collisions, incomplete required physics, and unsupported backend semantics must produce structured diagnostics. Defaults are acceptable only when declared by policy and visible in the evidence.

### 7. Validate change scope

Validation must identify changed files, ontology entities, public contracts, dependencies, identities, aliases, migrations, and backend mappings. A change can fail validation even when tests pass if it modifies behaviour or structure outside the authorised scope.

### 8. Prioritise boundaries and negative cases

Every validation rule must include, where applicable:

1. a valid case;
2. a boundary case;
3. an expected-failure case.

Particular attention is required for identity and scope collisions, rename and deprecation behaviour, partial backend support, unit conversion, invalid intermediate states, reordered references, and repeat execution.

### 9. Require deterministic and reproducible evidence

Given the same canonical input, policy, adapter version, backend version, and relevant environment, the system must reproduce an equivalent canonical representation, diagnostics, backend configuration, and scientifically equivalent result. Non-semantic differences must be normalised explicitly rather than ignored informally.

### 10. Maintain end-to-end traceability

Validation artefacts must support the chain:

```text
Requirement
  -> Ontology invariant
  -> Validation rule
  -> Test case
  -> Evidence
  -> Acceptance decision
```

A passing test without a known requirement or invariant is supporting evidence, but not sufficient evidence of intended correctness.

### 11. Scale assurance with risk

| Risk | Typical change | Minimum assurance |
|---|---|---|
| Low | Documentation or display label | Structure and scope check |
| Medium | Property, alias, or adapter mapping | Unit, negative, and regression tests |
| High | Identity, units, equations, or coupling | Independent oracle and integration test |
| Critical | Scientific meaning, solver export, or migration | Benchmark plus cross-backend comparison or explicit human approval |

Human review should concentrate on high-risk intent, scientific meaning, exceptions, and final acceptance rather than manually reproducing every mechanical coding action.

### 12. Produce explainable decisions

Validation must not return only a boolean. Findings should include a stable rule identifier, severity, affected entity, claim, expected and observed values, evidence provenance, and a possible remediation.

A minimal representation is:

```ts
interface ValidationFinding {
  ruleId: string;
  severity: "info" | "warning" | "error" | "fatal";
  entityId?: string;
  claim: string;
  evidence: Evidence[];
  expected: unknown;
  observed: unknown;
  provenance: Provenance;
  remediation?: string;
}
```

## Required change evidence

Before a change is accepted, its evidence package should answer:

- What was requested?
- What changed?
- What was deliberately left unchanged?
- What assumptions or defaults were used?
- Which validation commands ran, in which environment?
- Which requirements and invariants were exercised?
- Which negative and boundary cases were checked?
- Was any information lost during canonical or backend transformation?
- Who or what supplied the independent acceptance decision?

## Reference practices

- [MOOSE regression testing](https://mooseframework.inl.gov/getting_started/examples_and_tutorials/tutorial01_app_development/step08_test_harness.html)
- [MOOSE Tools requirements traceability](https://mooseframework.inl.gov/python/sqa/python_rtm.html)
- [MOOSE verification methods](https://mooseframework.inl.gov/modules/stochastic_tools/sqa/stochastic_tools_srs.html)
- [OpenAI Codex best practices](https://developers.openai.com/codex/learn/best-practices)
- [OpenAI: A Practical Approach to Verifying Code at Scale](https://alignment.openai.com/scaling-code-verification/)
- [Palantir Ontology action submission criteria](https://palantir.com/docs/foundry/action-types/submission-criteria/)

## Planned companion documents

- `agent-error-taxonomy.md`
- `validation-levels.md`
- `evidence-and-acceptance.md`
