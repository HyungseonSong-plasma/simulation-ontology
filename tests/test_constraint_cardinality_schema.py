import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource

from tests.constraint_cardinality_normalizer import normalize_bounds, normalize_cardinality_payload

ROOT = Path(__file__).resolve().parents[1]
AUTHORING = json.loads((ROOT / "schema" / "constraint-cardinality-authoring-v0.1.schema.json").read_text())
NORMALIZED = json.loads((ROOT / "schema" / "constraint-cardinality-normalized-v0.1.schema.json").read_text())
QRC = json.loads((ROOT / "schema" / "qrc-v0.1.schema.json").read_text())


class ConstraintCardinalitySchemaTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Draft202012Validator.check_schema(AUTHORING)
        Draft202012Validator.check_schema(NORMALIZED)
        Draft202012Validator.check_schema(QRC)
        cls.authoring_validator = Draft202012Validator(AUTHORING)
        cls.normalized_validator = Draft202012Validator(NORMALIZED)
        registry = Registry().with_resource(
            AUTHORING["$id"], Resource.from_contents(AUTHORING)
        )
        cls.qrc_validator = Draft202012Validator(QRC, registry=registry)

    def test_boundless_authoring_is_valid_and_normalizes_unbounded(self):
        value = {"type": "cardinality", "relation": "solved_by"}
        self.authoring_validator.validate(value)
        normalized = normalize_cardinality_payload(value, canonical_relation="core:solved_by")
        self.assertEqual(normalized["min"], 0)
        self.assertEqual(normalized["max"], "unbounded")
        self.normalized_validator.validate(normalized)

    def test_qrc_compatibility_entry_point_uses_common_authoring_shape(self):
        value = {
            "type": "cardinality",
            "relation": "products",
            "qualifier": {"target_type": "NegativeIonSpecies"},
            "min": 1,
        }
        self.qrc_validator.validate(value)
        with self.assertRaises(ValidationError):
            self.qrc_validator.validate(dict(value, unknown=True))

    def test_mixed_bounds_normalize_exact(self):
        value = {
            "type": "cardinality",
            "relation": "products",
            "min": 1,
            "max": 3,
            "exact": 2,
        }
        self.authoring_validator.validate(value)
        normalized = normalize_cardinality_payload(value, canonical_relation="core:products")
        self.assertEqual((normalized["min"], normalized["max"]), (2, 2))
        self.assertNotIn("exact", normalized)
        self.normalized_validator.validate(normalized)

    def test_empty_interval_is_semantic_normalization_failure(self):
        value = {"type": "cardinality", "relation": "products", "min": 3, "exact": 2}
        self.authoring_validator.validate(value)
        self.assertEqual(normalize_bounds(value)[:2], ("FAIL", "QRC_EMPTY_INTERVAL"))
        with self.assertRaisesRegex(ValueError, "QRC_EMPTY_INTERVAL"):
            normalize_cardinality_payload(value, canonical_relation="core:products")

    def test_negative_and_non_integer_bounds_are_structurally_rejected(self):
        for value in (-1, 1.5, True):
            with self.subTest(value=value):
                with self.assertRaises(ValidationError):
                    self.authoring_validator.validate({"type": "cardinality", "relation": "x", "min": value})

    def test_normalized_requires_explicit_interval_and_direction(self):
        valid = {
            "type": "cardinality",
            "relation": "core:solved_by",
            "direction": "source",
            "min": 0,
            "max": "unbounded",
        }
        self.normalized_validator.validate(valid)
        for missing in ("direction", "min", "max"):
            with self.subTest(missing=missing):
                bad = {k: v for k, v in valid.items() if k != missing}
                with self.assertRaises(ValidationError):
                    self.normalized_validator.validate(bad)

    def test_normalized_rejects_exact_and_non_source_direction(self):
        base = {
            "type": "cardinality",
            "relation": "core:solved_by",
            "direction": "source",
            "min": 0,
            "max": "unbounded",
        }
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate(dict(base, exact=1))
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate(dict(base, direction="target"))

    def test_qualified_normalization_requires_resolved_target_type(self):
        value = {
            "type": "cardinality",
            "relation": "products",
            "qualifier": {"target_type": "NegativeIonSpecies"},
            "min": 1,
        }
        with self.assertRaisesRegex(ValueError, "QRC_QUALIFIER_CANONICAL_ID_REQUIRED"):
            normalize_cardinality_payload(value, canonical_relation="core:products")
        normalized = normalize_cardinality_payload(
            value,
            canonical_relation="core:products",
            canonical_target_type="plasma:NegativeIonSpecies",
        )
        self.normalized_validator.validate(normalized)
        self.assertEqual(normalized["qualifier"]["target_type"], "plasma:NegativeIonSpecies")

    def test_core_projection_fixture_matches_normalized_constraint(self):
        relation_projection = {"min": 0, "max": "unbounded"}
        canonical_constraint = {
            "type": "cardinality",
            "relation": "solved_by",
            "direction": "source",
            "min": 0,
            "max": "unbounded",
        }
        self.normalized_validator.validate(canonical_constraint)
        self.assertEqual(
            relation_projection,
            {"min": canonical_constraint["min"], "max": canonical_constraint["max"]},
        )


if __name__ == "__main__":
    unittest.main()
