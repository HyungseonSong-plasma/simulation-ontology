import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

from tests.constraint_value_semantics import (
    ValueConstraintError,
    canonicalize_decimal_lexeme,
    classify_empty_result,
    compare_decimal,
    intersect_allowed_sets,
    intersect_interval_and_set,
    intersect_intervals,
    normalize_allowed_set,
    normalize_numeric_interval,
)

ROOT = Path(__file__).resolve().parents[1]
AUTHORING = json.loads((ROOT / "schema" / "constraint-value-authoring-v0.1.schema.json").read_text())
NORMALIZED = json.loads((ROOT / "schema" / "constraint-value-normalized-v0.1.schema.json").read_text())


class ConstraintValueSchemaTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Draft202012Validator.check_schema(AUTHORING)
        Draft202012Validator.check_schema(NORMALIZED)
        cls.authoring_validator = Draft202012Validator(AUTHORING)
        cls.normalized_validator = Draft202012Validator(NORMALIZED)

    def test_authoring_numeric_interval_and_allowed_sets(self):
        self.authoring_validator.validate({
            "type": "value",
            "form": "numeric_interval",
            "lower": {"value": 0, "inclusive": False},
            "upper": {"value": 1, "inclusive": True},
        })
        self.authoring_validator.validate({
            "type": "value",
            "form": "allowed_set",
            "scalar_kind": "string",
            "values": ["a", "b"],
        })

    def test_authoring_heterogeneous_set_is_rejected(self):
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate({
                "type": "value",
                "form": "allowed_set",
                "scalar_kind": "number",
                "values": [1, "1"],
            })
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate({
                "type": "value",
                "form": "allowed_set",
                "scalar_kind": "boolean",
                "values": [True, 1],
            })

    def test_exact_decimal_equivalent_lexemes_canonicalize_equal(self):
        expected = {"coefficient": "1", "exponent10": 0}
        for lexeme in ("1", "1.0", "1e0", "10e-1"):
            with self.subTest(lexeme=lexeme):
                self.assertEqual(canonicalize_decimal_lexeme(lexeme), expected)

    def test_large_adjacent_integers_remain_distinct(self):
        left = canonicalize_decimal_lexeme("9007199254740992")
        right = canonicalize_decimal_lexeme("9007199254740993")
        self.assertNotEqual(left, right)
        self.assertEqual(compare_decimal(left, right), -1)

    def test_signed_zero_has_unique_canonical_form(self):
        expected = {"coefficient": "0", "exponent10": 0}
        for lexeme in ("0", "-0", "0.0", "-0e99"):
            with self.subTest(lexeme=lexeme):
                self.assertEqual(canonicalize_decimal_lexeme(lexeme), expected)

    def test_nonfinite_and_non_json_numeric_extensions_are_rejected(self):
        for lexeme in ("NaN", "Infinity", "-Infinity", ".inf", "0x10"):
            with self.subTest(lexeme=lexeme):
                with self.assertRaisesRegex(ValueConstraintError, "VALUE_NUMBER_INVALID"):
                    canonicalize_decimal_lexeme(lexeme)

    def test_normalized_decimal_schema_requires_canonical_coefficient(self):
        valid = {
            "type": "value",
            "form": "allowed_set",
            "scalar_kind": "number",
            "values": [{"coefficient": "123", "exponent10": -2}],
        }
        self.normalized_validator.validate(valid)
        for coefficient in ("01", "10", "-0"):
            with self.subTest(coefficient=coefficient):
                bad = dict(valid)
                bad["values"] = [{"coefficient": coefficient, "exponent10": 0}]
                with self.assertRaises(ValidationError):
                    self.normalized_validator.validate(bad)
        bad_zero = dict(valid)
        bad_zero["values"] = [{"coefficient": "0", "exponent10": 1}]
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate(bad_zero)

    def test_normalized_exponent_matches_json_schema_integer_value_semantics(self):
        payload = {
            "type": "value",
            "form": "allowed_set",
            "scalar_kind": "number",
            "values": [{"coefficient": "1", "exponent10": 1.0}],
        }
        self.normalized_validator.validate(payload)
        self.assertEqual(
            compare_decimal(
                {"coefficient": "1", "exponent10": 1.0},
                {"coefficient": "1", "exponent10": 1},
            ),
            0,
        )
        for exponent in (1.5, True):
            with self.subTest(exponent=exponent):
                bad = dict(payload)
                bad["values"] = [{"coefficient": "1", "exponent10": exponent}]
                with self.assertRaises(ValidationError):
                    self.normalized_validator.validate(bad)
                with self.assertRaisesRegex(ValueConstraintError, "VALUE_NUMBER_CANONICAL_INVALID"):
                    compare_decimal(
                        {"coefficient": "1", "exponent10": exponent},
                        {"coefficient": "1", "exponent10": 1},
                    )

    def test_open_closed_interval_empty_boundary(self):
        empty = normalize_numeric_interval(
            lower={"value_lexeme": "1", "inclusive": True},
            upper={"value_lexeme": "1", "inclusive": False},
            comparison_space_state="not_required",
        )
        self.assertEqual(empty["status"], "empty")
        singleton = normalize_numeric_interval(
            lower={"value_lexeme": "1.0", "inclusive": True},
            upper={"value_lexeme": "1e0", "inclusive": True},
            comparison_space_state="not_required",
        )
        self.assertEqual(singleton["status"], "satisfiable")
        self.normalized_validator.validate(singleton["payload"])

    def test_interval_intersection_uses_stronger_open_bound(self):
        left = normalize_numeric_interval(
            lower={"value_lexeme": "0", "inclusive": False},
            upper={"value_lexeme": "2", "inclusive": True},
            comparison_space_state="not_required",
        )["payload"]
        right = normalize_numeric_interval(
            lower={"value_lexeme": "0.0", "inclusive": True},
            upper={"value_lexeme": "1", "inclusive": True},
            comparison_space_state="not_required",
        )["payload"]
        result = intersect_intervals(left, right)
        self.assertEqual(result["status"], "satisfiable")
        self.assertFalse(result["payload"]["lower"]["inclusive"])
        self.assertEqual(result["payload"]["upper"]["value"], {"coefficient": "1", "exponent10": 0})

    def test_numeric_allowed_set_deduplicates_exact_decimal_values(self):
        result = normalize_allowed_set(
            scalar_kind="number",
            values=["1", "1.0", "2", "2.00"],
            comparison_space_state="not_required",
        )
        self.assertEqual(result["status"], "satisfiable")
        self.assertEqual(len(result["payload"]["values"]), 2)
        self.normalized_validator.validate(result["payload"])

    def test_empty_authored_set_becomes_empty_semantic_result(self):
        authored = {"type": "value", "form": "allowed_set", "scalar_kind": "string", "values": []}
        self.authoring_validator.validate(authored)
        result = normalize_allowed_set(scalar_kind="string", values=[])
        self.assertEqual(result["status"], "empty")
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate(authored)

    def test_interval_numeric_set_cross_intersection(self):
        interval = normalize_numeric_interval(
            lower={"value_lexeme": "0", "inclusive": False},
            upper={"value_lexeme": "1", "inclusive": True},
            comparison_space_state="not_required",
        )["payload"]
        allowed = normalize_allowed_set(
            scalar_kind="number",
            values=["0", "0.5", "2"],
            comparison_space_state="not_required",
        )["payload"]
        result = intersect_interval_and_set(interval, allowed)
        self.assertEqual(result["status"], "satisfiable")
        self.assertEqual(result["payload"]["values"], [{"coefficient": "5", "exponent10": -1}])

    def test_different_allowed_set_kinds_intersect_empty_without_coercion(self):
        strings = normalize_allowed_set(scalar_kind="string", values=["1"])["payload"]
        booleans = normalize_allowed_set(scalar_kind="boolean", values=[True])["payload"]
        self.assertEqual(intersect_allowed_sets(strings, booleans)["status"], "empty")

    def test_empty_result_conflict_class_depends_on_activation_context(self):
        empty = {"status": "empty"}
        self.assertEqual(classify_empty_result(empty, "static"), "Schema Conflict")
        self.assertEqual(classify_empty_result(empty, "active_conditional"), "Configuration Conflict")
        self.assertIsNone(classify_empty_result(empty, "inactive_conditional"))

    def test_unresolved_comparison_space_prevents_numeric_payload(self):
        with self.assertRaisesRegex(ValueConstraintError, "VALUE_COMPARISON_SPACE_UNRESOLVED"):
            normalize_numeric_interval(
                lower={"value_lexeme": "1", "inclusive": True},
                comparison_space_state="unresolved",
            )
        with self.assertRaisesRegex(ValueConstraintError, "VALUE_COMPARISON_SPACE_UNRESOLVED"):
            normalize_allowed_set(
                scalar_kind="number",
                values=["1"],
                comparison_space_state="unresolved",
            )


if __name__ == "__main__":
    unittest.main()
