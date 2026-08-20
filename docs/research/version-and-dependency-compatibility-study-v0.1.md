# Version and Dependency Compatibility Study v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Define SOL package-version and backend-release compatibility semantics using evidence from three simulation ecosystems.

## 1. Cross-backend evidence

### MOOSE

MOOSE does not use traditional product-style semantic versioning for the framework itself and is under continuous development. MOOSE-based applications are expected to use the specific dependency/development-environment version required by the application. The `versioner.py` mechanism ties MOOSE and dependency versions together, and using an incorrect environment version may produce warnings or build failures.

This suggests that exact or tightly pinned backend compatibility matters for some adapters, and that backend release identifiers cannot be assumed to follow SemVer.

### COMSOL

COMSOL uses product releases such as 6.3 and 6.4 and explicitly documents backward-compatibility and migration behavior. Older model files can often be migrated automatically, while generated Java/API code may require explicit changes and is not always backward compatible.

This shows that compatibility is not a single Boolean property of a product release: file/model compatibility, API compatibility, and semantic/default-behavior compatibility may differ.

### Ansys

Ansys uses release identifiers such as 2025 R1 / 2025 R2. Official verification/project files from a previous release are commonly documented as compatible with the latest release, but release identifiers are not SemVer and product/component compatibility can vary.

This supports treating backend release identifiers as backend-native opaque identifiers with adapter-defined ordering/matching semantics.

## 2. Design separation

SOL should separate two versioning domains:

```text
SOL package versioning
        !=
backend release compatibility
```

### SOL package versioning

Applies to:

- SOL Core
- domain ontologies
- backend ontologies/adapters
- profiles
- optional extension packages

These packages are controlled by the SOL ecosystem and MAY use SemVer consistently.

### Backend release compatibility

Applies to external systems such as:

```text
MOOSE commit/versioner environment
COMSOL 6.4
Ansys 2025 R2
```

These identifiers SHALL remain backend-native and SHALL NOT be rewritten into fake SemVer values.

## 3. Candidate SOL package syntax

```yaml
package:
  id: plasma
  version: 0.3.0

requires:
  sol: ">=0.1.0 <0.2.0"
  thermal: ">=0.2.0 <1.0.0"
```

A package version is separate from the stable semantic identities defined by that package.

```text
plasma:ElectronDensity
```

keeps the same canonical identity across compatible package revisions unless the concept itself is intentionally replaced by a new semantic identity.

## 4. Backend compatibility syntax

A backend ontology/profile MAY declare support independently of SOL package dependencies.

Candidate form:

```yaml
backend:
  id: comsol
  releases:
    compatible:
      - ">=6.3 <6.5"
```

However, because backend identifiers are not uniformly SemVer, SOL should not define one universal numeric grammar for every backend.

Preferred general form:

```yaml
backend:
  id: ansys
  releases:
    matcher: ansys-release
    compatible:
      - "2025 R1"
      - "2025 R2"
```

```yaml
backend:
  id: moose
  releases:
    matcher: moose-versioner
    compatible:
      - "2026.07.30"
```

The backend adapter owns interpretation of its release identifiers and matcher semantics.

## 5. Compatibility dimensions

Evidence from COMSOL shows that one generic `compatible: true/false` flag is insufficient. A backend adapter MAY distinguish compatibility dimensions such as:

```text
model-format
api
semantic-behavior
runtime/build-environment
```

For v0.1, SOL Core should not standardize a large compatibility taxonomy. Instead, a backend profile MAY expose named capability/compatibility channels where evidence requires them.

Example:

```yaml
backend:
  id: comsol
  releases:
    matcher: comsol-release
    compatibility:
      model-format:
        - ">=6.3 <6.5"
      api:
        - "6.4"
```

The exact matcher remains adapter-owned.

## 6. Candidate dependency rules

### V1 — Separate semantic identity from package version

A semantic concept's canonical identity SHALL NOT embed the package version by default.

### V2 — SOL-controlled packages use SemVer

SOL Core, domain ontologies, backend ontologies/adapters, profiles, and extension packages SHOULD use Semantic Versioning for package compatibility declarations.

### V3 — Backend-native release identifiers remain native

External backend release identifiers SHALL be preserved in their native representation and SHALL NOT be coerced into SOL package SemVer.

### V4 — Adapter-owned backend release matcher

Each backend adapter/profile SHALL define or reference a deterministic matcher for backend release identifiers when compatibility ranges or ordering are needed.

### V5 — Explicit dependency ranges

A SOL package SHALL explicitly declare compatible versions of required SOL packages rather than assuming latest-version compatibility.

### V6 — No silent compatibility migration

When a dependency/backend release lies outside the declared compatibility contract, the system SHALL report the mismatch rather than silently treating a newer or older version as compatible.

### V7 — Compatibility is scoped

Where a backend distinguishes model-file, API, runtime, or semantic/default-behavior compatibility, profiles MAY express those scopes separately rather than collapsing them into one Boolean.

## 7. Why a single version grammar is rejected

A universal syntax such as:

```text
backend >= 1.2.3
```

would incorrectly assume all simulation backends use SemVer-like ordered releases.

The three reference systems already contradict that assumption:

- MOOSE may be tied to continuous development/versioner environment identifiers;
- COMSOL uses dotted product releases and explicit migration semantics;
- Ansys uses year + R1/R2 release identifiers.

Therefore SOL standardizes **the existence and location of compatibility contracts**, not the internal version grammar of every external backend.

## 8. Candidate package manifest

```yaml
package:
  id: plasma-moose-profile
  version: 0.1.0

requires:
  sol: ">=0.1.0 <0.2.0"
  plasma: ">=0.3.0 <0.4.0"
  moose: ">=0.1.0 <0.2.0"

backend:
  id: moose
  releases:
    matcher: moose-versioner
    compatible:
      - "2026.07.30"
```

A COMSOL profile could instead declare:

```yaml
package:
  id: plasma-comsol-profile
  version: 0.1.0

requires:
  sol: ">=0.1.0 <0.2.0"
  plasma: ">=0.3.0 <0.4.0"
  comsol: ">=0.1.0 <0.2.0"

backend:
  id: comsol
  releases:
    matcher: comsol-release
    compatibility:
      model-format:
        - ">=6.3 <6.5"
      api:
        - "6.4"
```

## 9. Remaining questions

1. Whether SOL v0.x should enforce full SemVer semantics or a minimal `major.minor.patch` compatibility subset.
2. Whether optional dependencies are needed in v0.1.
3. Whether backend compatibility dimensions should have a small Core vocabulary or remain adapter-defined initially.
4. Whether lockfiles/resolved manifests belong in framework implementation rather than ontology architecture.
