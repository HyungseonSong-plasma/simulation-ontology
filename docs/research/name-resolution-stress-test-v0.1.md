# Name Resolution Stress Test v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** SOL Core, Plasma ontology, MOOSE, COMSOL, Ansys, Palantir naming/identity patterns  
**Purpose:** Validate deterministic name resolution while allowing simple user-facing authoring.

## 1. Design objective

SOL should let users write simple names when the reference is unambiguous, while preserving stable canonical identities internally.

The system must never silently guess between multiple valid candidates.

## 2. Identity layers

```text
Human-facing label
    "Electron Density"

Authoring/API name
    ElectronDensity
    plasma:ElectronDensity

Canonical identity
    persistent URI / opaque stable identifier
```

Authoring names are resolvable aliases for canonical identities, not canonical identities themselves.

## 3. Evidence

### Palantir

Palantir separates display names and API names from unique resource identifiers (RIDs). Object and interface resources expose an API name and a separate RID. This supports a SOL design where authoring names are convenient references while stable identity remains independent.

### COMSOL

COMSOL model entities expose labels, tags, scopes, and versions as separate metadata. Tags are used for programmatic access and may be unique within a list/context, while labels remain user-facing. Combined tags may be context-dependent. This reinforces the need to distinguish local/contextual references from canonical semantic identity.

### MOOSE

MOOSE input syntax resolves replacement expressions from the current scope through parent scopes and provides fully-qualified paths for explicit addressing. Duplicate parameter declarations are errors unless override is explicitly requested. This supports deterministic scoped resolution and explicit disambiguation rather than silent selection.

### Ansys

Ansys backend object names/categories are implementation-facing addressing/classification mechanisms and should not define SOL canonical identity. Backend adapters may resolve backend-local names only within backend/profile context.

## 4. Stress-test setup

Assume the following loaded namespaces:

```text
current namespace: plasma
imports:
  sol
  moose
  comsol
  ansys
```

Candidate symbols:

```text
sol:Field
plasma:ElectronDensity
plasma:Field
moose:Field
comsol:Field
ansys:Field
```

## 5. Scenario A — unique local name

Input:

```text
ElectronDensity
```

Only `plasma:ElectronDensity` exists.

Result:

```text
ElectronDensity
→ plasma:ElectronDensity
→ canonical identity
```

Valid.

## 6. Scenario B — unique imported name

Input:

```text
BoundaryCondition
```

No local definition exists; exactly one imported namespace exports `sol:BoundaryCondition`.

Result:

```text
BoundaryCondition
→ sol:BoundaryCondition
```

Valid.

## 7. Scenario C — local shadows imported candidate

Suppose:

```text
plasma:Field
sol:Field
```

both exist, and current namespace is `plasma`.

If bare-name resolution gives unconditional precedence to the current namespace, then:

```text
Field → plasma:Field
```

is deterministic but potentially dangerous: importing `sol:Field` later can silently change the intended meaning of old authoring text when a new local `Field` is introduced.

Therefore the safer rule is not lexical shadowing. Bare-name resolution should collect all visible candidates; if more than one canonical identity is compatible with the bare name, the reference is ambiguous.

Result:

```text
Field → ambiguity error
```

User may write:

```text
sol:Field
plasma:Field
```

explicitly.

## 8. Scenario D — same authoring name in multiple imported namespaces

Input:

```text
Field
```

Visible candidates:

```text
sol:Field
moose:Field
comsol:Field
ansys:Field
```

Result:

```text
AMBIGUOUS
```

No import order, declaration order, backend priority, or last-import-wins rule may select one automatically.

## 9. Scenario E — explicitly qualified reference

Input:

```text
moose:ADDiffusion
```

Resolution:

1. resolve prefix `moose` through namespace registry;
2. resolve `ADDiffusion` in that namespace;
3. map authoring identifier to canonical identity.

If exactly one match exists, resolution succeeds regardless of same-named concepts elsewhere.

Qualified references therefore take semantic precedence because they remove search ambiguity rather than because of textual ordering.

## 10. Scenario F — alias

