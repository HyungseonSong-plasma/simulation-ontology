# SOL v0.1 Canonical Package / Schema Integration Proposal v0.1

**Status:** Research proposal  
**Date:** 2026-08-20  
**Scope:** normalized ontology-package integration for the accepted SOL v0.1 language/schema slices  
**Depends on:** ADR-0007, ADR-0008, ADR-0009, ADR-0014..0027

## 1. Problem

SOL now has accepted focused machine-readable contracts for Constraints, Interfaces, Value/Unit/Dimension, and substantial Core Entity/Relation transcription. However the repository still stores Core source fragments such as:

```text
ontology/core/entities.yaml
ontology/core/relations.yaml
ontology/core/constraints.yaml
```

These fragments are useful authoring/consolidation inputs but do not yet satisfy ADR-0009's normalized identity requirements:

- canonical semantic identity must be independent of local name and file path;
- package and semantic namespace are distinct;
- references must resolve deterministically;
- one active package provider per semantic namespace is required in v0.1;
- package versions and backend identities must not enter canonical semantic IDs.

A final design-stage package boundary is therefore needed before reference models are authored against a canonical snapshot.

## 2. Source fragments are not canonical identity authority

`ontology/core/*.yaml` SHALL be treated as authoring/consolidation source fragments until compiled.

File path, YAML mapping key, declaration order, and local display/authoring name SHALL NOT become canonical semantic identity.

The normalized package is the machine snapshot used for independent validation/reference-model loading.

## 3. Two distinct package layers

### 3.1 Authoring/distribution manifest

Human/tooling package metadata may contain unresolved dependency ranges and authoring-oriented namespace declarations.

Conceptually:

```yaml
package:
  name: "@simulation-ontology/core"
  version: "0.1.0"

provides:
  namespaces: [sol]

requires:
  - package: "..."
    version_range: "..."
```

This layer is resolver input, not the final normalized semantic snapshot.

### 3.2 Resolved normalized ontology package

After package resolution and semantic-name resolution, normalized package data uses exact provider/dependency versions and canonical semantic references.

Conceptually:

```yaml
package:
  name: "@simulation-ontology/core"
  version: "0.1.0"

namespaces:
  - name: sol
    exports:
      - name: Field
        kind: entity_type
        id: https://simulation-ontology.org/id/<persistent-id>

resolved_dependencies:
  - package: "@simulation-ontology/base"
    version: "0.1.0"

entity_types: [...]
properties: [...]
relations: [...]
constraint_definitions: [...]
interfaces: [...]
interface_implementations: [...]
```

No dependency ranges remain in the normalized snapshot.

## 4. Namespace export table is the authoring-name authority

For each semantic namespace provided by the package, the normalized snapshot contains an explicit export table:

```text
(namespace_name, authoring_name)
    -> (canonical_id, resource_kind)
```

Rules:

1. within one active namespace provider, one exported authoring name resolves to exactly one canonical ID;
2. multiple export names MAY resolve to the same canonical ID for alias/deprecation use under ADR-0009;
3. canonical ID does not encode namespace name, resource kind, package name, version, or path;
4. resource kind is metadata and SHALL agree with the referenced resource collection;
5. display labels are optional presentation metadata and are not lookup authority;
6. import/declaration order creates no priority.

Bare/qualified resolution continues to follow ADR-0009 after the environment's visible namespace set is established.

## 5. Normalized schema resources

Every exported schema concept with durable cross-document identity SHALL appear exactly once in the resolved resource set under its canonical ID.

Focused package collections are:

```text
entity_types
properties
relations
constraint_definitions
interfaces
interface_implementations
```

`interface_implementations` are keyed semantically by `(entity_type_id, interface_id)` under ADR-0025 rather than receiving a new mandatory canonical schema-resource identity.

Model-instance entities such as a reified ValueDefinition instance are NOT ontology-package schema resources. The package contains the `ValueDefinition` Entity Type definition; actual instances belong to model documents/reference models.

## 6. Minimal common resource identity envelope

For identified schema resources:

```text
SchemaResourceIdentity = {
  id: canonical persistent SOL URI
}
```

Human authoring/API names live in namespace exports, not inside the deepest canonical identity.

A resource MAY carry display metadata, but display metadata does not participate in equality or references.

References between normalized resources use canonical IDs only.

## 7. Minimum kind-specific slices required before package closure

### EntityTypeDefinition

Minimum normalized fields:

```yaml
id: <canonical-id>
is_a: <canonical-entity-type-id>   # optional, at most one
```

No `children` field exists. Interface conformance remains in InterfaceImplementation declarations.

### PropertyDefinition

The minimum v0.1 package slice requires stable identity only:

```yaml
id: <canonical-id>
```

Property value/dimension/unit refinements are expressed through accepted Constraint/Interface/use-site contracts rather than guessed here. More property metadata is deferred unless reference models require it.

