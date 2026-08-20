import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource

from tests.constraint_predicate_semantics import normalize_membership_values
from tests.constraint_value_semantics import canonicalize_decimal_lexeme

ROOT = Path(__file__).resolve().parents[1]
VALUE_SCHEMA = json.loads((ROOT / "schema" / "constraint-value-normalized-v0.1.schema.json").read_text())
PREDICATE_SCHEMA = json.loads((ROOT / "schema" / "predicate-normalized-v0.1.schema.json").read_text())
REGISTRY = Registry().with_resources([
    (VALUE_SCHEMA["$id"], Resource.from_contents(VALUE_SCHEMA)),
    (PREDICATE_SCHEMA["$id"], Resource.from_contents(PREDICATE_SCHEMA)),
])
VALIDATOR = Draft202012Validator(PREDICATE_SCHEMA, registry=REGISTRY)


class MembershipNormalizationRegressionTests(unittest.TestCase):
    def test_normalizer_collapses_string_duplicates(self):
        self.assertEqual(
            normalize_membership_values(["advanced", "basic", "advanced"], "string"),
            ["advanced", "basic"],
        )

    def test_normalizer_collapses_exact_decimal_duplicates(self):
        one = canonicalize_decimal_lexeme("1.0")
        same_one = canonicalize_decimal_lexeme("1e0")
        two = canonicalize_decimal_lexeme("2")
        self.assertEqual(
            normalize_membership_values([one, same_one, two], "number"),
            [one, two],
        )

    def test_normalized_schema_rejects_retained_duplicates(self):
        payload = {
            "predicate": "membership",
            "ref": {"key": "cfg:mode"},
            "scalar_kind": "string",
            "values": ["advanced", "advanced"],
        }
        with self.assertRaises(ValidationError):
            VALIDATOR.validate(payload)

    def test_normalized_schema_accepts_deduplicated_set(self):
        payload = {
            "predicate": "membership",
            "ref": {"key": "cfg:mode"},
            "scalar_kind": "string",
            "values": normalize_membership_values(["advanced", "advanced"], "string"),
        }
        VALIDATOR.validate(payload)


if __name__ == "__main__":
    unittest.main()
