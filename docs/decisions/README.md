# Design Decisions

This directory records significant ontology design decisions and their rationale.

[Back to documentation map](../README.md) · [Research evidence](../research/README.md) · [Ontology Language](../ontology-language.md)

Each decision should state the problem, considered alternatives, decision, rationale, and consequences. This keeps the current specification concise while preserving the reasoning that led to it.

## Decision index

| ADR | Decision area | Primary evidence / related decisions |
|---|---|---|
| [0001](0001-parameter-semantics-and-backend-metadata-mapping.md) | Parameter semantics and backend metadata mapping | [Cross-backend matrix](../research/cross-backend-semantic-mapping-matrix-v0.1.md) |
| [0002](0002-value-and-value-definition-semantics.md) | Value and ValueDefinition semantics | [Value identity study](../research/value-and-value-definition-identity-study-v0.1.md), [ADR 0003](0003-reference-and-value-dependency-semantics.md) |
| [0003](0003-reference-and-value-dependency-semantics.md) | Reference and value dependency semantics | [Reference study](../research/reference-vs-value-definition-study-v0.1.md), [ADR 0002](0002-value-and-value-definition-semantics.md) |
| [0004](0004-unit-and-physical-dimension-semantics.md) | Unit and PhysicalDimension semantic separation | [Dimensionless/unit study](../research/dimensionless-unit-dimension-study-v0.1.md), [ADR 0005](0005-dimension-contract-and-metrology-boundary.md) |
| [0005](0005-dimension-contract-and-metrology-boundary.md) | Dimension contract and external metrology boundary | [ADR 0004](0004-unit-and-physical-dimension-semantics.md), [Unit stress test](../research/unit-reference-stress-test-v0.1.md), [Metrology contract](../research/unit-reference-and-metrology-adapter-contract-v0.1.md) |
| [0006](0006-semantic-preservation-across-backends.md) | Preserve SOL semantics across weaker backend projections | [Cross-backend matrix](../research/cross-backend-semantic-mapping-matrix-v0.1.md) |

## Traceability rule

Every accepted ADR SHOULD link to its principal evidence and to any ADR it extends, depends on, or supersedes. Research documents SHOULD link back to the ADRs they support. Normative rules derived from an ADR SHOULD be reflected in [Ontology Language](../ontology-language.md).
