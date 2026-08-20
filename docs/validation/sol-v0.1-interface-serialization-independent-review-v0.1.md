# SOL v0.1 Interface Serialization Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-interface-serialization-proposal-v0.1.md`  
**Normative basis:** ADR-0007, ADR-0008, ADR-0009, ADR-0014 and accepted Constraint-family contracts  
**Verdict:** Revise

## Accepted direction

No counterexample was found against these directions:

- Interface remains a language-level capability contract, not an Entity Type;
- `Interface` remains distinct from `SpatialInterface`;
- direct Interface extension is multiple and effective composition is conjunctive;
- backend implementation inheritance is non-normative;
- Property/Relation requirements use semantic references rather than local-name matching;
- implementation mappings are explicit and canonical-reference based;
- backend-local handles are invalid Core Interface mapping targets;
- operation/action requirements remain deferred;
- schema-capability existence is distinct from model-instance value/edge cardinality;
- extension cycles must be rejected;
- optional Interface members need not be added merely because a reference product supports them.

## Findings

### IFS-01 — Constraint requirement target/context is under-specified

**Classification:** Architecture/contract serialization defect

The proposal serializes Interface constraints as only canonical `ConstraintDefinition` references:

```yaml
constraint_requirements:
  - <constraint-definition-id>
```

This is insufficient for context-local Constraint families. A reified Value or Dimension constraint payload does not independently identify which Interface Property requirement is its subject. Likewise a context-local constraint may need to apply to one requirement rather than the Interface as a whole.

Counterexample: an Interface requires `temperature` and `emissivity` Properties and references a Dimension constraint `Theta`. Two validators cannot infer which Property requirement the Dimension contract constrains from the ConstraintDefinition ID alone.

The serialization must preserve an explicit target/context binding for Interface constraints without creating a seventh Constraint family or a general graph-query language.

### IFS-02 — Duplicate/overlapping InterfaceImplementation composition is ambiguous

**Classification:** Contract completeness defect

The proposal does not define uniqueness or conflict handling for:

- two direct declarations for the same `(entity_type, interface)` pair;
- a direct implementation of child `B` plus a redundant direct implementation of parent `A` where `B extends A`;
- two directly implemented sibling Interfaces that inherit the same canonical requirement but map it to different concrete definitions.

Counterexample:

```text
B extends A
E implements B: A.requirement R -> concrete X
E implements A: A.requirement R -> concrete Y
X != Y
```

No deterministic effective mapping exists unless a composition rule rejects or reconciles this state.

A unique direct declaration key and an order-independent effective-mapping consistency rule are required.

### IFS-03 — Entity taxonomic specialization cannot defer inherited Interface guarantees

**Classification:** Architecture/contract consistency defect

The proposal says whether an Entity subtype inherits an ancestor Entity Type's InterfaceImplementation declaration is not introduced as a v0.1 rule. This leaves two validators free to disagree and can violate ADR-0007 monotonic refinement: an Entity subtype must not silently lose a semantic guarantee inherited from its taxonomic parent.

At minimum the effective Interface contract of an Entity subtype must include Interface guarantees inherited through Entity `is_a` closure. The serialization may still store only direct implementation declarations, but effective conformance cannot forget an ancestor guarantee.

How concrete requirement mappings are reused/refined by the subtype must be deterministic and must not weaken the inherited Interface contract.

## Regression / non-findings

No defect is assigned to the absence of final PropertyDefinition serialization. The Interface slice may use canonical Property references while package/compiler integration resolves provider definitions later, provided unresolved/ambiguous references fail deterministically.

No backend installation, license, Adapter runtime, or SDK generation is required for this review.

## Verdict

**Revise.**

Return only IFS-01 through IFS-03 to Research. The accepted Interface/Entity separation, extension model, explicit semantic mapping direction, cycle rejection, and backend-independence boundaries should remain closed.