### RelationDefinition

Minimum normalized fields:

```yaml
id: <canonical-id>
```

plus accepted relation-semantic metadata that is actually present, such as endpoint contracts, derived semantics, allowed-pair contract, or relation characteristics. Cardinality remains Constraint authority under ADR-0007/0017.

Because the current Core has both ordinary domain/range relations and ADR-0016 allowed-pair relations, the normalized RelationDefinition schema must represent both without interpreting one as the other.

### ConstraintDefinition

Reusable identified ConstraintDefinitions carry canonical `id` plus one accepted normalized family payload and any accepted application metadata such as ADR-0025 `interface_application_target_kinds`.

Inline/local Constraint applications need not be promoted to package resources merely to fit this collection.

### InterfaceDefinition / InterfaceImplementation

Reuse ADR-0025 semantics. Interface canonical `id` is a schema-resource identity. Implementation declarations reference canonical Entity Type, Interface, Property, Relation, and ConstraintDefinition IDs.

## 8. Canonical reference rule

After normalization:

```text
resource-to-resource reference -> canonical ID only
model-instance reference        -> resolved model-instance ID
backend handle                  -> backend/Profile/Adapter layer only
```

An authoring qualified name SHALL NOT remain as the semantic endpoint in a normalized package resource.

## 9. Resolved environment and namespace-provider rule

Package validation occurs inside a resolved environment containing exact package versions.

For every visible semantic namespace:

```text
0 providers -> unresolved if referenced
1 provider  -> valid provider binding
>1 providers -> FAIL: NAMESPACE_PROVIDER_AMBIGUOUS
```

Namespace augmentation/federation remains deferred.

A package MAY provide multiple semantic namespaces; package identity and namespace identity remain distinct.

## 10. Referential completeness

Before a normalized package is accepted:

1. every namespace export target resolves to exactly one resource of the declared kind;
2. every resource canonical ID is unique within the resolved environment;
3. every canonical resource reference resolves exactly once;
4. Entity `is_a` references resolve to EntityTypeDefinitions;
5. Interface/implementation references satisfy ADR-0025 kind requirements;
6. relation endpoint/allowed-pair references resolve to EntityTypeDefinitions;
7. Constraint references resolve to accepted family/ConstraintDefinition contracts;
8. no backend-local identity appears where a canonical schema/model reference is required.

## 11. Source-to-normalized compilation boundary

The compiler conceptually performs:

```text
source fragments
  -> package resolution
  -> semantic namespace visibility
  -> authoring-name resolution
  -> canonical-ID assignment/readback
  -> kind-specific normalization
  -> canonical reference rewrite
  -> referential-completeness validation
  -> normalized immutable package snapshot
```

Initial canonical IDs may be minted by an implementation-defined algorithm, but once stored they SHALL survive local-name/package/path changes under ADR-0009.

## 12. Machine schema slice after acceptance

Add focused schemas:

- `schema/entity-type-definition-v0.1.schema.json`
- `schema/property-definition-v0.1.schema.json`
- `schema/relation-definition-v0.1.schema.json`
- `schema/constraint-definition-v0.1.schema.json`
- `schema/ontology-package-normalized-v0.1.schema.json`

The package schema SHALL reference existing Interface/Constraint slices rather than duplicate them.

Add a semantic helper/test for:

- namespace export uniqueness and alias-to-same-ID behavior;
- one active namespace provider;
- resource ID uniqueness;
- export kind/resource-kind agreement;
- canonical reference resolution;
- Entity `is_a` kind checking;
- InterfaceImplementation referential completeness;
- relation endpoint/allowed-pair resolution;
- package version/reference independence from canonical IDs;
- declaration/import order invariance.

## 13. Required boundary cases

1. rename export name while canonical ID unchanged -> same semantic concept;
2. same canonical ID exported under deprecated alias and new name -> valid if unambiguous;
3. same export name -> two canonical IDs in one namespace -> fail;
4. same namespace -> two active provider packages -> `NAMESPACE_PROVIDER_AMBIGUOUS`;
5. package version changes while unchanged resource ID remains same -> valid identity continuity;
6. resource ID duplicated with conflicting definitions -> fail;
7. relation range points to Property ID -> kind mismatch;
8. InterfaceImplementation mapping requirement points to unresolved PropertyDefinition -> fail;
9. file move with IDs/export table unchanged -> no semantic change;
10. backend-local handle used as normalized canonical resource reference -> fail;
11. declaration order permutation -> identical normalized semantic graph.

## 14. Deferred

- namespace federation/augmentation;
- optional package dependency language;
- package-lock file syntax/content digests;
- final npm/ESM layout;
- complete PropertyDefinition metadata taxonomy;
- advanced relation algebra not already accepted;
- general model-instance document schema;
- Profile/backend mapping package schema beyond what final design-stage reference models require.

## 15. Research verdict

**Ready for independent contract Validation.**
