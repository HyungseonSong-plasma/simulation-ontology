import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

from tests.interface_semantics import (
    InterfaceContractError,
    effective_entity_conformance,
    effective_interface_contract,
    validate_constraint_applications,
    validate_direct_declaration_uniqueness,
    validate_direct_implementation,
)

ROOT = Path(__file__).resolve().parents[1]
INTERFACE_SCHEMA = json.loads((ROOT / "schema" / "interface-definition-v0.1.schema.json").read_text())
IMPLEMENTATION_SCHEMA = json.loads((ROOT / "schema" / "interface-implementation-v0.1.schema.json").read_text())
INTERFACE_VALIDATOR = Draft202012Validator(INTERFACE_SCHEMA)
IMPLEMENTATION_VALIDATOR = Draft202012Validator(IMPLEMENTATION_SCHEMA)


def iface(identifier, *, extends=(), properties=(), relations=(), applications=()):
    return {
        "id": identifier,
        "extends": list(extends),
        "property_requirements": [{"property": item} for item in properties],
        "relation_requirements": [{"relation": item} for item in relations],
        "constraint_applications": list(applications),
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


KINDS = {
    "prop:req": "property",
    "prop:x": "property",
    "prop:y": "property",
    "prop:extra": "property",
    "rel:req": "relation",
    "rel:x": "relation",
    "rel:y": "relation",
}


class InterfaceSchemaTests(unittest.TestCase):
    def test_schema_documents_are_valid(self):
        Draft202012Validator.check_schema(INTERFACE_SCHEMA)
        Draft202012Validator.check_schema(IMPLEMENTATION_SCHEMA)

    def test_interface_target_shapes_are_closed(self):
        valid = iface(
            "iface:A",
            properties=["prop:req"],
            applications=[
                {"constraint": "constraint:dim", "target": {"kind": "property_requirement", "requirement": "prop:req"}},
                {"constraint": "constraint:whole", "target": {"kind": "interface"}},
            ],
        )
        INTERFACE_VALIDATOR.validate(valid)
        invalid = iface(
            "iface:A",
            applications=[
                {"constraint": "constraint:whole", "target": {"kind": "interface", "requirement": "prop:req"}},
            ],
        )
        with self.assertRaises(ValidationError):
            INTERFACE_VALIDATOR.validate(invalid)

    def test_extension_cycle_is_rejected(self):
        interfaces = [iface("iface:A", extends=["iface:B"]), iface("iface:B", extends=["iface:A"])]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_EXTENSION_CYCLE"):
            effective_interface_contract("iface:A", interfaces)

    def test_diamond_extension_collapses_requirement_identity(self):
        interfaces = [
            iface("iface:A", properties=["prop:req"]),
            iface("iface:B", extends=["iface:A"]),
            iface("iface:C", extends=["iface:A"]),
            iface("iface:D", extends=["iface:B", "iface:C"]),
        ]
        contract = effective_interface_contract("iface:D", interfaces)
        self.assertEqual(contract["properties"], frozenset({"prop:req"}))
        self.assertEqual(contract["interfaces"], frozenset({"iface:A", "iface:B", "iface:C", "iface:D"}))

    def test_constraint_application_target_and_admissibility(self):
        application = {
            "constraint": "constraint:dim",
            "target": {"kind": "property_requirement", "requirement": "prop:req"},
        }
        contract = effective_interface_contract("iface:A", [iface("iface:A", properties=["prop:req"], applications=[application])])
        definitions = [{"id": "constraint:dim", "interface_application_target_kinds": ["property_requirement"]}]
        self.assertTrue(validate_constraint_applications(contract, definitions))

        bad_definitions = [{"id": "constraint:dim", "interface_application_target_kinds": ["relation_requirement"]}]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE"):
            validate_constraint_applications(contract, bad_definitions)

    def test_constraint_target_unknown_and_kind_mismatch(self):
        unknown = {
            "constraint": "constraint:dim",
            "target": {"kind": "property_requirement", "requirement": "prop:missing"},
        }
        contract = effective_interface_contract("iface:A", [iface("iface:A", properties=["prop:req"], applications=[unknown])])
        definitions = [{"id": "constraint:dim", "interface_application_target_kinds": ["property_requirement"]}]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONSTRAINT_TARGET_UNKNOWN"):
            validate_constraint_applications(contract, definitions)

        mismatch = {
            "constraint": "constraint:dim",
            "target": {"kind": "relation_requirement", "requirement": "prop:req"},
        }
        contract = effective_interface_contract("iface:A", [iface("iface:A", properties=["prop:req"], applications=[mismatch])])
        definitions = [{"id": "constraint:dim", "interface_application_target_kinds": ["relation_requirement"]}]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH"):
            validate_constraint_applications(contract, definitions)

    def test_constraint_target_contract_missing_or_invalid(self):
        application = {"constraint": "constraint:c", "target": {"kind": "interface"}}
        contract = effective_interface_contract("iface:A", [iface("iface:A", applications=[application])])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONSTRAINT_TARGET_CONTRACT_MISSING"):
            validate_constraint_applications(contract, [{"id": "constraint:c"}])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID"):
            validate_constraint_applications(contract, [{"id": "constraint:c", "interface_application_target_kinds": []}])

    def test_duplicate_direct_declaration_is_rejected(self):
        declaration = impl("entity:E", "iface:A")
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_IMPLEMENTATION_DUPLICATE_DIRECT"):
            validate_direct_declaration_uniqueness([declaration, dict(declaration)])

    def test_direct_mapping_completeness_and_kind(self):
        interfaces = [iface("iface:A", properties=["prop:req"], relations=["rel:req"])]
        valid = impl("entity:E", "iface:A", properties=[("prop:req", "prop:x")], relations=[("rel:req", "rel:x")])
        validate_direct_implementation(valid, interfaces, KINDS)

        missing = impl("entity:E", "iface:A", relations=[("rel:req", "rel:x")])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_REQUIREMENT_MAPPING_MISSING"):
            validate_direct_implementation(missing, interfaces, KINDS)

        duplicate = impl("entity:E", "iface:A", properties=[("prop:req", "prop:x"), ("prop:req", "prop:x")], relations=[("rel:req", "rel:x")])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS"):
            validate_direct_implementation(duplicate, interfaces, KINDS)

        wrong_kind = impl("entity:E", "iface:A", properties=[("prop:req", "rel:x")], relations=[("rel:req", "rel:x")])
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_REQUIREMENT_KIND_MISMATCH"):
            validate_direct_implementation(wrong_kind, interfaces, KINDS)

    def test_sibling_overlap_converges_or_conflicts(self):
        interfaces = [iface("iface:A", properties=["prop:req"]), iface("iface:B", properties=["prop:req"])]
        same = [
            impl("entity:E", "iface:A", properties=[("prop:req", "prop:x")]),
            impl("entity:E", "iface:B", properties=[("prop:req", "prop:x")]),
        ]
        result = effective_entity_conformance(
            "entity:E", entity_parent={"entity:E": None}, interfaces=interfaces, implementations=same, definition_kinds=KINDS
        )
        self.assertEqual(result["mappings"][("property", "prop:req")], "prop:x")

        conflict = [same[0], impl("entity:E", "iface:B", properties=[("prop:req", "prop:y")])]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_EFFECTIVE_MAPPING_CONFLICT"):
            effective_entity_conformance(
                "entity:E", entity_parent={"entity:E": None}, interfaces=interfaces, implementations=conflict, definition_kinds=KINDS
            )

    def test_child_interface_plus_redundant_parent_is_order_independent(self):
        interfaces = [iface("iface:A", properties=["prop:req"]), iface("iface:B", extends=["iface:A"])]
        declarations = [
            impl("entity:E", "iface:B", properties=[("prop:req", "prop:x")]),
            impl("entity:E", "iface:A", properties=[("prop:req", "prop:x")]),
        ]
        first = effective_entity_conformance(
            "entity:E", entity_parent={"entity:E": None}, interfaces=interfaces, implementations=declarations, definition_kinds=KINDS
        )
        second = effective_entity_conformance(
            "entity:E", entity_parent={"entity:E": None}, interfaces=list(reversed(interfaces)), implementations=list(reversed(declarations)), definition_kinds=KINDS
        )
        self.assertEqual(first["interfaces"], second["interfaces"])
        self.assertEqual(first["mappings"], second["mappings"])

    def test_entity_subtype_inherits_interface_guarantee(self):
        interfaces = [iface("iface:A", properties=["prop:req"]), iface("iface:B")]
        declarations = [impl("entity:Parent", "iface:A", properties=[("prop:req", "prop:x")])]
        result = effective_entity_conformance(
            "entity:Child",
            entity_parent={"entity:Child": "entity:Parent", "entity:Parent": None},
            interfaces=interfaces,
            implementations=declarations,
            definition_kinds=KINDS,
        )
        self.assertIn("iface:A", result["interfaces"])
        self.assertEqual(result["mappings"][("property", "prop:req")], "prop:x")

    def test_child_adds_interface_without_losing_parent(self):
        interfaces = [iface("iface:A", properties=["prop:req"]), iface("iface:B", relations=["rel:req"])]
        declarations = [
            impl("entity:Parent", "iface:A", properties=[("prop:req", "prop:x")]),
            impl("entity:Child", "iface:B", relations=[("rel:req", "rel:x")]),
        ]
        result = effective_entity_conformance(
            "entity:Child",
            entity_parent={"entity:Child": "entity:Parent", "entity:Parent": None},
            interfaces=interfaces,
            implementations=declarations,
            definition_kinds=KINDS,
        )
        self.assertEqual(result["interfaces"], frozenset({"iface:A", "iface:B"}))

    def test_child_conflicting_remap_of_inherited_requirement_fails(self):
        interfaces = [iface("iface:A", properties=["prop:req"]), iface("iface:B", properties=["prop:req"])]
        declarations = [
            impl("entity:Parent", "iface:A", properties=[("prop:req", "prop:x")]),
            impl("entity:Child", "iface:B", properties=[("prop:req", "prop:y")]),
        ]
        with self.assertRaisesRegex(InterfaceContractError, "INTERFACE_EFFECTIVE_MAPPING_CONFLICT"):
            effective_entity_conformance(
                "entity:Child",
                entity_parent={"entity:Child": "entity:Parent", "entity:Parent": None},
                interfaces=interfaces,
                implementations=declarations,
                definition_kinds=KINDS,
            )

    def test_backend_metadata_is_not_structural_input(self):
        declaration = impl("entity:E", "iface:A")
        invalid = dict(declaration, backend="moose")
        with self.assertRaises(ValidationError):
            IMPLEMENTATION_VALIDATOR.validate(invalid)


if __name__ == "__main__":
    unittest.main()
