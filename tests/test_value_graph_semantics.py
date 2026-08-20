import unittest

from tests.value_graph_semantics import (
    DIMENSION_AXES,
    ValueGraphError,
    choose_value_definition_representation,
    dimensions_equal,
    evaluate_metrology_state,
    is_dimension_one,
    unit_reference_key,
    validate_reified_definition_graph,
    validate_value_semantics,
)


def dim(**overrides):
    result = {axis: 0 for axis in DIMENSION_AXES}
    result.update(overrides)
    return result


class ValueGraphSemanticsTests(unittest.TestCase):
    def test_dimension_one_is_explicit_zero_vector(self):
        self.assertTrue(is_dimension_one(dim()))
        self.assertFalse(is_dimension_one(dim(length=1)))

    def test_dimension_equality_uses_vector_not_label(self):
        self.assertTrue(dimensions_equal(dim(mass=1, length=-1), dim(mass=1.0, length=-1.0)))

    def test_unit_reference_identity_is_namespace_and_id(self):
        self.assertEqual(unit_reference_key({"namespace": "qudt", "id": "K"}), ("qudt", "K"))

    def test_non_numeric_unit_is_rejected(self):
        with self.assertRaisesRegex(ValueGraphError, "VALUE_UNIT_NON_NUMERIC"):
            validate_value_semantics(
                {"shape": "scalar", "scalar_kind": "string", "data": "hot", "unit": {"namespace": "x", "id": "K"}}
            )

    def test_tensor_component_count_is_checked(self):
        value = {
            "shape": "tensor",
            "scalar_kind": "number",
            "tensor_shape": [2, 2],
            "data": [1, 2, 3],
        }
        with self.assertRaisesRegex(ValueGraphError, "VALUE_TENSOR_COMPONENT_COUNT_MISMATCH"):
            validate_value_semantics(value)

    def test_tensor_integral_float_shape_matches_json_integer_semantics(self):
        value = {
            "shape": "tensor",
            "scalar_kind": "boolean",
            "tensor_shape": [2.0, 2.0],
            "data": [True, False, False, True],
        }
        self.assertTrue(validate_value_semantics(value))

    def test_missing_unit_does_not_imply_validity(self):
        state, code = evaluate_metrology_state(
            expected_dimension=dim(thermodynamic_temperature=1),
            explicit_unit=None,
            omission_policy="unresolved",
        )
        self.assertEqual((state, code), ("INDETERMINATE", "VALUE_CONTEXTUAL_UNIT_POLICY_UNRESOLVED"))

    def test_explicit_required_unit_missing_is_fail(self):
        state, code = evaluate_metrology_state(
            expected_dimension=dim(thermodynamic_temperature=1),
            explicit_unit=None,
            omission_policy="forbidden",
        )
        self.assertEqual((state, code), ("FAIL", "VALUE_UNIT_REQUIRED_MISSING"))

    def test_unresolved_metrology_service_is_indeterminate(self):
        state, code = evaluate_metrology_state(
            expected_dimension=dim(thermodynamic_temperature=1),
            explicit_unit={"namespace": "qudt", "id": "DEG_C"},
            resolved_unit_dimension=None,
        )
        self.assertEqual((state, code), ("INDETERMINATE", "VALUE_UNIT_REFERENCE_UNRESOLVED"))

    def test_resolved_dimension_mismatch_is_fail(self):
        state, code = evaluate_metrology_state(
            expected_dimension=dim(length=1),
            explicit_unit={"namespace": "qudt", "id": "SEC"},
            resolved_unit_dimension=dim(time=1),
        )
        self.assertEqual((state, code), ("FAIL", "VALUE_UNIT_DIMENSION_MISMATCH"))

    def test_provider_dependency_forces_reification(self):
        result = choose_value_definition_representation(
            mechanism="expression",
            provider_state="PASS",
            resolved_semantic_dependencies=["model:A", "model:A"],
        )
        self.assertEqual(result, ("REIFIED", None))

    def test_provider_unavailable_never_raw_scans(self):
        result = choose_value_definition_representation(
            mechanism="expression",
            provider_state="INDETERMINATE",
            resolved_semantic_dependencies=(),
        )
        self.assertEqual(result, ("INDETERMINATE", "VALUE_DEFINITION_FORMAT_PROVIDER_UNRESOLVED"))

    def test_missing_provider_evidence_is_indeterminate_for_nonliteral(self):
        result = choose_value_definition_representation(
            mechanism="expression",
            resolved_semantic_dependencies=(),
        )
        self.assertEqual(result, ("INDETERMINATE", "VALUE_DEFINITION_FORMAT_PROVIDER_UNRESOLVED"))

    def test_local_nonliteral_without_dependencies_can_stay_inline(self):
        result = choose_value_definition_representation(
            mechanism="function",
            provider_state="PASS",
            resolved_semantic_dependencies=(),
        )
        self.assertEqual(result, ("INLINE", None))

    def test_literal_can_be_reified_for_reuse(self):
        self.assertEqual(
            choose_value_definition_representation(mechanism="literal", other_reification_trigger=True),
            ("REIFIED", None),
        )

    def test_two_owners_share_one_reified_definition_identity(self):
        self.assertTrue(
            validate_reified_definition_graph(
                definition_payloads=[("vd:1", {"mechanism": "literal"})],
                owner_edges=[("owner:A", "vd:1"), ("owner:B", "vd:1")],
                dependency_edges=[("vd:1", "model:T")],
            )
        )

    def test_conflicting_payload_same_definition_identity_fails(self):
        with self.assertRaisesRegex(ValueGraphError, "VALUE_DEFINITION_IDENTITY_CONTENT_CONFLICT"):
            validate_reified_definition_graph(
                definition_payloads=[
                    ("vd:1", {"mechanism": "literal", "v": 1}),
                    ("vd:1", {"mechanism": "literal", "v": 2}),
                ],
                owner_edges=[],
                dependency_edges=[],
            )

    def test_unknown_relation_endpoint_cannot_stand_for_inline_definition(self):
        with self.assertRaisesRegex(ValueGraphError, "VALUE_DEFINITION_RELATION_ENDPOINT_INVALID"):
            validate_reified_definition_graph(
                definition_payloads=[],
                owner_edges=[],
                dependency_edges=[("inline:anonymous", "model:T")],
            )


if __name__ == "__main__":
    unittest.main()
