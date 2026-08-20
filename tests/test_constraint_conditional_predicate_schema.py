import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource

from tests.constraint_predicate_semantics import (
    PredicateEvaluationError,
    activate_conditional,
    aggregate_validation_states,
    evaluate_boolean,
    evaluate_predicate,
    failure,
    resolve_binding,
    truth,
)
from tests.constraint_value_semantics import canonicalize_decimal_lexeme

ROOT = Path(__file__).resolve().parents[1]

SCHEMA_FILES = [
    "constraint-cardinality-authoring-v0.1.schema.json",
    "constraint-cardinality-normalized-v0.1.schema.json",
    "constraint-type-authoring-v0.1.schema.json",
    "constraint-type-normalized-v0.1.schema.json",
    "constraint-value-authoring-v0.1.schema.json",
    "constraint-value-normalized-v0.1.schema.json",
    "constraint-dimension-authoring-v0.1.schema.json",
    "constraint-dimension-normalized-v0.1.schema.json",
    "constraint-compatibility-authoring-v0.1.schema.json",
    "constraint-compatibility-normalized-v0.1.schema.json",
    "predicate-authoring-v0.1.schema.json",
    "predicate-normalized-v0.1.schema.json",
    "constraint-conditional-authoring-v0.1.schema.json",
    "constraint-conditional-normalized-v0.1.schema.json",
]
SCHEMAS = {
    name: json.loads((ROOT / "schema" / name).read_text())
    for name in SCHEMA_FILES
}
REGISTRY = Registry().with_resources(
    (schema["$id"], Resource.from_contents(schema))
    for schema in SCHEMAS.values()
)
AUTHORING_PREDICATE = Draft202012Validator(SCHEMAS["predicate-authoring-v0.1.schema.json"], registry=REGISTRY)
NORMALIZED_PREDICATE = Draft202012Validator(SCHEMAS["predicate-normalized-v0.1.schema.json"], registry=REGISTRY)
AUTHORING_CONDITIONAL = Draft202012Validator(SCHEMAS["constraint-conditional-authoring-v0.1.schema.json"], registry=REGISTRY)
NORMALIZED_CONDITIONAL = Draft202012Validator(SCHEMAS["constraint-conditional-normalized-v0.1.schema.json"], registry=REGISTRY)

BINDINGS = [
    {"key": "cfg:mode", "slot": "mode"},
    {"key": "cfg:number", "slot": "number"},
    {"key": "cfg:optional", "slot": "optional"},
    {"key": "cfg:bad", "slot": "bad"},
]


def present(kind, value):
    return {"state": "PRESENT", "scalar_kind": kind, "value": value}


