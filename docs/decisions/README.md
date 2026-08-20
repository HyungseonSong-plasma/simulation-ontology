# Design Decisions

This directory records significant ontology design decisions and their rationale.

[Back to documentation map](../README.md) · [Research evidence](../research/README.md) · [Ontology Language](../ontology-language.md)

Each accepted decision states the semantic problem, considered alternatives, decision, rationale, consequences, and validation evidence. Later focused ADRs may refine serialization or validation boundaries without reopening unrelated earlier decisions.

## Decision index

| ADR | Decision area |
|---|---|
| [0001](0001-parameter-semantics-and-backend-metadata-mapping.md) | Parameter semantics and backend metadata mapping |
| [0002](0002-value-and-value-definition-semantics.md) | Value and ValueDefinition semantics |
| [0003](0003-reference-and-value-dependency-semantics.md) | Reference and value-dependency semantics |
| [0004](0004-unit-and-physical-dimension-semantics.md) | Unit and PhysicalDimension separation |
| [0005](0005-dimension-contract-and-metrology-boundary.md) | Dimension contract and metrology boundary |
| [0006](0006-semantic-preservation-across-backends.md) | Semantic preservation across weaker backend projections |
| [0007](0007-constraint-architecture-and-composition.md) | Constraint families and composition |
| [0008](0008-inheritance-and-interface-composition.md) | Entity inheritance and Interface composition |
| [0009](0009-identity-namespace-package-and-versioning.md) | Identity, namespace, package, and versioning |
| [0010](0010-profile-backend-mapping-contract.md) | Profile/backend mapping contract |
| [0011](0011-mapping-plan-determinism-and-executable-backend-target.md) | MappingPlan determinism and backend target |
| [0012](0012-qualified-relation-cardinality.md) | Qualified Relation Cardinality |
| [0013](0013-sol-v0.1-design-stage-architecture-freeze.md) | SOL v0.1 design-stage architecture freeze |
| [0014](0014-interface-disambiguation-and-machine-readable-taxonomy-semantics.md) | Interface disambiguation and explicit taxonomy |
| [0015](0015-simulation-model-task-composition-semantics.md) | Simulation / Model / Task composition |
| [0016](0016-model-component-membership-and-condition-target-semantics.md) | Model component membership and condition targeting |
| [0017](0017-core-relation-cardinality-requiredness-baseline.md) | Core relation cardinality baseline |
| [0018](0018-constraint-authoring-normalization-and-cardinality-schema-boundary.md) | Constraint authoring/normalization boundary |
| [0019](0019-relation-target-type-constraint-semantics-and-schema.md) | Relation-target Type Constraint |
| [0020](0020-dimension-constraint-and-canonical-dimensionvector.md) | Dimension Constraint and canonical DimensionVector |
| [0021](0021-scalar-value-constraint-and-exact-decimal-normalization.md) | Value Constraint and exact-decimal normalization |
| [0022](0022-compatibility-constraint-and-semantic-criterion-contract.md) | Compatibility Constraint |
| [0023](0023-conditional-constraint-and-predicate-evaluation-contract.md) | Conditional Constraint and Predicate evaluation |
| [0024](0024-cross-family-validation-state-aggregation.md) | Cross-family validation-state aggregation |
| [0025](0025-interface-serialization-and-inherited-capability-conformance.md) | Interface serialization and inherited conformance |
| [0026](0026-value-unit-dimension-and-valuedefinition-transcription-boundary.md) | Value/Unit/Dimension/ValueDefinition transcription |
| [0027](0027-inline-valuedefinition-format-provider-boundary.md) | InlineValueDefinition format-provider boundary |
| [0028](0028-canonical-ontology-package-and-resource-integration.md) | Canonical ontology package/resource integration |
| [0029](0029-resolved-model-snapshot-and-reference-validation-environment.md) | Resolved model snapshot and reference-validation environment |
| [0030](0030-sol-v0.1-design-stage-closure.md) | SOL v0.1 design-stage closure |

## Current design-stage state

**SOL v0.1 DESIGN-STAGE CLOSED by ADR-0030.**

- Core architecture is frozen through ADR-0017.
- Focused language/schema/package/model-snapshot consolidation is accepted through ADR-0029.
- Minimal Thermal and Plasma/QRC reference gates are accepted.
- Final independent closure readback is Accept.
- Implementation, Profile/BackendAdapter integration, and executable backend V&V are post-design work unless they reveal a genuine ADR-0030 reopen condition.

## Traceability rule

Every accepted ADR SHOULD link to principal evidence and to any ADR it extends, depends on, or supersedes. Research documents SHOULD link back to the ADRs they support. Normative rules derived from an ADR SHOULD be reflected in the Ontology Language, Architecture, and focused machine-readable artifacts where the design-stage scope requires them.
