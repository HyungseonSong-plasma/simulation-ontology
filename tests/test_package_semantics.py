import copy
import unittest

from tests.package_semantics import PackageContractError, validate_resolved_environment


PREFIX = "https://simulation-ontology.org/id/"
E = PREFIX + "entity-e"
E2 = PREFIX + "entity-e2"
P = PREFIX + "property-p"
R = PREFIX + "relation-r"
C = PREFIX + "constraint-c"
I = PREFIX + "interface-i"


def base_package():
    return {
        "package": {"name": "@simulation-ontology/core", "version": "0.1.0"},
        "namespaces": [
            {
                "name": "sol",
                "exports": [
                    {"name": "E", "kind": "entity_type", "id": E},
                    {"name": "P", "kind": "property", "id": P},
                    {"name": "R", "kind": "relation", "id": R},
                    {"name": "C", "kind": "constraint_definition", "id": C},
                    {"name": "I", "kind": "interface", "id": I},
                ],
            }
        ],
        "resolved_dependencies": [],
        "entity_types": [{"id": E}],
        "properties": [{"id": P}],
        "relations": [
            {
                "id": R,
                "endpoint_contract": {
                    "kind": "typed",
                    "matching": "canonical_type_or_subtype",
                    "domain": [E],
                    "range": [E],
                },
                "source_cardinality_projection": {"constraint": C, "min": 0, "max": "unbounded"},
            }
        ],
        "constraint_definitions": [
            {
                "id": C,
                "payload": {
                    "type": "cardinality",
                    "relation": R,
                    "direction": "source",
                    "min": 0,
                    "max": "unbounded",
                },
                "interface_application_target_kinds": ["relation_requirement"],
            }
        ],
        "interfaces": [
            {
                "id": I,
                "extends": [],
                "property_requirements": [{"property": P}],
                "relation_requirements": [{"relation": R}],
                "constraint_applications": [
                    {"constraint": C, "target": {"kind": "relation_requirement", "requirement": R}}
                ],
            }
        ],
        "interface_implementations": [
            {
                "entity_type": E,
                "interface": I,
                "property_mappings": [{"requirement": P, "concrete": P}],
                "relation_mappings": [{"requirement": R, "concrete": R}],
            }
        ],
    }


def empty_package(name="dep", version="0.1.0", namespace="dep"):
    return {
        "package": {"name": name, "version": version},
        "namespaces": [{"name": namespace, "exports": []}],
        "resolved_dependencies": [],
        "entity_types": [],
        "properties": [],
        "relations": [],
        "constraint_definitions": [],
        "interfaces": [],
        "interface_implementations": [],
    }


