# SOL v0.1 ADR-0023/0024 Implementation Focused Readback

**Role:** Validation  
**Date:** 2026-08-20  
**Scope:** Predicate/Conditional schema and semantic-helper transcription against accepted ADR-0023 and ADR-0024  
**Verdict:** Revise

## Inputs

- `docs/decisions/0023-conditional-constraint-and-predicate-evaluation-contract.md`
- `docs/decisions/0024-cross-family-validation-state-aggregation.md`
- `schema/predicate-authoring-v0.1.schema.json`
- `schema/predicate-normalized-v0.1.schema.json`
- `schema/constraint-conditional-authoring-v0.1.schema.json`
- `schema/constraint-conditional-normalized-v0.1.schema.json`
- `tests/constraint_predicate_semantics.py`
- `tests/test_constraint_conditional_predicate_schema.py`

## Findings

### CPIV-01 — normalized Membership duplicate-removal contract is not enforced

**Classification:** Validation-tooling / schema transcription defect  
**Architecture impact:** none

ADR-0023 states that normalized Membership values have mathematical set semantics and that duplicates are removed. The current normalized Predicate schema declares `values` as an array with type-specific items but does not require uniqueness. The semantic helper evaluates membership in a way that makes duplicates observationally irrelevant, but it does not canonicalize a normalized Membership payload or otherwise reject duplicate normalized members.

Counterexample:

```json
{
  "predicate": "membership",
  "ref": {"key": "cfg:mode"},
  "scalar_kind": "string",
  "values": ["advanced", "advanced"]
}
```

This payload currently satisfies the normalized structural shape even though the accepted normalized-representation contract says duplicates are removed.

Required remediation is narrow: ensure a normalized Membership payload cannot retain duplicate canonical values. This may be enforced structurally where exact JSON equality is sufficient and/or by the normalization helper where semantic canonicalization is required. Authoring payloads may continue accepting duplicates because normalization is responsible for removal.

## Regression review

No new defect was found in:

- exact-one EvaluationReference binding;
- PRESENT / ABSENT / UNRESOLVED distinction;
- Exists presence semantics;
- Compare ABSENT -> INDETERMINATE;
- scalar-kind mismatch -> EvaluationFailure;
- strong-Kleene truth semantics;
- EvaluationFailure dominance over Boolean truth reduction;
- `then: 1..*` ordinary Constraint restriction;
- nested Conditional rejection;
- ADR-0024 ordinary-family INDETERMINATE propagation;
- `FAIL > INDETERMINATE > PASS` precedence;
- rejection of `BLOCKED` from the three-state aggregation slice.

## Verdict

**Revise** — CPIV-01 only.

The accepted ADR contracts are not reopened. After duplicate-removal enforcement is added, run a focused re-review of CPIV-01 plus regression cases. Backend installation, licensing, Adapter runtime, and repository-download availability are outside this design-stage semantic verdict.
