# ADR-0009: Identity, Namespace, Package, and Versioning

**Status:** Accepted for SOL v0.1 architecture  
**Date:** 2026-08-20

## Context

SOL needs stable semantic identities that survive human-facing renames, ontology/package reorganization, backend changes, and package upgrades, while keeping ordinary authoring lightweight. It also needs deterministic name resolution and package compatibility rules suitable for an ESM-first TypeScript framework without copying backend-specific release semantics into Core.

Relevant evidence:

- [Canonical identity URI structure validation](../research/canonical-identity-uri-structure-validation-v0.1.md)
- [Name resolution stress test](../research/name-resolution-stress-test-v0.1.md)
- [Version and dependency compatibility study](../research/version-and-dependency-compatibility-study-v0.1.md)
- [Package and semantic namespace boundary](../research/package-and-semantic-namespace-boundary-v0.1.md)
- [ADR-0006: Semantic preservation across backends](0006-semantic-preservation-across-backends.md)
- [ADR-0008: Inheritance and Interface Composition](0008-inheritance-and-interface-composition.md)

## Decision

### 1. Stable canonical semantic identity

Every schema concept that requires durable cross-document identity SHALL have a stable canonical identifier independent of its display label, authoring alias, category, package version, repository path, and backend representation.

Canonical identity SHALL use an opaque/persistent SOL identity URI form conceptually equivalent to:

```text
https://simulation-ontology.org/id/<persistent-id>
```

The exact persistent-id generation algorithm is implementation-defined in v0.1, but changing a human-facing or authoring name SHALL NOT by itself create a new semantic identity.

Canonical identifiers SHALL NOT encode:

- Entity/Relation/Property category;
- ontology/package version;
- repository/file path;
- backend name;
- display label.

### 2. Human-facing and authoring names are separate from identity

SOL distinguishes:

```text
Human label
    -> authoring name
    -> canonical semantic identity
```

A typical authoring name MAY use a compact qualified form such as:

```text
plasma:ElectronDensity
sol:Field
moose:ADDiffusion
```

Users SHOULD NOT need to interact with canonical persistent URIs during ordinary authoring.

### 3. Hybrid name resolution

SOL v0.1 adopts hybrid visibility and deterministic resolution.

For a qualified reference such as `plasma:ElectronDensity`, the resolver SHALL search only the explicitly named semantic namespace.

For a bare reference such as `Field`, the resolver SHALL:

1. collect visible public/exported candidates from the current and imported semantic namespaces;
2. deduplicate candidates by canonical identity;
3. resolve automatically if exactly one canonical candidate remains;
4. report unresolved if no candidate remains;
5. report an ambiguity error if multiple canonical candidates remain.

The resolver SHALL NOT silently choose a candidate based on import order or current-namespace priority.

Import order provides visibility only; it does not create semantic priority.

### 4. Explicit export boundary

A semantic namespace SHALL expose an explicit public/exported symbol boundary. Internal implementation symbols SHALL NOT become bare-name resolution candidates merely because their provider package is loaded.

Selective exposure/aliasing MAY be supported as authoring convenience, but it SHALL NOT weaken the uniqueness and ambiguity rules above.

### 5. Package and semantic namespace are distinct concepts

SOL SHALL NOT equate a distribution package, ESM module, TypeScript namespace, and semantic namespace.

```text
SOL semantic namespace
    != TypeScript namespace
    != ESM module
    != npm package
```

A package MAY provide one or more semantic namespaces. Package structure MAY evolve independently of semantic identity.

For v0.1, a resolved environment SHALL allow at most one active provider package for a given semantic namespace. Namespace federation/augmentation by multiple simultaneously active packages is deferred.

The reference TypeScript implementation SHOULD be ESM-first and MAY use npm package scopes and subpath exports without changing SOL semantic namespaces.

### 6. Three identity layers

SOL distinguishes three identity spaces:

```text
1. Schema Identity
   What semantic concept is this?

        instance_of
             |
             v
2. Model Instance Identity
   Which object is this in a particular simulation model?

        backend binding
             |
             v
3. Backend-local Identity
   How does the selected backend address this object?
```

