# SOL v0.1 ADR-0010 remediation v0.2.2 — focused final contract review

**Status:** Independent Validation role review  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Scope

Review only the C-21-01 delta in:

- `docs/research/sol-v0.1-adr0010-supplement-remediation-proposal-v0.2.2.md`

against:

- `docs/validation/sol-v0.1-remediation-proposals-v0.2.1-final-contract-re-review.md`
- the unchanged v0.2.1 contract inherited by v0.2.2.

Research's proposed `Resolved` verdict was not assumed correct.

## Counterexample replay

### C-21-01 — effects owned by different target components

Input:

```text
EA.resource_component_id = A
EB.resource_component_id = B
A != B
```

v0.2.2 now forces:

```text
comparison context
= cross_component(A-binding, B-binding, orchestration-binding)
```

rather than allowing a validator to select A, B, or the orchestrator heuristically. The orchestration adapter contract owns the exact cross-component comparator binding. For symmetric purposes the pair is canonicalized; for directional transfer purposes source/target order is retained. Zero/multiple registry matches remain validation FAILs.

The old outcomes:

```text
V1 -> choose A comparator
V2 -> choose B comparator
V3 -> choose orchestration comparator
```

are no longer all conforming. Only the canonical local/cross-component context is conforming.

**Verdict: Resolved.**

## Component ownership boundary

`RealizationEffect.resource_component_id` is derived MappingPlan/Adapter evidence rather than a MappingClaim field. Missing structural ownership is FAIL; unresolved valid runtime component evidence is BLOCKED; ambiguous ownership is FAIL. This preserves the existing provenance-based decision boundary and does not add backend vocabulary to Core.

**Verdict: Accept.**

## Candidate-pair and pruning consistency

Different component identities are explicitly insufficient to prove disjointness. Cross-component pairs remain in the complete effect-pair universe until a uniquely selected cross-component procedure proves disjointness. Same-action effects are not silently exempted.

This is conservative but deterministic and does not reopen RE-01.

**Verdict: Accept.**

## Backend sanity check

- MOOSE exposes explicit task dependencies in its Action system, consistent with local action/effect lowering outside SOL Core: https://mooseframework.inl.gov/moose/source/actions/Action.html
- COMSOL separates feature creation, selection, and property mutation, consistent with component-local action/effect descriptors: https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html
- Ansys System Coupling defines data transfers between explicitly identified participant sides, supporting the need for pair/direction-aware cross-component comparison context: https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/pdf/System_Coupling_Users_Guide.pdf

No backend-native object taxonomy is imported into Core.

## Final contract verdict

| Target | Verdict |
|---|---|
| v0.2.2 cross-component comparator delta | **Accept** |
| ADR-0010 remediation contract as v0.2.1 + v0.2.2 | **ADR drafting ready** |
| QRC v0.2.1 | **ADR drafting ready** (per previous final review) |
| MappingRule 5-field | **Accept at contract level** |
| MappingClaim 4-field | **Accept at contract level** |
| Executable validation readiness | **Not yet complete** |
| Architecture freeze | **Blocked** |

No contract-level `Unresolved` or `Regression` remains in the reviewed AR-01/02/03/04 remediation scope.

## Next gate

Operating Desk may enter Decision and authorize ADR drafting. Architecture freeze must remain blocked until machine-readable schemas, validators, MappingPlan generator, comparator registry, BackendTarget resolver, and Thermal/Plasma executable fixtures demonstrate the accepted contract without hidden side channels or order dependence.
