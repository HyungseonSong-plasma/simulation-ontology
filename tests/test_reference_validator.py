import unittest

from tests.reference_validator import (
    PASS, FAIL, BLOCKED, INDETERMINATE,
    aggregate_decisions, validate_closed_envelope, normalize_qrc_bounds, evaluate_qrc,
    comparator_key, resolve_comparator, effect_pairs, retained_pairs, validate_effect_coverage,
    validate_idempotency, validate_action_graph, finish_revision, reevaluate,
    aggregate_representability, canonical_loss_key, execution_permission, validate_backend_target,
)


class ContractHarnessTests(unittest.TestCase):
    def test_closed_mapping_shapes(self):
        rule = {"source": 1, "applicability": 1, "realization": 1, "bindings": 1, "capabilities": 1}
        self.assertEqual(validate_closed_envelope(rule, list(rule))[0], PASS)
        rule["priority"] = 9
        self.assertEqual(validate_closed_envelope(rule, ["source","applicability","realization","bindings","capabilities"])[0], FAIL)
        claim = {"obligation":1,"source":1,"realization":1,"provenance":1,"diagnostics":1}
        self.assertEqual(validate_closed_envelope(claim,["obligation","source","realization","provenance"])[0], FAIL)

    def test_aggregation_precedence(self):
        self.assertEqual(aggregate_decisions([PASS, BLOCKED, FAIL, INDETERMINATE]), FAIL)
        self.assertEqual(aggregate_decisions([PASS, INDETERMINATE]), INDETERMINATE)

    def test_qrc_bounds(self):
        self.assertEqual(normalize_qrc_bounds({"min":1,"max":3,"exact":2})[2], (2,2))
        self.assertEqual(normalize_qrc_bounds({"min":3,"exact":2})[:2], (FAIL,"QRC_EMPTY_INTERVAL"))
        self.assertEqual(normalize_qrc_bounds({"min":-1})[:2], (FAIL,"QRC_BOUND_INVALID"))
        self.assertEqual(normalize_qrc_bounds({"exact":1.5})[:2], (FAIL,"QRC_BOUND_INVALID"))

    def test_qrc_closed_world_stable_identity_multiple_typing_and_diagnostics(self):
        c = {
            "source_identity":"reaction-1",
            "relation":"products",
            "qualifier":{"target_type":"NegativeIonSpecies"},
            "min":1,
        }
        open_result = evaluate_qrc(c,{"complete":False,"ontology_package_versions":{"plasma":"0.1"}})
        self.assertEqual((open_result["decision"],open_result["code"]),(BLOCKED,"QRC_CLOSED_SNAPSHOT_REQUIRED"))
        snap = {
            "complete": True,
            "available_types":["NegativeIonSpecies","OminusType"],
            "edges":["o2","om","om"],
            "target_types":{"o2":["O2Type"],"om":["OminusType","NegativeIonSpecies","ChargedSpecies"]},
            "subtype_closure":{"OminusType":["NegativeIonSpecies"]},
            "ontology_package_versions":{"plasma":"0.1"},
        }
        r = evaluate_qrc(c,snap)
        self.assertEqual(r["decision"],PASS)
        self.assertEqual(r["count"],1)
        self.assertEqual(r["counted"],["om"])
        self.assertEqual(r["diagnostics"]["source_identity"],"reaction-1")
        self.assertEqual(r["diagnostics"]["relation_identity"],"products")
        self.assertEqual(r["diagnostics"]["ontology_package_versions"],{"plasma":"0.1"})

    def test_qrc_qualifier_resolution_boundaries(self):
        c={"relation":"products","qualifier":{"target_type":"NegativeIonSpecies"},"min":1}
        ambiguous={
            "complete":True,
            "qualifier_candidates":{"NegativeIonSpecies":["pkgA:Tneg","pkgB:Tneg"]},
            "available_types":["pkgA:Tneg","pkgB:Tneg"],
        }
        self.assertEqual(evaluate_qrc(c,ambiguous)["code"],"QRC_QUALIFIER_AMBIGUOUS")
        missing={
            "complete":True,
            "qualifier_candidates":{"NegativeIonSpecies":[]},
            "type_provider_resolved":False,
        }
        self.assertEqual(evaluate_qrc(c,missing)["decision"],BLOCKED)
        no_resolver={"complete":True,"available_types":["NegativeIonSpecies"],"subtype_resolver_available":False}
        self.assertEqual(evaluate_qrc(c,no_resolver)["code"],"QRC_SUBTYPE_RESOLVER_UNAVAILABLE")

    def test_comparator_exactly_one_cross_context_and_direction(self):
        bindings = {
            "A":{"component_id":"A","adapter_contract_id":"a","adapter_contract_version":"1"},
            "B":{"component_id":"B","adapter_contract_id":"b","adapter_contract_version":"1"},
            "O":{"component_id":"O","adapter_contract_id":"orch","adapter_contract_version":"1"},
        }
        ea={"id":"ea","resource_component_id":"A","kind":"selection-read"}
        eb={"id":"eb","resource_component_id":"B","kind":"selection-write"}
        symmetric_key = comparator_key(ea,eb,"overlap",bindings,"O",True)
        self.assertEqual(symmetric_key[0],"cross_component")
        self.assertEqual(symmetric_key, comparator_key(eb,ea,"overlap",bindings,"O",True))
        ab = comparator_key(ea,eb,"transfer",bindings,"O",False)
        ba = comparator_key(eb,ea,"transfer",bindings,"O",False)
        self.assertNotEqual(ab,ba)
        self.assertEqual(resolve_comparator([],symmetric_key)["code"],"COMPARATOR_MISSING")
        reg=[{"id":"c1","version":"1","key":symmetric_key}]
        self.assertEqual(resolve_comparator(reg,symmetric_key)["decision"],PASS)
        self.assertEqual(resolve_comparator(reg+[{"id":"c2","version":"1","key":symmetric_key}],symmetric_key)["code"],"COMPARATOR_AMBIGUOUS")

    def test_pair_universe_and_proof_only_pruning_order_independent(self):
        effects=[{"id":"c"},{"id":"a"},{"id":"b"}]
        self.assertEqual(effect_pairs(effects),[("a","b"),("a","c"),("b","c")])
        self.assertEqual(retained_pairs(effects,[("b","a")]),[("a","c"),("b","c")])
        self.assertEqual(retained_pairs(list(reversed(effects)),[("a","b")]),[("a","c"),("b","c")])

    def test_effect_coverage_and_bookkeeping(self):
        declared=[{"fingerprint":"semantic-x","kind":"property-write"}]
        derived=[{"fingerprint":"semantic-x","kind":"property-write"},{"fingerprint":"counter","kind":"tag-counter"}]
        self.assertEqual(validate_effect_coverage(declared,derived,["tag-counter"])["decision"],PASS)
        self.assertEqual(validate_effect_coverage(declared,derived,[])["code"],"EXECUTABLE_EFFECT_MISMATCH")
        duplicate_declared=[{"fingerprint":"semantic-x"},{"fingerprint":"semantic-x"}]
        self.assertEqual(validate_effect_coverage(duplicate_declared,[{"fingerprint":"semantic-x"}],[])["code"],"EFFECT_EQUIVALENCE_AMBIGUOUS")

    def test_idempotency_boundaries(self):
        self.assertEqual(validate_idempotency({"mode":"state_independent","adapter_guarantee":False})["decision"],FAIL)
        sd={"mode":"state_dependent","procedure_match_count":1,"procedure_result":"idempotent"}
        self.assertEqual(validate_idempotency(sd,False)["decision"],BLOCKED)
        sd["procedure_result"]="indeterminate"
        self.assertEqual(validate_idempotency(sd,True)["decision"],INDETERMINATE)
        sd["procedure_match_count"]=2
        self.assertEqual(validate_idempotency(sd,True)["code"],"IDEMPOTENCY_PROCEDURE_AMBIGUOUS")

    def test_action_graph(self):
        actions=[
            {"id":"create-T","produces":["T"],"requires":[]},
            {"id":"time-kernel","produces":["td"],"requires":["T"]},
        ]
        self.assertEqual(validate_action_graph(actions)["decision"],PASS)
        dup=actions+[{"id":"dup","produces":["T"],"requires":[]}]
        self.assertEqual(validate_action_graph(dup)["code"],"DUPLICATE_PRODUCER_UNRESOLVED")
        self.assertEqual(validate_action_graph([{"id":"a","produces":[],"requires":["missing"]}])["code"],"UNRESOLVED_PREREQUISITE")
        cycle=[
            {"id":"a","produces":["A"],"requires":["B"]},
            {"id":"b","produces":["B"],"requires":["A"]},
        ]
        self.assertEqual(validate_action_graph(cycle)["code"],"ACTION_DEPENDENCY_CYCLE")

    def test_immutable_revision_runtime_drift_and_representability(self):
        r1=finish_revision("r1","blocked","snap-1")
        r2=reevaluate(r1,"r2","complete","snap-1")
        r3=reevaluate(r2,"r3","complete","snap-2")
        self.assertEqual(r1.state,"blocked")
        self.assertEqual(r2.prior_revision_id,"r1")
        self.assertEqual(r3.prior_revision_id,"r2")
        self.assertEqual(r2.target_snapshot_fingerprint,"snap-1")
        self.assertEqual(r3.target_snapshot_fingerprint,"snap-2")
        self.assertIsNone(aggregate_representability(PASS,"blocked",["exact"]))
        self.assertIsNone(aggregate_representability(FAIL,"complete",["exact"]))
        self.assertEqual(aggregate_representability(PASS,"complete",["exact","transformed","lossy"]),"lossy")
        self.assertEqual(aggregate_representability(PASS,"complete",["transformed","unsupported"]),"unsupported")

    def test_loss_key_policy_conjunctive_default_deny_and_blocker(self):
        loss=canonical_loss_key("source-1","transport-term","coefficient-provenance")
        policies=[{"loss_key":loss,"decision":"allow"},{"loss_key":loss,"decision":"deny"}]
        self.assertEqual(execution_permission("lossy",[loss],policies),"prohibited")
        self.assertEqual(execution_permission("lossy",[loss],list(reversed(policies))),"prohibited")
        self.assertEqual(execution_permission("lossy",["unlisted"],policies),"prohibited")
        self.assertEqual(execution_permission("lossy",[loss],[{"loss_key":loss,"decision":"allow"}]),"permitted")
        self.assertEqual(execution_permission("exact",[],[],independent_blocker=True),"prohibited")

    def _target_requirement(self):
        return {"components":[
            {
                "component_id":"A",
                "adapter_contract":{"id":"a","version":"1"},
                "release_compatibility":{"allowed":["1"]},
                "required_capabilities":["thermal"],
                "formulation_bindings":[{"sol_obligation":"transient","backend_formulation":"TransientThermal"}],
            },
            {
                "component_id":"O",
                "adapter_contract":{"id":"orch","version":"1"},
                "release_compatibility":{"allowed":["1"]},
                "required_capabilities":["transfer"],
                "formulation_bindings":[],
            },
        ],"orchestration_component_id":"O"}

    def test_backend_target_component_release_capability_formulation_boundaries(self):
        req=self._target_requirement()
        missing={"components":[{
            "component_id":"A","adapter_contract":{"id":"a","version":"1"},"actual_release":"1","capabilities":["thermal"],"available_formulations":["TransientThermal"]
        }]}
        self.assertEqual(validate_backend_target(req,missing)["decision"],BLOCKED)
        resolved={"components":[
            {"component_id":"A","adapter_contract":{"id":"a","version":"1"},"actual_release":"1","capabilities":["thermal"],"available_formulations":["TransientThermal"]},
            {"component_id":"O","adapter_contract":{"id":"orch","version":"1"},"actual_release":"1","capabilities":["transfer"],"available_formulations":[],"orchestration_capable":True},
        ]}
        self.assertEqual(validate_backend_target(req,resolved)["decision"],PASS)
        release_bad={"components":[dict(resolved["components"][0],actual_release="2"),resolved["components"][1]]}
        self.assertEqual(validate_backend_target(req,release_bad)["code"],"TARGET_RELEASE_MISMATCH")
        cap_absent={"components":[dict(resolved["components"][0],capabilities=[]),resolved["components"][1]]}
        self.assertEqual((validate_backend_target(req,cap_absent)["decision"],validate_backend_target(req,cap_absent).get("representability")),(PASS,"unsupported"))
        form_absent={"components":[dict(resolved["components"][0],available_formulations=[]),resolved["components"][1]]}
        self.assertEqual(validate_backend_target(req,form_absent).get("representability"),"unsupported")
        no_orch_cap={"components":[resolved["components"][0],dict(resolved["components"][1],orchestration_capable=False)]}
        self.assertEqual(validate_backend_target(req,no_orch_cap).get("representability"),"unsupported")

    def test_end_to_end_order_independence(self):
        effects1=[{"id":"3"},{"id":"1"},{"id":"2"}]
        effects2=list(reversed(effects1))
        self.assertEqual(effect_pairs(effects1),effect_pairs(effects2))
        self.assertEqual(aggregate_decisions([BLOCKED,PASS,INDETERMINATE]),aggregate_decisions([INDETERMINATE,BLOCKED,PASS]))
        req=self._target_requirement()
        rev_req={"components":list(reversed(req["components"])),"orchestration_component_id":"O"}
        resolved={"components":[
            {"component_id":"A","adapter_contract":{"id":"a","version":"1"},"actual_release":"1","capabilities":["thermal"],"available_formulations":["TransientThermal"]},
            {"component_id":"O","adapter_contract":{"id":"orch","version":"1"},"actual_release":"1","capabilities":["transfer"],"available_formulations":[],"orchestration_capable":True},
        ]}
        self.assertEqual(validate_backend_target(req,resolved),validate_backend_target(rev_req,{"components":list(reversed(resolved["components"]))}))


if __name__ == "__main__":
    unittest.main()
