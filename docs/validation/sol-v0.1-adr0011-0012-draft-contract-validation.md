# SOL v0.1 proposed ADR-0011 / ADR-0012 — contract transcription validation

**Status:** Independent Validation role review  
**Date:** 2026-08-20

## Scope

Validate whether proposed ADR-0011 and ADR-0012 faithfully preserve the already accepted remediation contracts. This review does not reopen AR-01–AR-04 architecture design and does not treat proposal prose as authoritative where the final validation gate did not accept it.

## ADR-0011

**Verdict: Revise — drafting defect, not architecture defect.**

The draft preserves the major accepted boundaries: five-field MappingRule, four-field MappingClaim, complete effect-pair checking, component ownership, local/cross-component comparator context, executable PlanAction, dependency/idempotency/atomicity separation, lifecycle versus representability, component-keyed BackendTarget, and no backend-native Core vocabulary.

Two contract details are insufficiently preserved:

1. **Comparator registry selection key:** the accepted contract requires comparison purpose and normalized operand kinds in the exact binding key. The draft requires a unique comparator but does not state these discriminators as normative lookup inputs. Independent implementations could therefore define different registry partitions while still claiming conformance.
2. **Immutable evaluation revisions:** the accepted lifecycle contract requires `pending -> blocked|indeterminate|complete` within one immutable evaluation revision; resolving blocked/indeterminate evidence or runtime drift creates a new revision rather than mutating historical outcome. The draft omits this rule.

Required correction: add both clauses without changing architecture shape.

## ADR-0012

**Verdict: Revise — drafting defect, not architecture defect.**

The draft preserves QRC taxonomy, closed-snapshot requirement, qualifier identity, stable-identity counting, multiple typing, interval intersection, subtype algebra, conjunctive composition, and deferred universal predicates.

Two accepted normative details were weakened/omitted:

1. supplied cardinality bounds must be non-negative integers and invalid values have canonical `FAIL: QRC_BOUND_INVALID`;
2. failure diagnostics in the accepted contract SHALL include constrained source, relation, qualifier identity, normalized interval, observed count, counted canonical identities, and ontology/package version context; the draft weakens this to SHOULD.

Required correction: restore the canonical invalid-bound result and SHALL-level diagnostic requirement.

## Decision

These are transcription defects in Proposed ADR text, not new Architecture/Profile/Adapter/Backend/Reference-model/Validation-tooling defects. Correct the Proposed ADRs and run a focused readback check. Architecture freeze remains blocked independently by executable validation readiness.
