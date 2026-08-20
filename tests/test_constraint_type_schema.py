import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

from tests.constraint_type_semantics import (
    REPRESENTABILITY_AXIS,
    SEMANTIC_AXIS,
    TypeConstraintError,
    group_evidence_by_axis,
    intersect_type_targets,
    normalize_type_payload,
    validate_allowed_pair_narrowing,
    validate_target_family,
)

ROOT = Path(__file__).resolve().parents[1]
AUTHORING = json.loads((ROOT / "schema" / "constraint-type-authoring-v0.1.schema.json").read_text())
NORMALIZED = json.loads((ROOT / "schema" / "constraint-type-normalized-v0.1.schema.json").read_text())

ENTITY_TYPES = {
    "core:SimulationModel",
    "core:MathematicalModel",
    "core:MaterialModel",
    "core:ConditionModel",
    "core:BoundaryCondition",
    "core:Field",
    "core:Equation",
    "core:Scope",
    "core:Material",
    "thermal:TemperatureField",
    "thermal:WallTemperatureBC",
    "plasma:ElectronTemperatureField",
    "plasma:ElectricField",
}
INTERFACES = {"core:ScopedCapability"}
PARENTS = {
    "thermal:TemperatureField": "core:Field",
    "plasma:ElectronTemperatureField": "thermal:TemperatureField",
    "plasma:ElectricField": "core:Field",
    "thermal:WallTemperatureBC": "core:BoundaryCondition",
}

INCLUDES_COMPONENT_PAIRS = [
    ("core:SimulationModel", ["core:MathematicalModel", "core:MaterialModel", "core:ConditionModel"]),
    ("core:MathematicalModel", ["core:Field", "core:Equation"]),
    ("core:MaterialModel", ["core:Material"]),
    ("core:ConditionModel", ["core:BoundaryCondition"]),
]


class ConstraintTypeSchemaTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Draft202012Validator.check_schema(AUTHORING)
        Draft202012Validator.check_schema(NORMALIZED)
        cls.authoring_validator = Draft202012Validator(AUTHORING)
        cls.normalized_validator = Draft202012Validator(NORMALIZED)

    def test_authoring_requires_exact_three_fields(self):
        valid = {"type": "type", "relation": "applied_to", "target_type": "TemperatureField"}
        self.authoring_validator.validate(valid)
        for missing in ("type", "relation", "target_type"):
            with self.subTest(missing=missing):
                with self.assertRaises(ValidationError):
                    self.authoring_validator.validate({k: v for k, v in valid.items() if k != missing})
        with self.assertRaises(ValidationError):
            self.authoring_validator.validate(dict(valid, subject="bc-1"))

    def test_normalization_requires_resolved_canonical_ids(self):
        authored = {"type": "type", "relation": "applied_to", "target_type": "TemperatureField"}
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_RELATION_CANONICAL_ID_REQUIRED"):
            normalize_type_payload(
                authored,
                canonical_relation=None,
                canonical_target_type="thermal:TemperatureField",
                entity_type_ids=ENTITY_TYPES,
                interface_ids=INTERFACES,
            )
        normalized = normalize_type_payload(
            authored,
            canonical_relation="core:applied_to",
            canonical_target_type="thermal:TemperatureField",
            entity_type_ids=ENTITY_TYPES,
            interface_ids=INTERFACES,
        )
        self.normalized_validator.validate(normalized)
        self.assertEqual(normalized["relation"], "core:applied_to")
        self.assertEqual(normalized["target_type"], "thermal:TemperatureField")

    def test_interface_identifier_is_not_a_type_target(self):
        authored = {"type": "type", "relation": "applied_to", "target_type": "ScopedCapability"}
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_TARGET_INTERFACE_FORBIDDEN"):
            normalize_type_payload(
                authored,
                canonical_relation="core:applied_to",
                canonical_target_type="core:ScopedCapability",
                entity_type_ids=ENTITY_TYPES,
                interface_ids=INTERFACES,
            )

    def test_finite_target_family_accepts_subtype_and_rejects_unrelated_type(self):
        validate_target_family(
            "plasma:ElectronTemperatureField",
            ["core:Field", "core:Equation", "core:Scope"],
            PARENTS,
        )
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_TARGET_NOT_ALLOWED"):
            validate_target_family(
                "core:Material",
                ["core:Field", "core:Equation", "core:Scope"],
                PARENTS,
            )

    def test_type_intersection_returns_narrower_subtype(self):
        self.assertEqual(
            intersect_type_targets("core:Field", "thermal:TemperatureField", PARENTS),
            "thermal:TemperatureField",
        )
        self.assertEqual(
            intersect_type_targets("thermal:TemperatureField", "plasma:ElectronTemperatureField", PARENTS),
            "plasma:ElectronTemperatureField",
        )

    def test_type_intersection_rejects_unrelated_targets(self):
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_INTERSECTION_EMPTY"):
            intersect_type_targets("thermal:TemperatureField", "core:Material", PARENTS)

    def test_allowed_pair_narrowing_does_not_create_new_pair(self):
        validate_allowed_pair_narrowing(
            "core:MathematicalModel",
            "thermal:TemperatureField",
            INCLUDES_COMPONENT_PAIRS,
            PARENTS,
        )
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_TARGET_NOT_ALLOWED"):
            validate_allowed_pair_narrowing(
                "core:MaterialModel",
                "thermal:WallTemperatureBC",
                INCLUDES_COMPONENT_PAIRS,
                PARENTS,
            )

    def test_subtype_source_context_inherits_allowed_pair(self):
        local_parents = dict(PARENTS)
        local_parents["thermal:ThermalMathematicalModel"] = "core:MathematicalModel"
        validate_allowed_pair_narrowing(
            "thermal:ThermalMathematicalModel",
            "thermal:TemperatureField",
            INCLUDES_COMPONENT_PAIRS,
            local_parents,
        )

    def test_semantic_and_representability_axes_do_not_intersect_implicitly(self):
        semantic_payload = {"type": "type", "relation": "core:applied_to", "target_type": "core:Field"}
        profile_payload = {"type": "type", "relation": "core:applied_to", "target_type": "thermal:TemperatureField"}
        grouped = group_evidence_by_axis(
            [
                {"axis": SEMANTIC_AXIS, "constraint": semantic_payload, "source": "core"},
                {"axis": REPRESENTABILITY_AXIS, "constraint": profile_payload, "source": "profile:thermal-backend"},
            ]
        )
        self.assertEqual(len(grouped[SEMANTIC_AXIS]), 1)
        self.assertEqual(len(grouped[REPRESENTABILITY_AXIS]), 1)
        self.assertIsNot(grouped[SEMANTIC_AXIS], grouped[REPRESENTABILITY_AXIS])

    def test_invalid_evaluation_axis_is_rejected(self):
        with self.assertRaisesRegex(TypeConstraintError, "TYPE_EVALUATION_AXIS_INVALID"):
            group_evidence_by_axis([{"axis": "profile-priority", "constraint": {}}])


if __name__ == "__main__":
    unittest.main()
