import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

from tests.constraint_compatibility_semantics import (
    CompatibilityConstraintError,
    aggregate_states,
    compose_obligations,
    evaluate_obligation,
    normalize_payload,
    obligation_key,
    resolve_criterion,
)

ROOT = Path(__file__).resolve().parents[1]
AUTHORING = json.loads((ROOT / "schema" / "constraint-compatibility-authoring-v0.1.schema.json").read_text())
NORMALIZED = json.loads((ROOT / "schema" / "constraint-compatibility-normalized-v0.1.schema.json").read_text())

ORDERED = {
    "id": "criterion:ordered",
    "family": "compatibility",
    "arity": 2,
    "operand_order": "ordered",
    "evaluator_contract": "eval:ordered",
}
SYMMETRIC = {
    "id": "criterion:symmetric",
    "family": "compatibility",
    "arity": 2,
    "operand_order": "symmetric",
    "evaluator_contract": "eval:symmetric",
}
A = {"identity_space": "model_instance", "id": "model:A"}
B = {"identity_space": "model_instance", "id": "model:B"}
SCHEMA_A = {"identity_space": "schema", "id": "model:A"}


class CompatibilityConstraintTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Draft202012Validator.check_schema(AUTHORING)
        Draft202012Validator.check_schema(NORMALIZED)
        cls.authoring_validator = Draft202012Validator(AUTHORING)
        cls.normalized_validator = Draft202012Validator(NORMALIZED)

    def authored(self, criterion="criterion:symmetric", left="A", right="B", expect="compatible"):
        return {
            "type": "compatibility",
            "criterion": criterion,
            "left": left,
            "right": right,
            "expect": expect,
        }

    def normalized(self, criterion="criterion:symmetric", left=A, right=B, expect="compatible"):
        return normalize_payload(
            self.authored(criterion=criterion, expect=expect),
            canonical_criterion=criterion,
            left_ref=left,
            right_ref=right,
        )

    def test_authoring_exact_fields(self):
        self.authoring_validator.validate(self.authored())
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate(dict(self.authored(), backend="moose"))

    def test_normalized_rejects_backend_identity_space(self):
        payload = self.normalized()
        payload["left"] = {"identity_space": "backend", "id": "u"}
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate(payload)

    def test_schema_and_model_instance_same_text_id_are_distinct(self):
        left = self.normalized(left=A, right=B)
        right = self.normalized(left=SCHEMA_A, right=B)
        self.assertNotEqual(obligation_key(left, SYMMETRIC), obligation_key(right, SYMMETRIC))

    def test_criterion_resolution_boundaries(self):
        self.assertEqual(resolve_criterion("criterion:symmetric", [SYMMETRIC])["id"], "criterion:symmetric")
        with self.assertRaisesRegex(CompatibilityConstraintError, "COMPATIBILITY_CRITERION_UNRESOLVED"):
            resolve_criterion("missing", [SYMMETRIC])
        with self.assertRaisesRegex(CompatibilityConstraintError, "COMPATIBILITY_CRITERION_AMBIGUOUS"):
            resolve_criterion("criterion:symmetric", [SYMMETRIC, dict(SYMMETRIC)])
        bad_family = dict(SYMMETRIC, family="type")
        with self.assertRaisesRegex(CompatibilityConstraintError, "COMPATIBILITY_CRITERION_FAMILY_INVALID"):
            resolve_criterion("criterion:symmetric", [bad_family])

    def test_symmetric_pair_is_unordered_without_sorting(self):
        ab = self.normalized(left=A, right=B)
        ba = self.normalized(left=B, right=A)
        self.assertEqual(obligation_key(ab, SYMMETRIC), obligation_key(ba, SYMMETRIC))

    def test_symmetric_self_pair_preserves_two_members(self):
        aa = self.normalized(left=A, right=A)
        key = obligation_key(aa, SYMMETRIC)
        counts = dict(key[2])
        self.assertEqual(counts[("model_instance", "model:A")], 2)

    def test_ordered_pair_preserves_direction(self):
        ab = self.normalized(criterion="criterion:ordered", left=A, right=B)
        ba = self.normalized(criterion="criterion:ordered", left=B, right=A)
        self.assertNotEqual(obligation_key(ab, ORDERED), obligation_key(ba, ORDERED))

    def test_same_obligation_same_expect_deduplicates(self):
        payload = self.normalized()
        composed = compose_obligations([(payload, SYMMETRIC), (dict(payload), SYMMETRIC)])
        self.assertEqual(len(composed), 1)

    def test_opposite_expectations_conflict(self):
        yes = self.normalized(expect="compatible")
        no = self.normalized(expect="incompatible")
        with self.assertRaisesRegex(CompatibilityConstraintError, "COMPATIBILITY_OBLIGATION_CONFLICT"):
            compose_obligations([(yes, SYMMETRIC), (no, SYMMETRIC)])

    def test_different_criteria_remain_distinct(self):
        a = self.normalized(criterion="criterion:symmetric")
        b = self.normalized(criterion="criterion:ordered")
        composed = compose_obligations([(a, SYMMETRIC), (b, ORDERED)])
        self.assertEqual(len(composed), 2)

    def test_unresolved_semantic_context_is_indeterminate_with_code(self):
        payload = self.normalized()
        result = evaluate_obligation(payload, semantic_context_resolved=False, binary_result=None)
        self.assertEqual(result["state"], "INDETERMINATE")
        self.assertEqual(result["code"], "COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED")

    def test_binary_evaluation_matches_expectation(self):
        yes = self.normalized(expect="compatible")
        no = self.normalized(expect="incompatible")
        self.assertEqual(evaluate_obligation(yes, semantic_context_resolved=True, binary_result="compatible")["state"], "PASS")
        self.assertEqual(evaluate_obligation(yes, semantic_context_resolved=True, binary_result="incompatible")["state"], "FAIL")
        self.assertEqual(evaluate_obligation(no, semantic_context_resolved=True, binary_result="incompatible")["state"], "PASS")

    def test_family_aggregation(self):
        self.assertEqual(aggregate_states([]), "PASS")
        self.assertEqual(aggregate_states(["PASS", "PASS"]), "PASS")
        self.assertEqual(aggregate_states(["PASS", "INDETERMINATE"]), "INDETERMINATE")
        self.assertEqual(aggregate_states(["FAIL", "INDETERMINATE"]), "FAIL")
        self.assertEqual(aggregate_states(["FAIL", "PASS"]), "FAIL")

    def test_backend_metadata_is_not_semantic_input(self):
        payload = self.normalized()
        baseline = evaluate_obligation(payload, semantic_context_resolved=True, binary_result="compatible")
        backend_state = {"installed": False, "licensed": False, "release": None}
        self.assertEqual(baseline["state"], "PASS")
        self.assertTrue(backend_state)


if __name__ == "__main__":
    unittest.main()
