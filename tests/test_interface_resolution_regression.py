import unittest

from tests.interface_semantics import (
    InterfaceContractError,
    effective_entity_conformance,
    effective_interface_contract,
    validate_direct_implementation,
    validate_requirement_definitions,
)


def iface(identifier, *, properties=(), relations=()):
    return {
        "id": identifier,
        "extends": [],
        "property_requirements": [{"property": item} for item in properties],
        "relation_requirements": [{"relation": item} for item in relations],
        "constraint_applications": [],
    }


def impl(entity, interface, *, properties=(), relations=()):
    return {
        "entity_type": entity,
        "interface": interface,
        "property_mappings": [
            {"requirement": requirement, "concrete": concrete}
            for requirement, concrete in properties
        ],
        "relation_mappings": [
            {"requirement": requirement, "concrete": concrete}
            for requirement, concrete in relations
        ],
    }


class InterfaceResolutionRegressionTests(unittest.TestCase):
    def test_unresolved_requirement_definition_fails_before_mapping(self):
        contract = effective_interface_contract("iface:A", [iface("iface:A", properties=["prop:missing"])])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED"):
            validate_requirement_definitions(contract, {})

    def test_unresolved_concrete_definition_is_not_kind_mismatch(self):
        interfaces = [iface("iface:A", properties=["prop:req"])]
        declaration = impl("entity:E", "iface:A", properties=[("prop:req", "prop:missing")])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONCRETE_DEFINITION_UNRESOLVED"):
            validate_direct_implementation(declaration, interfaces, {"prop:req": "property"})

    def test_resolved_wrong_concrete_kind_remains_kind_mismatch(self):
        interfaces = [iface("iface:A", properties=["prop:req"])]
        declaration = impl("entity:E", "iface:A", properties=[("prop:req", "rel:x")])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_REQUIREMENT_KIND_MISMATCH"):
            validate_direct_implementation(
                declaration,
                interfaces,
                {"prop:req": "property", "rel:x": "relation"},
            )

    def test_unresolved_entity_parent_fails_in_ancestor_closure(self):
        interfaces = [iface("iface:A")]
        with self.assertRaisesRegex(InterfaceContractError, "ENTITY_TYPE_UNRESOLVED"):
            effective_entity_conformance(
                "entity:Child",
                entity_parent={"entity:Child": "entity:MissingParent"},
                interfaces=interfaces,
                implementations=[],
                definition_kinds={},
            )

    def test_resolved_root_entity_remains_valid(self):
        result = effective_entity_conformance(
            "entity:E",
            entity_parent={"entity:E": None},
            interfaces=[],
            implementations=[],
            definition_kinds={},
        )
        self.assertEqual(result["interfaces"], frozenset())
        self.assertEqual(result["mappings"], {})


if __name__ == "__main__":
    unittest.main()