Example:

```text
plasma:ElectronDensity          # schema concept
model-A:electron-density-1      # SOL model instance
ne                              # MOOSE-local name
```

Schema and model-instance identities SHALL NOT be replaced by backend-local tags, object names, handles, or paths. A backend projection MAY change backend-local identity while preserving schema and model-instance identity.

### 7. Package versioning

SOL-controlled packages SHALL use Semantic Versioning syntax.

For pre-1.0 packages, SOL does not assume compatibility across minor versions. Dependency compatibility SHALL therefore be declared explicitly, for example:

```yaml
package:
  version: 0.3.0

requires:
  sol: ">=0.3.0 <0.4.0"
```

For 1.0 and later, SOL packages SHOULD follow conventional strict SemVer meaning:

- MAJOR: breaking semantic/schema change;
- MINOR: backward-compatible capability addition;
- PATCH: backward-compatible correction.

Version is package metadata and SHALL NOT be embedded in canonical semantic identity.

### 8. Required dependencies only in Core v0.1

Core v0.1 SHALL model required package dependencies only.

Optional integrations SHALL be represented through adapter/profile capabilities rather than a Core optional-dependency and feature-negotiation system.

This avoids turning the ontology Core into a general package manager or capability negotiation framework.

### 9. Backend release compatibility is adapter-owned

External simulation backend release identifiers SHALL remain in backend-native form. SOL SHALL NOT force MOOSE, COMSOL, Ansys, or future backends into SemVer notation.

Examples may include release strings, versioner identifiers, or vendor release labels.

Backend compatibility scopes and matching semantics are adapter-defined. Core SHALL NOT standardize backend-specific scopes such as `model-format`, `api`, `solver`, `module`, or equivalent categories.

Thus:

```text
Semantic Identity
    != SOL Package Version
    != Backend Release Identifier
```

### 10. Minimal alias, rename, and deprecation contract

Core defines only the semantic invariants required for safe identity resolution:

1. canonical identity is stable;
2. human-facing and authoring names MAY change without changing canonical identity;
3. deprecated names MAY resolve to the same canonical identity while emitting a diagnostic;
4. alias resolution MUST NOT introduce ambiguity;
5. backend/model-specific rename, migration, retention periods, fallback behavior, and deprecation mechanics are adapter/profile/package-owned.

Core SHALL NOT prescribe backend-specific tag migration or model-object rename policies.

## Consequences

### Positive

- Semantic identity survives package refactoring, human-facing rename, and backend changes.
- Users can normally author with concise names instead of persistent URIs.
- Ambiguity is detected rather than silently guessed.
- npm/ESM/TypeScript packaging can evolve independently of ontology identity.
- Cross-backend model projection can preserve SOL identities while changing backend-local addressing.
- Backend release semantics remain faithful to each simulation system.
- Core version/dependency logic remains intentionally small.

### Costs

- A namespace registry/resolver is required.
- Canonical persistent identifiers must be generated and stored reliably.
- Resolved environments must detect duplicate namespace providers.
- Package dependency validation and backend compatibility validation remain distinct mechanisms.

## Deferred

This ADR deliberately does not define:

- the persistent-id generation algorithm;
- multi-provider namespace federation/augmentation;
- a Core optional dependency language;
- a generic feature-negotiation system;
- standardized backend compatibility scope names;
- backend-specific alias retention or migration policy;
- globally unique runtime instance URI format beyond the schema/model/backend identity separation.

These require implementation evidence before addition to Core.

## Decision summary

SOL v0.1 separates stable canonical semantic identity from authoring names, packages, versions, model instances, and backend-local identifiers. It adopts hybrid ambiguity-safe name resolution, explicit namespace exports, package/namespace separation with one active namespace provider, ESM-first TypeScript packaging, explicit pre-1.0 SemVer dependency ranges, required dependencies only in Core, and adapter-owned backend release compatibility and rename/deprecation mechanics.
