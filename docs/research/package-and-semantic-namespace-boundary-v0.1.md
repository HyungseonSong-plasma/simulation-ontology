# Package and Semantic Namespace Boundary v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Primary implementation target:** TypeScript / ECMAScript modules  
**Comparative references:** npm/Node.js packages, TypeScript modules, Python namespace packages

## 1. Research question

Should a SOL package and a semantic namespace be the same thing, or should packaging/distribution be separated from semantic identity?

The design should be friendly to TypeScript/ESM, avoid unnecessary resolver complexity, and remain extensible to bindings and alternate runtimes.

## 2. TypeScript / Node.js observations

Modern TypeScript recommends ECMAScript modules over TypeScript-specific global namespaces. Modules provide isolation, explicit exports/imports, and strong tooling support.

npm scopes group related distribution packages, for example `@simulation-ontology/core` and `@simulation-ontology/plasma`, but the scope is not itself the package. A scope may contain many packages.

Node/TypeScript package resolution also supports explicit package exports/subpath exports, which allows one distribution package to expose multiple public module entry points without treating those entry points as separate distribution identities.

This creates three distinct implementation concepts:

```text
npm scope
  -> groups packages

npm package
  -> distribution/version/dependency unit

ES module / export path
  -> code-level public entry point
```

These should not be collapsed into SOL semantic identity.

## 3. Python comparison

Python namespace packages allow multiple independently installed distributions to contribute modules to one logical namespace. This is powerful for large plugin ecosystems, but it introduces additional coordination and discovery complexity.

For SOL v0.1, allowing several installed packages to contribute competing definitions to the same semantic namespace would complicate deterministic name resolution, package version selection, provenance, and schema conflict diagnostics.

Therefore Python-style multi-distribution namespace merging should not be a baseline SOL v0.1 feature.

## 4. Candidate SOL separation

SOL should distinguish:

```text
DistributionPackage
  -> versioned installable artifact
  -> dependencies
  -> code/runtime exports
  -> provides semantic namespace(s)

SemanticNamespace
  -> semantic names and aliases
  -> public/exported schema symbols
  -> maps authoring names to stable canonical identities
```

A package MAY technically provide more than one semantic namespace, but package identity SHALL NOT determine canonical semantic identity.

Conversely, semantic namespace identity SHALL NOT encode npm package name, repository path, or module path.

## 5. TypeScript-friendly package layout

A likely initial npm layout is:

```text
@simulation-ontology/core
@simulation-ontology/plasma
@simulation-ontology/backend-moose
@simulation-ontology/backend-comsol
@simulation-ontology/backend-ansys
@simulation-ontology/profile-plasma-moose
```

Each package can expose explicit ESM entry points such as:

```text
@simulation-ontology/core
@simulation-ontology/core/schema
@simulation-ontology/core/validation
@simulation-ontology/core/runtime
```

These are implementation module/export paths, not semantic namespaces.

For example:

```text
npm package: @simulation-ontology/plasma
semantic namespace: plasma
schema symbol: plasma:ElectronDensity
canonical identity: persistent SOL URI
```

The package name may later change without changing the semantic identity.

## 6. Namespace provider rule for v0.1

Although package and semantic namespace are separate concepts, SOL v0.1 SHOULD require a deterministic active provider for each semantic namespace in a resolved environment.

```text
resolved environment
  semantic namespace `plasma`
       -> exactly one active package provider
```

This does not mean the namespace and package are conceptually 1:1. It is a resolver constraint intended to prevent ambiguous or conflicting definitions.

Future versions may introduce explicit namespace federation/augmentation if real plugin use cases justify the added complexity.

## 7. Why not enforce permanent 1:1 package/namespace identity

A permanent 1:1 rule would make semantic identity depend too strongly on distribution structure. Common refactors should remain possible:

- split one npm package into several packages;
- merge runtime/helper packages;
- change repository layout;
- move TypeScript code without renaming ontology concepts;
- expose additional language bindings.

Therefore packaging should be allowed to evolve independently of semantic identity.

## 8. Binding implications

The separation also supports bindings.

Conceptually:

```text
SOL semantic namespace / canonical schema
        |          |          |
        v          v          v
 TypeScript SDK  Python SDK  Rust/WASM adapter
```

Each language ecosystem may package the same semantic namespace differently while resolving to the same canonical identifiers.

This is preferable to encoding npm-specific structure into the ontology.

## 9. Candidate rules

### PN1 — Package/namespace separation

A SOL distribution package and a SOL semantic namespace SHALL be distinct concepts.

### PN2 — Package as distribution unit

A package SHALL own versioning, dependency metadata, installation, implementation exports, and runtime artifacts.

### PN3 — Namespace as semantic naming unit

A semantic namespace SHALL own authoring names, aliases, public schema visibility, and mapping to canonical semantic identities.

### PN4 — No packaging data in canonical identity

Canonical semantic identity SHALL NOT depend on npm package name, repository path, module path, or package version.

### PN5 — Deterministic active provider

SOL v0.1 SHALL require at most one active provider for a given semantic namespace in a resolved environment unless an explicit future namespace-augmentation mechanism is introduced.

### PN6 — ESM-first implementation

The TypeScript implementation SHOULD use ECMAScript modules and explicit package exports rather than TypeScript global namespaces as its primary code-organization mechanism.

## 10. Consequences

Positive:

- TypeScript/npm packaging remains idiomatic.
- Semantic identifiers survive package refactors.
- Alternate language bindings can map to the same semantic namespace.
- Resolver semantics remain deterministic.
- The architecture avoids premature Python-style namespace federation complexity.

Cost:

- Package manifests must explicitly declare which semantic namespaces they provide.
- Runtime/package resolver and semantic name resolver remain separate layers.

## 11. Suggested manifest direction

Illustrative only:

```yaml
package:
  name: "@simulation-ontology/plasma"
  version: "0.3.0"

provides:
  namespaces:
    - plasma

requires:
  sol: ">=0.1.0 <0.2.0"
```

The package field is distribution metadata. `plasma` is semantic namespace metadata. Neither determines the persistent canonical IDs of the symbols defined inside the namespace.
