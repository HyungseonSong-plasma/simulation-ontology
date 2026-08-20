# SOL v0.1 Compatibility Constraint Independent Review v0.1

**Status:** Validation finding  
**Date:** 2026-08-20  
**Target:** `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.1.md`  
**Normative basis:** ADR-0007, ADR-0008, ADR-0009, ADR-0018 through ADR-0021

## Validation method

The proposal was reviewed as an independent contract, without treating its Research verdict as authoritative. The review asked whether two validators given the same resolved SOL environment and same model input are forced to construct the same Compatibility obligations and the same verdicts.

Official backend evidence was used only to stress the semantic boundary:

- MOOSE `MooseUnits` separates dimensional conformance from other semantic checks;
- COMSOL checks expected argument dimensions/unit consistency independently;
- Ansys System Coupling creates transfers between variables of same/compatible quantity type, providing evidence for a compatibility relation richer than literal unit equality.

Backend implementation behavior is not promoted into Core semantics.

## Overall verdict

**Revise**

The proposal correctly preserves one generic Compatibility primitive, preserves the semantic/representability separation, and gives Compatibility criterion identity a durable semantic role. Four contract ambiguities remain. None requires reopening the six-family architecture.

## Findings

### COMP-01 — Normalized operand reference does not preserve ADR-0009 identity space

**Classification:** Architecture transcription / contract defect  
**Severity:** Required revision

The normalized payload currently uses:

```yaml
left: <resolved-semantic-reference>
right: <resolved-semantic-reference>
```

but does not normatively distinguish **schema identity** from **model-instance identity**. ADR-0009 explicitly separates schema identity, model-instance identity, and backend-local identity.

Counterexample:

```text
left = "plasma:Temperature"
```

Validator A interprets this as a schema concept. Validator B interprets the same string as a model-instance reference supplied by a local namespace. Both satisfy the current structural description but create different obligations.

The normalized operand reference must therefore carry enough information to identify at least:

```text
identity_space = schema | model_instance
stable resolved identifier
```

Backend-local identity must be forbidden from the Core Compatibility payload.

This does not require fixing a global model-instance URI grammar.

### COMP-02 — Symmetric obligation identity depends on an unspecified comparison key

**Classification:** Contract determinism defect  
**Severity:** Required revision

The proposal says symmetric criteria canonicalize `(A,B)` using "stable resolved-reference comparison keys" but no such ordering contract exists in ADR-0009.

Two validators may sort equivalent resolved references differently while still respecting all existing ADRs.

The semantic `ObligationKey` should not depend on an unspecified lexical ordering. Either:

1. define symmetric operand identity as an unordered two-member multiset/set of typed resolved references; or
2. define a canonical ordering key as part of the normalized reference contract.

The first option is architecturally smaller because textual serialization order can remain non-semantic.

### COMP-03 — Binary evaluator contract conflicts with unresolved semantic-service case

**Classification:** Contract lifecycle/evaluation defect  
**Severity:** Required revision

The proposal defines:

```text
evaluate(...) -> compatible | incompatible
```

but later requires an explicit unresolved evaluation state if a semantic service such as a metrology resolver is unavailable.

Independent validators can therefore disagree between:

```text
normalization failure
indeterminate evaluation
incompatible
operational blocked state
```

The contract must choose one boundary.

Recommended minimal boundary for design-stage v0.1:

- criterion/environment prerequisites required for semantic evaluation are normalization/evaluation-context prerequisites;
- if unresolved, no binary semantic result is produced and the validator emits one deterministic semantic-resolution diagnostic/state;
- backend runtime/license/capability absence remains outside this semantic prerequisite system.

The exact diagnostic name may be chosen by Research, but the state transition must be unique.

### COMP-04 — Reducibility-to-another-primitive cannot be a mandatory validator test

**Classification:** Validation contract defect  
**Severity:** Required revision

Boundary case 10 requires a validator to diagnose a Type/Dimension/Value/Cardinality rule "disguised" as Compatibility.

That architectural guidance is useful, but generic machine detection is not deterministic. For example, a semantic-quantity compatibility criterion may internally use Type and Dimension evidence while still representing a domain contract that is intentionally stronger than either primitive alone.

Two conforming validators can disagree on whether a criterion is reducible without violating any accepted ADR.

Therefore:

- the non-duplication rule should remain a **design/review rule** for criterion definitions;
- it should not be a required generic runtime/schema validator outcome unless a later criterion-specific contract makes the reduction mechanically decidable.

## Accepted parts / no regression

The following parts are accepted and should not be reopened during revision:

1. one generic Compatibility primitive remains the v0.1 taxonomy;
2. compatibility criterion semantics require canonical durable identity rather than a free-form diagnostic string;
3. using reification when identity/reuse/versioning/evaluator semantics justify it is compatible with ADR-0007;
4. v0.1 binary compatibility arity is acceptable as a focused slice;
5. `expect = compatible | incompatible` is sufficient for obligation polarity;
6. same obligation + same expectation deduplicates;
7. same obligation + opposite expectation conflicts;
8. different criterion identities remain distinct conjunctive obligations;
9. semantic compatibility remains separate from backend/Profile representability and backend release matching;
10. backend installation, license, runtime availability, and MappingPlan state are not Core Compatibility inputs.

## Required revision scope

Research should revise only these four items:

- typed normalized operand reference preserving identity space;
- deterministic symmetric obligation identity without unspecified ordering;
- one explicit unresolved semantic-evaluation prerequisite boundary;
- move generic reducibility detection from validator requirement to design guidance.

No new Compatibility subclass taxonomy, backend capability model, path language, N-ary algebra, or execution/plugin architecture is required.

## Final verdict

**Revise** — limited contract revision only.

The proposal is structurally viable and does not expose an architecture failure. A focused v0.2 revision should be sufficient for final contract re-review.
