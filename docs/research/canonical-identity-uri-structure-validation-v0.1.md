# Canonical Identity URI Structure Validation v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** W3C semantic-web guidance, Palantir Ontology, MOOSE, COMSOL, Ansys  
**Purpose:** Decide whether SOL canonical identifiers should include semantic category and whether human-readable namespace/local names should themselves be the canonical stable identity.

## Research question

Candidate URI structures included:

```text
https://simulation-ontology.org/{namespace}/{category}/{local-id}
https://simulation-ontology.org/{namespace}/{local-id}
```

The architecture must remain stable under label changes, semantic-category refinement, ontology/package reorganization, backend evolution, and version changes while allowing human-friendly authoring syntax.

## Evidence

### W3C

W3C guidance for Semantic Web identifiers emphasizes URI simplicity, long-term stability, manageability, and avoiding implementation-specific details. Vocabulary-management guidance also stresses explicit ownership and persistence commitments for URI spaces. This argues against placing volatile structural metadata in canonical identifiers.

### Palantir Ontology

Palantir separates several naming/identity layers:

- a display name for users;
- an API name used programmatically;
- a Resource Identifier (RID) used as unique resource identity in Foundry APIs.

The existence of an opaque RID independent of the API/display name strongly supports separating stable identity from human-readable semantic naming.

### COMSOL

COMSOL separates Label, Tag, and Type. Labels are human-facing and renameable; Tags are programmatic identifiers within model scope; Types identify feature classes. COMSOL documentation explicitly notes that labels can vary while Tags are used for programmatic reference. Combined tags/paths may be required when a local tag is ambiguous. This demonstrates that human naming, scoped programmatic reference, and semantic type/category are distinct axes.

### MOOSE

MOOSE commonly references objects and parameters through structured block/object/parameter paths such as `block/object/name`. These paths are useful native/backend addressing constructs, but their dependence on input-tree structure makes them inappropriate as SOL canonical semantic identities.

### Ansys

Ansys Mechanical exposes object categories (`DataModelObjectCategory`) and, for some objects, separate identifiers. Category is an object classification axis, not a universal canonical identity mechanism. This again supports keeping category outside canonical SOL identity.

## Findings

### 1. Category SHOULD NOT be embedded in canonical identity

A URI such as:

```text
.../entity/ElectronDensity
```

binds identity to the current classification `entity`. If SOL later refines or reorganizes categories without changing the underlying semantic concept, the URI would either need to change or carry stale structural information.

Therefore semantic category belongs to schema metadata, not canonical identity.

### 2. Version SHOULD NOT be embedded in canonical identity

A semantic concept that remains the same across ontology package releases should retain the same identity. Package/version compatibility is a separate concern.

### 3. Namespace/local-name SHOULD be treated as authoring/API identity, not necessarily the deepest canonical identity

A readable identifier such as:

```text
plasma:ElectronDensity
```

is highly useful for authoring, diagnostics, and APIs, but directly encoding namespace ownership and local semantic naming in the deepest identity creates avoidable coupling to package reorganization or local-name evolution.

### 4. Preferred three-layer identity model

```text
Human layer
  label: "Electron Density"

Authoring/API layer
  plasma:ElectronDensity

Canonical identity layer
  persistent opaque URI/RID-like identifier
```

Example candidate:

```text
https://simulation-ontology.org/id/7f4c0c2e-...
```

or another generated persistent identifier under an SOL-owned URI space.

The canonical identifier is normally hidden from authors. The namespace registry resolves `plasma:ElectronDensity` to it.

## Why opaque canonical identity is preferred

It provides the strongest stability across:

- label rename;
- API/local-name rename;
- semantic category changes;
- ontology package relocation;
- repository reorganization;
- version changes.

The cost is reduced human readability, which is acceptable because authors and most APIs operate through qualified names and labels.

## Namespace move semantics

If a concept moves from one package/namespace to another while retaining semantic identity:

```text
old: plasma:ElectronDensity
new: plasma-core:ElectronDensity
```

both qualified names MAY resolve to the same canonical identity during a migration/deprecation period. The canonical URI does not change.

## Candidate rules

### ID1 — Separate human name, qualified name, and canonical identity

SOL SHALL distinguish display labels, authoring/API qualified names, and canonical stable identity.

### ID2 — Opaque persistent canonical identity

Canonical identity SHOULD be a persistent URI minted from an SOL-controlled identity space and SHOULD NOT encode semantic category, package version, repository path, or other volatile architecture details.

### ID3 — Qualified names for authoring

Authors SHOULD normally use local names or namespace-qualified names such as `plasma:ElectronDensity`. Tooling SHALL resolve these to canonical identity.

### ID4 — Category as metadata

Entity/Relation/Property/Interface/etc. category SHALL be represented as semantic/schema metadata rather than encoded into the canonical URI.

### ID5 — Version independence

Package/ontology version SHALL be separate from concept identity.

### ID6 — No silent ambiguity

A bare/local name MAY be resolved automatically only when it is unambiguous in the active namespace/import context. Ambiguity SHALL require qualification or produce a resolution error.

## Result

The validation rejects category-bearing canonical URIs for SOL v0.1 and favors a three-layer identity model:

```text
label
  ↓
qualified authoring/API name
  ↓ resolver
opaque persistent canonical URI
```

The next design task is to define exact name-resolution precedence and namespace-registry semantics.
