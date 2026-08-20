import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

from tests.constraint_dimension_semantics import (
    DimensionConstraintError,
    DIMENSION_AXES,
    intersect_dimension_vectors,
    is_dimension_one,
    normalize_dimension_payload,
    normalize_dimension_vector,
)

ROOT = Path(__file__).resolve().parents[1]
AUTHORING = json.loads((ROOT / "schema" / "constraint-dimension-authoring-v0.1.schema.json").read_text())
NORMALIZED = json.loads((ROOT / "schema" / "constraint-dimension-normalized-v0.1.schema.json").read_text())


class ConstraintDimensionSchemaTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Draft202012Validator.check_schema(AUTHORING)
        Draft202012Validator.check_schema(NORMALIZED)
        cls.authoring_validator = Draft202012Validator(AUTHORING)
        cls.normalized_validator = Draft202012Validator(NORMALIZED)

    def test_sparse_authoring_normalizes_omitted_axes_to_zero(self):
        authored = {
            "type": "dimension",
            "vector": {
                "mass": 1,
                "length": 1,
                "time": -3,
                "thermodynamic_temperature": -1,
            },
        }
        self.authoring_validator.validate(authored)
        normalized = normalize_dimension_payload(authored)
        self.normalized_validator.validate(normalized)
        self.assertEqual(set(normalized["vector"]), set(DIMENSION_AXES))
        self.assertEqual(normalized["vector"]["electric_current"], 0)
        self.assertEqual(normalized["vector"]["amount_of_substance"], 0)
        self.assertEqual(normalized["vector"]["luminous_intensity"], 0)

    def test_explicit_empty_vector_is_dimension_one(self):
        authored = {"type": "dimension", "vector": {}}
        self.authoring_validator.validate(authored)
        normalized = normalize_dimension_payload(authored)
        self.normalized_validator.validate(normalized)
        self.assertTrue(is_dimension_one(normalized["vector"]))
        self.assertEqual(set(normalized["vector"].values()), {0})

    def test_missing_vector_is_not_dimension_one(self):
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate({"type": "dimension"})
        with self.assertRaisesRegex(DimensionConstraintError, "DIMENSION_VECTOR_REQUIRED"):
            normalize_dimension_payload({"type": "dimension"})

    def test_unknown_axis_is_rejected(self):
        authored = {"type": "dimension", "vector": {"currency": 1}}
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate(authored)
        with self.assertRaisesRegex(DimensionConstraintError, "DIMENSION_AXIS_UNKNOWN"):
            normalize_dimension_payload(authored)

    def test_non_integer_exponent_is_rejected_in_v0_1(self):
        for exponent in (-0.5, 1.5, True):
            with self.subTest(exponent=exponent):
                authored = {"type": "dimension", "vector": {"time": exponent}}
                with self.assertRaises(ValidationError):
                    self.authoring_validator.validate(authored)

    def test_thermal_conductivity_round_trip(self):
        sparse = {
            "mass": 1,
            "length": 1,
            "time": -3,
            "thermodynamic_temperature": -1,
        }
        normalized = normalize_dimension_vector(sparse)
        self.assertEqual(normalized["mass"], 1)
        self.assertEqual(normalized["length"], 1)
        self.assertEqual(normalized["time"], -3)
        self.assertEqual(normalized["thermodynamic_temperature"], -1)
        self.normalized_validator.validate({"type": "dimension", "vector": normalized})

    def test_normalized_schema_requires_all_axes(self):
        vector = normalize_dimension_vector({"length": 1})
        self.normalized_validator.validate({"type": "dimension", "vector": vector})
        incomplete = dict(vector)
        incomplete.pop("luminous_intensity")
        with self.assertRaises(ValidationError):
            self.normalized_validator.validate({"type": "dimension", "vector": incomplete})

    def test_equal_vectors_intersect_independent_of_sparse_key_order(self):
        left = {"length": 1, "time": -1}
        right = {"time": -1, "length": 1}
        result = intersect_dimension_vectors(left, right)
        self.assertEqual(result, normalize_dimension_vector(left))

    def test_unequal_vectors_produce_empty_dimension_intersection(self):
        pressure = {"mass": 1, "length": -1, "time": -2}
        temperature = {"thermodynamic_temperature": 1}
        with self.assertRaisesRegex(DimensionConstraintError, "DIMENSION_INTERSECTION_EMPTY"):
            intersect_dimension_vectors(pressure, temperature)

    def test_backend_metadata_does_not_change_semantic_vector(self):
        authored = {"type": "dimension", "vector": {"length": 1}}
        baseline = normalize_dimension_payload(authored)
        backend_metadata = {
            "backend": "example",
            "unit_display": "in",
            "native_unit_system": "imperial",
        }
        self.assertEqual(baseline, normalize_dimension_payload(authored))
        self.assertNotIn("backend", baseline)
        self.assertNotIn("unit_display", baseline)
        self.assertTrue(backend_metadata)


if __name__ == "__main__":
    unittest.main()
