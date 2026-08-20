# ADR-0028 — Canonical Ontology Package and Resource Integration

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** SOL v0.1 normalized ontology-package/schema integration

## Context

SOL v0.1 has accepted focused machine-readable contracts for Constraints, Interfaces, Value/Unit/Dimension, and Core graph semantics. ADR-0009 requires canonical semantic identity to remain independent of package, namespace, version, local name, path, resource category, and backend identity.

Repository source fragments are useful authoring/consolidation inputs but are not sufficient as the final normalized identity authority.

## Decision

### 1. Source versus normalized package

`ontology/core/*.yaml` and equivalent fragments are authoring/consolidation inputs. The normalized ontology package is the immutable machine snapshot consumed by independent semantic validation and reference-model loading.

File path, mapping key, declaration order, and authoring name are not canonical semantic identity.

### 2. Package, namespace, identity separation

SOL distinguishes:

```text
Distribution package
Semantic namespace
Canonical semantic resource identity
```

These are separate concerns. Package/version/path/category/backend metadata SHALL NOT be encoded into canonical resource IDs.

### 3. Namespace exports

Each provided semantic namespace has an explicit export table:

```text
(namespace, authoring_name) -> (canonical_id, resource_kind)
```

Multiple names may map to one canonical ID for alias/deprecation use. One authoring name may not resolve to multiple canonical IDs within one active provider.

### 4. Resolved package snapshot

A normalized package contains exact resolved dependency versions and focused resource collections:

```text
package
namespaces
resolved_dependencies
entity_types
properties
relations
constraint_definitions
interfaces
interface_implementations
```

All normalized resource-to-resource references use canonical IDs.

### 5. EntityTypeDefinition

Closed focused shape:

```yaml
id: <canonical-id>
is_a: <canonical EntityType ID>   # optional
```

No `children` field exists.

### 6. PropertyDefinition

Focused v0.1 shape is identity-only:

```yaml
id: <canonical-id>
```

Additional property semantics remain expressed through accepted Constraint/Interface/use-site contracts until independently justified.

### 7. RelationDefinition

Closed focused shape:

```text
id
endpoint_contract
source_cardinality_projection?
derived?
transitive?
ownership?
order_semantics?
```

`endpoint_contract` is exactly one of:

- `typed` with optional nonempty canonical EntityType ID sets `domain` and `range`; omission means deliberately open generic endpoint, not unresolved evidence;
- `allowed_pairs` with nonempty canonical source/target EntityType pairs.

Both use `matching = canonical_type_or_subtype`.

Where a source-cardinality projection is serialized, it carries the canonical authoritative ConstraintDefinition ID plus the projected min/max interval. It is not a second semantic Constraint contributor.

### 8. ConstraintDefinition

Reusable identified constraints wrap, rather than modify, accepted closed family payloads:

```yaml
id: <canonical-id>
payload: <one normalized Constraint family payload>
interface_application_target_kinds: [...]   # optional
```

The payload remains unchanged and does not contain identity/application metadata.

### 9. InterfaceDefinition/Implementation

ADR-0025 structures are reused. InterfaceImplementation remains semantically keyed by `(entity_type_id, interface_id)` and receives no mandatory new canonical resource ID.

### 10. Durable canonical identity

Canonical identity comes from an explicit stored source binding or a prior committed normalized identity snapshot. A genuinely new resource with no prior binding may be minted once; the binding must then be persisted.

Identity continuity SHALL NOT be inferred from payload equality, name similarity, file path, declaration order, package position, backend mapping, or structural similarity.

Conflicting candidate prior IDs fail deterministically.

### 11. Identity versus versioned content evolution

Within one resolved immutable environment:

```text
one canonical ID -> one active normalized resource definition
```

Conflicting simultaneous definitions fail.

Across historical/package-version snapshots, the same canonical ID MAY retain semantic identity while its definition evolves. Package compatibility/version rules govern upgrade compatibility; content change alone does not mint a new identity.

### 12. Referential validation

Before package acceptance, validators ensure:

- one active provider per visible semantic namespace;
- export-name uniqueness;
- resource canonical-ID uniqueness/kind agreement;
- canonical reference resolution;
- Entity `is_a` kind correctness;
- Relation endpoint/allowed-pair kind correctness;
- cardinality projection authority/matching;
- ConstraintDefinition family payload validity;
- ADR-0025 Interface referential completeness;
- absence of backend-local identities in canonical semantic reference positions.

## Machine-readable slice

Add:

- `entity-type-definition-v0.1.schema.json`
- `property-definition-v0.1.schema.json`
- `relation-definition-v0.1.schema.json`
- `constraint-definition-v0.1.schema.json`
- `ontology-package-normalized-v0.1.schema.json`

and a focused semantic helper/test for package identity, namespace-provider, kind/reference, projection, and order-invariance rules.

## Deferred

- namespace federation/augmentation;
- optional package dependencies;
- package-lock/content-digest syntax;
- final npm/ESM distribution layout;
- complete PropertyDefinition metadata;
- general model-instance document schema;
- Profile/backend mapping package schema beyond final design-stage reference needs.
