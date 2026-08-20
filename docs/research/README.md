# Research Index

Research documents are evidence and design investigations. They are not normative unless promoted into an accepted ADR or the SOL language specification.

[Back to documentation map](../README.md) · [Design decisions](../decisions/README.md) · [Ontology Language](../ontology-language.md)

## Studies

| Study | Purpose | Related decisions / follow-up |
|---|---|---|
| [Cross-backend semantic mapping matrix](cross-backend-semantic-mapping-matrix-v0.1.md) | Compare MOOSE, COMSOL, and Ansys semantic boundaries. | [ADR 0001](../decisions/0001-parameter-semantics-and-backend-metadata-mapping.md), [ADR 0006](../decisions/0006-semantic-preservation-across-backends.md) |
| [Reference vs ValueDefinition](reference-vs-value-definition-study-v0.1.md) | Test whether reference is a value-definition mechanism or relation/dependency concern. | [ADR 0003](../decisions/0003-reference-and-value-dependency-semantics.md) |
| [Dimensionless, unit, and dimension](dimensionless-unit-dimension-study-v0.1.md) | Separate SemanticQuantity, PhysicalDimension, Unit, and Value. | [ADR 0004](../decisions/0004-unit-and-physical-dimension-semantics.md), [ADR 0005](../decisions/0005-dimension-contract-and-metrology-boundary.md) |
| [Value and ValueDefinition identity](value-and-value-definition-identity-study-v0.1.md) | Test typed value vs semantic entity and evaluation mechanism vs identity. | [ADR 0002](../decisions/0002-value-and-value-definition-semantics.md), [Entity boundary study](entity-boundary-and-orthogonal-axes-study-v0.1.md) |
| [Entity boundary and orthogonal axes](entity-boundary-and-orthogonal-axes-study-v0.1.md) | Generalize semantic identity boundary and independent semantic axes. | Candidate E1/E2 architecture principles; feeds future ADR. |
| [UnitReference stress test](unit-reference-stress-test-v0.1.md) | Stress-test dimensionless, affine temperature, and compound units. | [UnitReference / metrology contract](unit-reference-and-metrology-adapter-contract-v0.1.md) |
| [UnitReference and metrology adapter contract](unit-reference-and-metrology-adapter-contract-v0.1.md) | Define UnitReference, quantity-owned conversion semantics, binding expectation, and adapter boundary. | Extends [ADR 0005](../decisions/0005-dimension-contract-and-metrology-boundary.md); feeds Relation/Constraint design. |

## Evidence flow

The intended traceability direction is:

```text
backend / standards evidence
        ↓
research study
        ↓
ADR
        ↓
ontology-language.md / architecture.md
```

A research conclusion that changes Core semantics should not silently replace an accepted ADR. It should either produce a new ADR or explicitly supersede the older decision.
