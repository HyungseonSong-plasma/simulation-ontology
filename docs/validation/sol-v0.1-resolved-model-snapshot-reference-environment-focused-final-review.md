# SOL v0.1 Resolved Model Snapshot + Reference Environment — Focused Final Review

**Role:** Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-resolved-model-snapshot-and-thermal-reference-environment-proposal-v0.2.md`  
**Previous finding:** RSV-01

## Verdict

**ACCEPT.**

## RSV-01 closure

The revised snapshot no longer attaches an unlabeled `value_definition` directly to a generic Entity instance.

Instead:

```text
EntityInstance
  -> PropertyAssignment
       -> canonical PropertyDefinition
       -> InlineValueDefinition
```

This preserves the accepted SOL separation between Entity, Property, ValueDefinition, and Value while keeping the property assignment lightweight and non-reified.

Determinism is sufficient for the focused reference use:

- property identity is canonical and resolved from the exact ontology environment;
- one PropertyDefinition appears at most once per Entity instance;
- assignment ordering is irrelevant;
- unknown property identity fails before value evaluation;
- backend-local parameter names cannot substitute for PropertyDefinition identity;
- arbitrary lifecycle/provenance fields are not admitted into the focused assignment shape.

## Regression review

No regression was found in the previously accepted v0.1 snapshot decisions:

- closed finite model snapshot;
- exact ontology package environment;
- unique opaque model-instance identity;
- canonical EntityType references;
- canonical relation triples;
- duplicate relation triple rejection;
- subtype/allowed-pair endpoint validation;
- ordinary cardinality and later QRC evaluation over the closed graph;
- minimal committed normalized reference packages with durable canonical IDs;
- backend runtime/license exclusion.

The focused contract deliberately does not solve general PropertyDefinition applicability/domain semantics. That broader feature is not required to interpret or validate the chosen Thermal/Plasma reference fixtures and is therefore not introduced speculatively.

## Finding status

| Finding | Status |
|---|---|
| THV-01 normalized model snapshot | **Resolved** |
| THV-02 committed canonical-ID reference environment | **Resolved by accepted approach; implementation pending** |
| RSV-01 explicit property semantics | **Resolved** |

## Decision readiness

The contract is ready for a focused ADR and design-stage implementation slice:

1. `resolved-model-snapshot-v0.1.schema.json`;
2. semantic validator for package/type/property/relation/cardinality resolution;
3. minimal normalized Core reference fixture;
4. minimal Thermal extension package;
5. machine-readable Thermal model snapshot;
6. focused semantic/readback Validation.

No backend installation, license, production Adapter, or solver execution is required for this gate.
