# SOL v0.1 remediation proposals v0.2.1 — final contract re-review

**Status:** Independent Validation role review  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Scope:** Contract completeness and deterministic implementability only

## 1. Validation inputs

- `docs/research/sol-v0.1-adr0010-supplement-remediation-proposal-v0.2.1.md`
- `docs/research/sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.2.1.md`
- `docs/validation/sol-v0.1-remediation-proposals-v0.2-contract-re-review.md`
- ADR-0007 and ADR-0010 as accepted baseline contracts

Research conclusions were not treated as authoritative. The gate used here is: for the same immutable model, Profile, adapter contract/version, target snapshot, registry contents, and required evidence, conforming independent validators must have one normative route to the same decision. Executable evidence is assessed separately.

## 2. Backend sanity evidence

The contract direction remains compatible with official backend structures:

- MOOSE Actions execute registered tasks and tasks may have explicit dependencies; this supports action/task dependency semantics rather than backend-object DAG semantics: https://mooseframework.inl.gov/moose/source/actions/Action.html
- COMSOL `PhysicsFeature` exposes feature creation, selections, and parameter mutation as distinct API operations, supporting action/effect lowering outside Core: https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/api/com/comsol/model/physics/PhysicsFeature.html and https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/application_programming_guide.15.25.html
- Ansys System Coupling models data transfer between explicitly identified participant sides and quantities; cross-component operations therefore have pair/direction context that cannot safely be reduced to an arbitrary single component identity: https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/pdf/System_Coupling_Users_Guide.pdf

None of these backend-native object models needs to enter SOL Core.

## 3. Previous counterexample closure

| Previous finding | v0.2.1 result | Verdict |
|---|---|---|
| Effect candidate pruning | all unordered pairs retained unless proven disjoint | Resolved |
| Comparator zero/multiple match | exactly-one registry resolution required | Resolved for single-component comparison |
| Executable descriptor/effect equivalence | deterministic `describe_effects` + uniquely bound equivalence comparator | Resolved |
| Bookkeeping classification | adapter-contract-owned and versioned | Resolved |
| State-independent/state-dependent idempotency | adapter guarantee/procedure required | Resolved |
| Atomic compound action | semantic all-or-none visibility required | Resolved |
| FAIL with BLOCKED/INDETERMINATE | `FAIL > BLOCKED > INDETERMINATE > PASS` | Resolved |
| Loss-policy composition | canonical loss key + conjunctive policy | Resolved |
| Composite target field association | component-keyed release/capability/formulation/runtime records | Resolved |
| QRC open/schema-only model-instance evaluation | canonical `BLOCKED: QRC_CLOSED_SNAPSHOT_REQUIRED` | Resolved |
| QRC mixed `exact/min/max` | interval intersection; empty interval has canonical FAIL | Resolved |
| QRC qualifier evidence boundary | provenance-based FAIL/BLOCKED distinction | Resolved |

## 4. Remaining direct counterexample

### C-21-01 — Cross-component comparator binding is not uniquely defined

`RE21-I2` binds comparison procedures with a registry key containing one `target_component_id`:

```text
(adapter_contract_id,
 adapter_contract_version,
 target_component_id,
 comparison_purpose,
 normalized_left_kind,
 normalized_right_kind)
```

At the same time, `RE21-I1` requires the pair universe over all plan effects, and `BT21-I2` permits composite targets and cross-component transfer actions. Consider effects `EA` and `EB` whose canonical resources belong to different target components A and B. The validator must establish whether the pair is disjoint/overlapping and possibly compare compatibility before pruning or accepting it.

The contract does not state which single `target_component_id` is used to resolve that comparison: A, B, the orchestration component, or another pair-level binding. Two validators using the same immutable plan and registry can therefore select different registry keys, or one can reject while another delegates to the orchestrator. This directly violates the validator-independence gate.

This is not merely theoretical. Coupled backends can define transfer semantics in terms of source and target participants; Ansys System Coupling explicitly identifies the two participant sides for a data transfer. A pair-aware comparison context is therefore required for a composite MappingPlan.

**Classification:** Architecture defect — normative MappingPlan/Adapter contract gap.  
**Scope:** ADR-0010 supplement only; no Core expansion required.  
**Required correction:** define canonical comparison scope for effects from different components. A minimal solution is to make comparator resolution explicitly distinguish `local(component_id)` from `cross_component(canonical component pair, orchestration_component_id)` and bind the latter to exactly one versioned orchestration comparator. Equivalent internal representation is acceptable if it produces one canonical registry key. No validator may choose A/B/orchestrator heuristically.

## 5. New-regression check

No additional contract regression was found in the reviewed scope. In particular:

- complete pair enumeration is conservative but deterministic;
- all effect/action/backend operation vocabulary remains MappingPlan/Profile/Adapter-owned;
- the accepted five-field `MappingRule` and four-field `MappingClaim` public shapes still contain enough top-level semantic information;
- QRC remains a narrow extension of Cardinality rather than a generic collection query language;
- QRC subtype algebra and stable-identity set counting are deterministic at contract level.

## 6. Proposal verdicts

| Target | Contract completeness | Execution readiness | Verdict |
|---|---|---|---|
| ADR-0010 supplement v0.2.1 | one cross-component comparator ambiguity remains | not executable-validated | **Revise** |
| QRC proposal v0.2.1 | no unresolved contract ambiguity found | not executable-validated | **Accept (contract level)** |
| MappingRule 5-field | maintainable | requires executable validation | **Accept (contract level)** |
| MappingClaim 4-field | maintainable | requires executable validation | **Accept (contract level)** |

## 7. Gate decision

- ADR drafting for the ADR-0010 supplement: **blocked pending one limited contract revision**.
- Follow-up QRC Constraint ADR drafting: **contract-ready**, but may be held for synchronized Decision after ADR-0010 closes.
- Architecture freeze: **not permitted**.
- Executable validation: **not yet the next step for ADR-0010**, because the cross-component comparator selection contract is still ambiguous.

## 8. Required next state

Return only C-21-01 to Research. Do not reopen already resolved sections. Produce a v0.2.2 limited revision that defines deterministic local versus cross-component comparison-context resolution, including zero/multiple match behavior and a positive/negative composite-target boundary case. Then run a focused final contract re-review on that delta. If accepted, proceed to Decision for ADR drafting and executable validation planning.