class PackageSemanticsTests(unittest.TestCase):
    def test_valid_package_passes(self):
        index = validate_resolved_environment([base_package()])
        self.assertEqual(index[E][0], "entity_type")
        self.assertEqual(index[R][0], "relation")

    def test_alias_names_may_share_one_canonical_id(self):
        package = base_package()
        package["namespaces"][0]["exports"].append({"name": "OldE", "kind": "entity_type", "id": E})
        validate_resolved_environment([package])

    def test_same_export_name_to_two_ids_is_ambiguous(self):
        package = base_package()
        package["entity_types"].append({"id": E2})
        package["namespaces"][0]["exports"].append({"name": "E", "kind": "entity_type", "id": E2})
        with self.assertRaisesRegex(PackageContractError, "NAMESPACE_EXPORT_AMBIGUOUS"):
            validate_resolved_environment([package])

    def test_two_active_namespace_providers_fail(self):
        first = base_package()
        second = empty_package(name="other", namespace="sol")
        with self.assertRaisesRegex(PackageContractError, "NAMESPACE_PROVIDER_AMBIGUOUS"):
            validate_resolved_environment([first, second])

    def test_duplicate_active_package_identity_fails(self):
        first = empty_package(name="dep", namespace="dep-a")
        second = empty_package(name="dep", namespace="dep-b")
        with self.assertRaisesRegex(PackageContractError, "PACKAGE_IDENTITY_DUPLICATE_ACTIVE"):
            validate_resolved_environment([first, second])

    def test_exact_resolved_dependency_must_be_active(self):
        package = base_package()
        package["resolved_dependencies"] = [{"package": "dep", "version": "0.1.0"}]
        with self.assertRaisesRegex(PackageContractError, "RESOLVED_DEPENDENCY_UNRESOLVED"):
            validate_resolved_environment([package])

        dependency = empty_package(name="dep", version="0.1.0", namespace="dep")
        validate_resolved_environment([package, dependency])

    def test_resolved_dependency_wrong_active_version_is_unresolved(self):
        package = base_package()
        package["resolved_dependencies"] = [{"package": "dep", "version": "0.1.0"}]
        dependency = empty_package(name="dep", version="0.2.0", namespace="dep")
        with self.assertRaisesRegex(PackageContractError, "RESOLVED_DEPENDENCY_UNRESOLVED"):
            validate_resolved_environment([package, dependency])

    def test_relation_endpoint_property_id_is_kind_mismatch(self):
        package = base_package()
        package["relations"][0]["endpoint_contract"]["range"] = [P]
        with self.assertRaisesRegex(PackageContractError, "RELATION_ENDPOINT_KIND_MISMATCH"):
            validate_resolved_environment([package])

    def test_open_typed_endpoint_is_deliberate_not_unresolved(self):
        package = base_package()
        package["relations"][0]["endpoint_contract"].pop("range")
        validate_resolved_environment([package])

    def test_allowed_pairs_relation_resolves_entity_kinds(self):
        package = base_package()
        package["relations"][0]["endpoint_contract"] = {
            "kind": "allowed_pairs",
            "matching": "canonical_type_or_subtype",
            "pairs": [{"source": E, "targets": [E]}],
        }
        validate_resolved_environment([package])

    def test_allowed_pairs_duplicate_source_is_not_normalized(self):
        package = base_package()
        package["entity_types"].append({"id": E2})
        package["relations"][0]["endpoint_contract"] = {
            "kind": "allowed_pairs",
            "matching": "canonical_type_or_subtype",
            "pairs": [
                {"source": E, "targets": [E]},
                {"source": E, "targets": [E2]},
            ],
        }
        with self.assertRaisesRegex(PackageContractError, "RELATION_ALLOWED_PAIRS_NOT_NORMALIZED"):
            validate_resolved_environment([package])

    def test_cardinality_projection_wrong_relation_fails(self):
        package = base_package()
        package["constraint_definitions"][0]["payload"]["relation"] = PREFIX + "other-relation"
        with self.assertRaisesRegex(PackageContractError, "CARDINALITY_PROJECTION_AUTHORITY_INVALID"):
            validate_resolved_environment([package])

    def test_cardinality_projection_interval_mismatch_fails(self):
        package = base_package()
        package["relations"][0]["source_cardinality_projection"]["min"] = 1
        with self.assertRaisesRegex(PackageContractError, "CARDINALITY_PROJECTION_MISMATCH"):
            validate_resolved_environment([package])

    def test_interface_requirement_unresolved_fails(self):
        package = base_package()
        package["interfaces"][0]["property_requirements"] = [{"property": PREFIX + "missing"}]
        with self.assertRaisesRegex(PackageContractError, "INTERFACE_REQUIREMENT_DEFINITION_UNRESOLVED"):
            validate_resolved_environment([package])

    def test_backend_local_handle_is_not_canonical_reference(self):
        package = base_package()
        package["relations"][0]["endpoint_contract"]["domain"] = ["moose_local_name"]
        with self.assertRaisesRegex(PackageContractError, "CANONICAL_REFERENCE_INVALID"):
            validate_resolved_environment([package])

    def test_declaration_order_does_not_change_resource_identity_set(self):
        package = base_package()
        first = validate_resolved_environment([package])
        reordered = copy.deepcopy(package)
        reordered["namespaces"][0]["exports"].reverse()
        reordered["entity_types"].reverse()
        reordered["interfaces"].reverse()
        second = validate_resolved_environment([reordered])
        self.assertEqual(set(first), set(second))

    def test_same_id_can_evolve_across_historical_versions(self):
        old = base_package()
        new = base_package()
        new["package"]["version"] = "0.2.0"
        new["entity_types"].append({"id": E2, "is_a": E})
        new["namespaces"][0]["exports"].append({"name": "E2", "kind": "entity_type", "id": E2})
        self.assertIn(E, validate_resolved_environment([old]))
        self.assertIn(E, validate_resolved_environment([new]))

    def test_conflicting_duplicate_active_resource_id_fails(self):
        package = base_package()
        package["entity_types"].append({"id": E, "is_a": E2})
        with self.assertRaisesRegex(PackageContractError, "CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT"):
            validate_resolved_environment([package])


if __name__ == "__main__":
    unittest.main()
