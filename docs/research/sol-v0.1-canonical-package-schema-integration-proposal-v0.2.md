# SOL v0.1 Canonical Package / Schema Integration Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Input:** PKG-01 through PKG-03  
**Base:** `docs/research/sol-v0.1-canonical-package-schema-integration-proposal-v0.1.md`

## 1. Preserved decisions

Unchanged:

- source fragments are authoring/consolidation inputs, not canonical identity authority;
- package != semantic namespace != canonical semantic identity;
- normalized references use canonical IDs only;
- namespace export tables own authoring-name resolution;
- one active package provider per semantic namespace in v0.1;
- normalized dependencies use exact resolved versions;
- package/version/path/category/backend identity do not enter canonical semantic IDs;
- model-instance ValueDefinition instances are model data, not ontology-package schema resources;
- InterfaceImplementation remains keyed by `(entity_type_id, interface_id)`.

This revision closes only RelationDefinition shape, ConstraintDefinition wrapper shape, and durable canonical-ID readback.

## 2. PKG-01 — exact focused RelationDefinition shape

Normalized RelationDefinition is a closed object:

```text
RelationDefinition = {
  id,
  endpoint_contract,
  source_cardinality_projection?,
  derived?,
  transitive?,
  ownership?,
  order_semantics?
}
```

### 2.1 Endpoint contract union

Exactly one endpoint form is used.

#### Typed endpoint contract

```yaml
endpoint_contract:
  kind: typed
  matching: canonical_type_or_subtype
  domain: [<canonical EntityType IDs>]   # optional
  range:  [<canonical EntityType IDs>]   # optional
```

Rules:

- `domain` and `range`, when present, are nonempty order-independent unique-ID sets;
- omitted `domain` means deliberately open to any resolved semantic Entity Type at generic relation level, not unresolved evidence;
- omitted `range` has the analogous meaning;
- use-site/Interface/Profile Type constraints may narrow this generic open endpoint conjunctively;
- at least one of domain/range SHOULD normally be present, but both may be omitted only when a relation's generic semantic vocabulary genuinely imposes no endpoint type restriction;
- all references are canonical EntityTypeDefinition IDs.

This permits ADR-0026 `has_value_definition` to have open generic domain + `range=[ValueDefinition]`, and `depends_on` to have `domain=[ValueDefinition]` + open generic range without inventing an `Entity` metatype resource.

#### Allowed-pair endpoint contract

```yaml
endpoint_contract:
  kind: allowed_pairs
  matching: canonical_type_or_subtype
  pairs:
    - source: <canonical EntityType ID>
      targets: [<canonical EntityType IDs>]
```

Rules:

- `pairs` is nonempty;
- each target set is nonempty and order-independent;
- same canonical source appearing more than once is normalized by union then canonical sorting/deduplication;
- derived domain/range summaries SHALL NOT broaden the pair authority;
- `domain`/`range` fields do not coexist with this branch.

ADR-0016 `includes_component` uses this form. `applied_to` uses typed domain/range sets.

### 2.2 Source cardinality projection wrapper

Where a relation carries a frozen machine projection of a canonical Cardinality Constraint:

```yaml
source_cardinality_projection:
  constraint: <canonical ConstraintDefinition ID>
  min: 0
  max: unbounded
```

Rules:

1. `constraint` SHALL resolve to a Cardinality ConstraintDefinition whose payload targets this relation in source direction;
2. `min/max` SHALL exactly equal that normalized payload;
3. missing/mismatch follows ADR-0017 projection diagnostics where the projection is required;
4. this object is a cache/projection and never a second Constraint contributor.

`max` is nonnegative integer or `unbounded` and numeric `min <= max` when finite.

### 2.3 Other accepted metadata

Focused optional fields are closed to:

```text
derived: boolean
transitive: boolean
ownership: non_owning_reference
order_semantics: none
```

Absence means the package does not assert that characteristic. No additional arbitrary relation metadata field is admitted in v0.1 normalized package schema.

The derivation algorithm/text of a derived relation remains an accepted semantic contract outside this structural slice; `derived: true` marks that graph occurrences are derived rather than authoritative stored truth.

## 3. PKG-02 — exact ConstraintDefinition wrapper

Reusable identified constraints are represented as a wrapper, never by injecting identity metadata into closed family payloads:

```yaml
id: <canonical ConstraintDefinition ID>
payload: <exactly one accepted normalized Constraint family payload>
interface_application_target_kinds:   # optional ADR-0025 metadata
  - property_requirement
```

Closed wrapper fields:

```text
id
payload
interface_application_target_kinds?
```

`payload` is exactly one of:

- normalized Cardinality;
- normalized Type;
- normalized Dimension;
- normalized Value;
- normalized Compatibility;
- normalized Conditional.

The family payload remains unchanged and does not contain `id` or wrapper metadata.

`interface_application_target_kinds`, when present, is a nonempty unique subset of:

```text
interface | property_requirement | relation_requirement
```

No Interface application metadata is inferred from family name.

## 4. PKG-03 — durable canonical-ID binding/readback

Canonical ID minting and reuse follow an explicit identity-source rule.

### 4.1 Allowed identity sources

For each authored schema resource compilation, canonical identity comes from exactly one compatible durable source:

1. an explicit stored `canonical_id`/identity binding attached to the source resource; or
2. a prior committed normalized package/identity snapshot supplied as the compiler's identity readback source.

If neither source contains an existing binding for a genuinely new resource, the compiler MAY mint one new canonical ID and SHALL persist that binding before the resource is treated as stable.

### 4.2 No heuristic identity inference

The compiler SHALL NOT infer semantic identity continuity from:

- identical payload;
- similar label/name;
- file path;
- declaration order;
- package position;
- backend mapping;
- structural similarity.

A rename/move that is intended to preserve identity must explicitly reuse the prior canonical ID through source binding or prior identity snapshot/rename metadata.

### 4.3 Conflict rules

```text
no prior binding + new declaration
    -> mint once, persist

one prior compatible binding
    -> reuse exactly

multiple distinct candidate canonical IDs
    -> FAIL: CANONICAL_ID_BINDING_AMBIGUOUS

source canonical_id conflicts with prior committed binding
    -> FAIL: CANONICAL_ID_BINDING_CONFLICT

same canonical ID bound to two conflicting normalized resource definitions
    -> FAIL: CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT
```

A package/file/version rename does not by itself change the canonical ID.

### 4.4 Identity snapshot role

A committed normalized package may serve as the durable identity readback snapshot because it stores:

- canonical resource IDs;
- resource kinds;
- namespace export-name mappings.

Tooling MAY additionally materialize a dedicated identity-lock file, but that file is an implementation convenience and not a second authority. If both exist, they must agree.

## 5. Focused kind-specific resource shapes

### EntityTypeDefinition

```yaml
id: <canonical-id>
is_a: <canonical EntityTypeDefinition ID>   # optional
```

Closed fields: `id`, optional `is_a`.

### PropertyDefinition

```yaml
id: <canonical-id>
```

Identity-only focused v0.1 slice. Additional property semantics come from Constraints/Interfaces/use-site contracts until independently accepted.

### RelationDefinition

Uses Section 2 exact shape.

### ConstraintDefinition

Uses Section 3 wrapper.

### InterfaceDefinition / InterfaceImplementation

Reuse ADR-0025 schema semantics with canonical references.

## 6. Normalized package envelope

The v0.1 normalized package remains:

```text
OntologyPackageNormalized = {
  package: {name, version},
  namespaces: NamespaceProvider[],
  resolved_dependencies: ResolvedDependency[],
  entity_types: EntityTypeDefinition[],
  properties: PropertyDefinition[],
  relations: RelationDefinition[],
  constraint_definitions: ConstraintDefinition[],
  interfaces: InterfaceDefinition[],
  interface_implementations: InterfaceImplementation[]
}
```

All collections have order-independent semantic meaning. A canonical serialization MAY sort them for reproducible hashing/diffing, but order is not semantic.

`package.version` and resolved dependency versions use SemVer strings for SOL-controlled packages; semantic validation of exact SemVer syntax belongs to package schema/tooling and not canonical resource identity.

## 7. Referential validation order

```text
1. load exact resolved package environment
2. enforce one active provider per semantic namespace
3. load durable canonical-ID bindings / prior snapshot
4. validate namespace export-table uniqueness
5. build resource index by canonical ID and kind
6. validate resource ID uniqueness/content consistency
7. validate Entity is_a references/kinds
8. validate Relation endpoint canonical references and allowed pairs
9. validate cardinality projection -> ConstraintDefinition authority
10. validate ConstraintDefinition family payloads and application metadata
11. validate InterfaceDefinition/Implementation references under ADR-0025
12. publish immutable normalized package snapshot
```

## 8. Required focused boundary cases

### PKG-01

1. typed endpoint with range omitted -> deliberately open range, not unresolved;
2. `includes_component` allowed-pair branch -> valid without domain/range branch;
3. typed + allowed-pairs fields mixed -> structural fail;
4. relation range resolves to PropertyDefinition -> kind fail;
5. source cardinality projection points to wrong relation Constraint -> fail;
6. projection equals authoritative Cardinality payload -> pass.

### PKG-02

7. wrapper `id + payload` with Cardinality payload -> pass;
8. `id` injected into closed family payload -> structural fail;
9. wrapper target-kind metadata unknown value -> fail;
10. target-kind metadata omitted -> valid when Interface application is not claimed.

### PKG-03

11. clean rebuild with prior snapshot -> same IDs;
12. local name rename with explicit prior ID reuse -> same semantic identity;
13. identical payload under new declaration without identity binding -> new identity, not auto-merge;
14. source/prior ID disagreement -> fail;
15. two prior IDs claim one current declaration -> ambiguous fail;
16. package/file move with binding unchanged -> identity unchanged.

## 9. Finding closure

| Finding | Proposed status | Resolution |
|---|---|---|
| PKG-01 RelationDefinition shape | Resolved | closed typed-vs-allowed-pairs endpoint union + optional projection/accepted metadata |
| PKG-02 ConstraintDefinition wrapper | Resolved | external `{id,payload,interface_application_target_kinds?}` wrapper |
| PKG-03 durable identity source | Resolved | explicit source/prior-snapshot binding, mint-once persistence, no heuristic continuity |

## 10. Research verdict

**Ready for focused independent Validation.**