class ConditionalPredicateSchemaTests(unittest.TestCase):
    def test_all_schema_documents_are_valid_draft_2020_12(self):
        for name, schema in SCHEMAS.items():
            with self.subTest(name=name):
                Draft202012Validator.check_schema(schema)

    def test_membership_authoring_requires_scalar_kind_even_when_empty(self):
        valid = {
            "predicate": "membership",
            "ref": "mode",
            "scalar_kind": "string",
            "values": [],
        }
        AUTHORING_PREDICATE.validate(valid)
        invalid = dict(valid)
        invalid.pop("scalar_kind")
        with self.assertRaises(ValidationError):
            AUTHORING_PREDICATE.validate(invalid)

    def test_ordering_compare_is_numeric_only(self):
        with self.assertRaises(ValidationError):
            AUTHORING_PREDICATE.validate({
                "predicate": "compare",
                "ref": "mode",
                "op": "lt",
                "value": "advanced",
            })

    def test_boolean_and_or_require_nonempty_operands(self):
        for operator in ("and", "or"):
            with self.subTest(operator=operator):
                with self.assertRaises(ValidationError):
                    AUTHORING_PREDICATE.validate({"predicate": operator, "operands": []})

    def test_normalized_numeric_compare_reuses_exact_decimal_shape(self):
        predicate = {
            "predicate": "compare",
            "ref": {"key": "cfg:number"},
            "scalar_kind": "number",
            "op": "eq",
            "value": canonicalize_decimal_lexeme("1.0"),
        }
        NORMALIZED_PREDICATE.validate(predicate)
        bad = dict(predicate)
        bad["value"] = 1.0
        with self.assertRaises(ValidationError):
            NORMALIZED_PREDICATE.validate(bad)

    def test_conditional_requires_nonempty_ordinary_consequents(self):
        predicate = {"predicate": "exists", "ref": "aux"}
        with self.assertRaises(ValidationError):
            AUTHORING_CONDITIONAL.validate({"type": "conditional", "if": predicate, "then": []})
        valid = {
            "type": "conditional",
            "if": predicate,
            "then": [
                {"type": "dimension", "vector": {}},
            ],
        }
        AUTHORING_CONDITIONAL.validate(valid)

    def test_nested_conditional_consequent_is_rejected(self):
        nested = {
            "type": "conditional",
            "if": {"predicate": "exists", "ref": "inner"},
            "then": [{"type": "dimension", "vector": {}}],
        }
        outer = {
            "type": "conditional",
            "if": {"predicate": "exists", "ref": "outer"},
            "then": [nested],
        }
        with self.assertRaises(ValidationError):
            AUTHORING_CONDITIONAL.validate(outer)

    def test_evaluation_reference_requires_exactly_one_binding(self):
        self.assertEqual(resolve_binding("cfg:mode", BINDINGS)["slot"], "mode")
        with self.assertRaisesRegex(PredicateEvaluationError, "PREDICATE_REFERENCE_UNRESOLVED"):
            resolve_binding("cfg:missing", BINDINGS)
        ambiguous = list(BINDINGS) + [{"key": "cfg:mode", "slot": "mode-2"}]
        with self.assertRaisesRegex(PredicateEvaluationError, "PREDICATE_REFERENCE_AMBIGUOUS"):
            resolve_binding("cfg:mode", ambiguous)

    def test_exists_is_presence_not_truthiness(self):
        predicate = {"predicate": "exists", "ref": {"key": "cfg:optional"}}
        for value in (False, "", canonicalize_decimal_lexeme("0")):
            kind = "boolean" if isinstance(value, bool) else ("string" if isinstance(value, str) else "number")
            result = evaluate_predicate(predicate, BINDINGS, {"optional": present(kind, value)})
            self.assertEqual(result["truth"], "TRUE")
        self.assertEqual(evaluate_predicate(predicate, BINDINGS, {})["truth"], "FALSE")
        unresolved = evaluate_predicate(predicate, BINDINGS, {"optional": {"state": "UNRESOLVED"}})
        self.assertEqual(unresolved["truth"], "INDETERMINATE")

    def test_absent_compare_and_membership_are_indeterminate(self):
        compare = {
            "predicate": "compare",
            "ref": {"key": "cfg:optional"},
            "scalar_kind": "string",
            "op": "ne",
            "value": "advanced",
        }
        membership = {
            "predicate": "membership",
            "ref": {"key": "cfg:optional"},
            "scalar_kind": "string",
            "values": ["advanced"],
        }
        self.assertEqual(evaluate_predicate(compare, BINDINGS, {})["truth"], "INDETERMINATE")
        self.assertEqual(evaluate_predicate(membership, BINDINGS, {})["truth"], "INDETERMINATE")

    def test_scalar_kind_mismatch_is_evaluation_failure(self):
        predicate = {
            "predicate": "compare",
            "ref": {"key": "cfg:bad"},
            "scalar_kind": "number",
            "op": "eq",
            "value": canonicalize_decimal_lexeme("1"),
        }
        result = evaluate_predicate(predicate, BINDINGS, {"bad": present("string", "1")})
        self.assertEqual(result["kind"], "failure")
        self.assertIn("PREDICATE_SCALAR_KIND_MISMATCH", result["codes"])

    def test_exact_numeric_compare_avoids_float_identity(self):
        predicate = {
            "predicate": "compare",
            "ref": {"key": "cfg:number"},
            "scalar_kind": "number",
            "op": "eq",
            "value": canonicalize_decimal_lexeme("1e0"),
        }
        snapshot = {"number": present("number", canonicalize_decimal_lexeme("1.0"))}
        self.assertEqual(evaluate_predicate(predicate, BINDINGS, snapshot)["truth"], "TRUE")

    def test_empty_membership_is_false_for_present_correct_kind(self):
        predicate = {
            "predicate": "membership",
            "ref": {"key": "cfg:mode"},
            "scalar_kind": "string",
            "values": [],
        }
        snapshot = {"mode": present("string", "advanced")}
        self.assertEqual(evaluate_predicate(predicate, BINDINGS, snapshot)["truth"], "FALSE")

    def test_membership_array_order_is_semantically_irrelevant(self):
        base = {
            "predicate": "membership",
            "ref": {"key": "cfg:mode"},
            "scalar_kind": "string",
        }
        snapshot = {"mode": present("string", "advanced")}
        a = evaluate_predicate(dict(base, values=["basic", "advanced"]), BINDINGS, snapshot)
        b = evaluate_predicate(dict(base, values=["advanced", "basic", "advanced"]), BINDINGS, snapshot)
        self.assertEqual(a["truth"], b["truth"])
        self.assertEqual(a["truth"], "TRUE")

    def test_strong_kleene_truth_reduction(self):
        self.assertEqual(evaluate_boolean("and", [truth("FALSE"), truth("INDETERMINATE")])["truth"], "FALSE")
        self.assertEqual(evaluate_boolean("or", [truth("TRUE"), truth("INDETERMINATE")])["truth"], "TRUE")
        self.assertEqual(evaluate_boolean("not", [truth("INDETERMINATE")])["truth"], "INDETERMINATE")

    def test_evaluation_failure_cannot_be_short_circuit_masked(self):
        and_result = evaluate_boolean("and", [truth("FALSE"), failure("X")])
        or_result = evaluate_boolean("or", [truth("TRUE"), failure("Y")])
        self.assertEqual(and_result["kind"], "failure")
        self.assertEqual(or_result["kind"], "failure")
        multiple = evaluate_boolean("and", [failure("X"), failure("Y"), truth("FALSE")])
        self.assertEqual(multiple["codes"], frozenset({"X", "Y"}))

    def test_conditional_activation_states(self):
        consequent = [{"type": "dimension", "vector": {}}]
        self.assertEqual(activate_conditional(truth("TRUE"), consequent)["state"], "ACTIVE")
        self.assertEqual(activate_conditional(truth("FALSE"), consequent)["state"], "INACTIVE")
        self.assertEqual(activate_conditional(truth("INDETERMINATE"), consequent)["state"], "INDETERMINATE")
        self.assertEqual(activate_conditional(failure("X"), consequent)["state"], "FAIL")

    def test_cross_family_aggregation_propagates_ordinary_indeterminate(self):
        self.assertEqual(aggregate_validation_states([], []), "PASS")
        self.assertEqual(aggregate_validation_states(["INDETERMINATE"], ["ACTIVE"]), "INDETERMINATE")
        self.assertEqual(aggregate_validation_states(["FAIL", "INDETERMINATE"], ["INDETERMINATE"]), "FAIL")
        self.assertEqual(aggregate_validation_states(["PASS"], ["INDETERMINATE"]), "INDETERMINATE")
        self.assertEqual(
            aggregate_validation_states(["INDETERMINATE"], [], evaluation_failure_codes={"X"}),
            "FAIL",
        )

    def test_non_three_state_ordinary_result_is_not_silently_coerced(self):
        with self.assertRaisesRegex(PredicateEvaluationError, "VALIDATION_ORDINARY_STATE_UNSUPPORTED"):
            aggregate_validation_states(["BLOCKED"], [])


if __name__ == "__main__":
    unittest.main()