Imports may declare a local alias:

```yaml
imports:
  - namespace: org.simulation-ontology.moose
    as: mf
```

Then:

```text
mf:ADDiffusion
```

resolves to the same canonical identity as the registered MOOSE authoring name.

Aliases are document-local authoring conveniences and are not part of canonical identity.

## 11. Scenario G — rename

Old authoring name:

```text
plasma:ElectronDensity
```

New preferred authoring name:

```text
plasma:ElectronNumberDensityField
```

Both may resolve during a migration window to the same canonical identity if the old name is retained as a deprecated alias.

Thus authoring rename does not require semantic identity change.

## 12. Scenario H — concept moves package/namespace

A concept moves from:

```text
plasma:ElectronDensity
```

to:

```text
plasma-core:ElectronDensity
```

The canonical identity remains unchanged. The previous qualified authoring identifier may remain as an alias/deprecation redirect.

Package location therefore does not define semantic identity.

## 13. Scenario I — backend-local model instances

A COMSOL feature tag such as `ht1` or a MOOSE object name such as `wall_bc` is not resolved by the global schema namespace resolver.

Instead:

```text
schema identifier resolution
    !=
model-instance/backend-local addressing
```

Backend-local names are resolved by a model/profile/backend context after the schema concepts have been resolved.

## 14. Proposed resolution algorithm

For a **qualified reference** `prefix:name`:

```text
1. Resolve prefix via the active namespace/import registry.
2. Look up the authoring name in exactly that namespace.
3. Require exactly one canonical identity.
4. Otherwise report unknown/ambiguous qualified reference.
```

For a **bare reference** `name`:

```text
1. Collect matching names/aliases from the current namespace and explicitly imported namespaces that are visible to the document.
2. Normalize aliases to canonical identities.
3. Deduplicate candidates by canonical identity.
4. If candidate count = 1 → resolve.
5. If candidate count = 0 → unresolved-name error.
6. If candidate count > 1 → ambiguity error; require qualification.
```

Current namespace membership does NOT silently win over imported namespaces.

## 15. Why no local-shadowing rule

Local precedence is convenient in ordinary programming languages, but ontology documents are long-lived semantic assets. Silent rebinding after introducing a new local concept is more dangerous than requiring occasional qualification.

The chosen model optimizes for semantic stability and explainability rather than shortest possible syntax.

## 16. Candidate rules

### NR1 — Qualified references are deterministic

A qualified authoring identifier SHALL resolve only within the namespace denoted by its prefix/alias.

### NR2 — Bare references resolve only when unique

A bare authoring identifier MAY be used only when all visible matches normalize to exactly one canonical identity.

### NR3 — No silent ambiguity resolution

Namespace order, import order, current-namespace priority, backend priority, or declaration order SHALL NOT silently choose among distinct canonical identities.

### NR4 — Aliases are authoring-layer metadata

Namespace aliases and deprecated concept names MAY be used for authoring convenience but SHALL NOT alter canonical identity.

### NR5 — Canonical identity deduplication

Multiple names or aliases resolving to the same canonical identity SHALL count as one candidate during ambiguity checking.

### NR6 — Schema and instance resolution are separate

Schema concept resolution SHALL be distinct from model-instance/backend-local identifier resolution.

### NR7 — Resolution diagnostics

Resolution errors SHOULD report all candidate qualified names and enough provenance to identify the import or namespace that exposed each candidate.

## 17. Result

The stress test supports a user-friendly but strict model:

```text
User may write bare names
        ↓
Resolver gathers visible candidates
        ↓
exactly one canonical identity?
   yes → resolve
   no  → explicit qualification required
```

This avoids semantic drift when ontologies evolve or new backends/namespaces are imported.

## 18. Remaining questions

1. Whether imported namespaces are visible by default or must expose selected symbols explicitly.
2. Whether wildcard imports are allowed in normative SOL authoring.
3. Exact syntax for namespace declarations and aliases.
4. How package version selection participates in namespace resolution.
5. Deprecation lifetime and diagnostics for renamed authoring identifiers.
