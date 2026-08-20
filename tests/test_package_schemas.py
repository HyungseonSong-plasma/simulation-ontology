import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schema"
SCHEMAS = {
    path.name: json.loads(path.read_text())
    for path in SCHEMA_DIR.glob("*.schema.json")
}


def registry():
    result = Registry()
    for schema in SCHEMAS.values():
        if "$id" in schema:
            result = result.with_resource(schema["$id"], Resource.from_contents(schema))
    return result


def validator(name):
    return Draft202012Validator(SCHEMAS[name], registry=registry())


def canonical(name):
    return f"https://simulation-ontology.org/id/{name}"


def minimal_package():
    entity = canonical("entity")
    relation = canonical("relation")
    constraint = canonical("constraint")
    return {
        "package": {"name": "@simulation-ontology/core", "version": "0.1.0"},
        "namespaces": [
            {
                "name": "sol",
                "exports": [
                    {"name": "Entity", "kind": "entity_type", "id": entity},
                    {"name": "Relation", "kind": "relation", "id": relation},
                    {"name": "Constraint", "kind": "constraint_definition", "id": constraint},
                ],
            }
        ],
        "resolved_dependencies": [],
        "entity_types": [{"id": entity}],
        "properties": [],
        "relations": [
            {
                "id": relation,
                "endpoint_contract": {
                    "kind": "typed",
                    "matching": "canonical_type_or_subtype",
                    "domain": [entity],
                },
                "source_cardinality_projection": {
                    "constraint": constraint,
                    "min": 0,
                    "max": "unbounded",
                },
            }
        ],
        "constraint_definitions": [
            {
                "id": constraint,
                "payload": {
                    "type": "cardinality",
                    "relation": relation,
                    "direction": "source",
                    "min": 0,
                    "max": "unbounded",
                },
            }
        ],
        "interfaces": [],
        "interface_implementations": [],
    }


class PackageSchemaTests(unittest.TestCase):
    def test_all_package_related_schemas_are_valid_draft_2020_12(self):
        names = [
            "entity-type-definition-v0.1.schema.json",
            "property-definition-v0.1.schema.json",
            "relation-definition-v0.1.schema.json",
            "constraint-definition-v0.1.schema.json",
            "ontology-package-normalized-v0.1.schema.json",
        ]
        for name in names:
            Draft202012Validator.check_schema(SCHEMAS[name])

    def test_minimal_package_resolves_transitive_schema_refs(self):
        validator("ontology-package-normalized-v0.1.schema.json").validate(minimal_package())

    def test_normalized_package_requires_all_resource_collections(self):
        value = minimal_package()
        value.pop("properties")
        with self.assertRaises(ValidationError):
            validator("ontology-package-normalized-v0.1.schema.json").validate(value)

    def test_entity_type_definition_has_no_children_field(self):
        with self.assertRaises(ValidationError):
            validator("entity-type-definition-v0.1.schema.json").validate(
                {"id": canonical("child"), "children": [canonical("x")]}
            )

    def test_relation_typed_endpoint_may_omit_range(self):
        validator("relation-definition-v0.1.schema.json").validate(
            {
                "id": canonical("depends-on"),
                "endpoint_contract": {
                    "kind": "typed",
                    "matching": "canonical_type_or_subtype",
                    "domain": [canonical("value-definition")],
                },
            }
        )

    def test_relation_allowed_pairs_cannot_mix_domain_range(self):
        with self.assertRaises(ValidationError):
            validator("relation-definition-v0.1.schema.json").validate(
                {
                    "id": canonical("includes-component"),
                    "endpoint_contract": {
                        "kind": "allowed_pairs",
                        "matching": "canonical_type_or_subtype",
                        "pairs": [
                            {"source": canonical("model"), "targets": [canonical("field")]}
                        ],
                        "domain": [canonical("model")],
                    },
                }
            )

    def test_cardinality_projection_is_closed(self):
        with self.assertRaises(ValidationError):
            validator("relation-definition-v0.1.schema.json").validate(
                {
                    "id": canonical("r"),
                    "endpoint_contract": {
                        "kind": "typed",
                        "matching": "canonical_type_or_subtype",
                    },
                    "source_cardinality_projection": {
                        "constraint": canonical("c"),
                        "min": 0,
                        "max": "unbounded",
                        "authoritative": True,
                    },
                }
            )

    def test_constraint_definition_wraps_closed_family_payload(self):
        relation = canonical("r")
        value = {
            "id": canonical("c"),
            "payload": {
                "type": "cardinality",
                "relation": relation,
                "direction": "source",
                "min": 0,
                "max": "unbounded",
            },
            "interface_application_target_kinds": ["relation_requirement"],
        }
        validator("constraint-definition-v0.1.schema.json").validate(value)
        invalid = json.loads(json.dumps(value))
        invalid["payload"]["id"] = canonical("embedded")
        with self.assertRaises(ValidationError):
            validator("constraint-definition-v0.1.schema.json").validate(invalid)

    def test_unknown_interface_application_target_kind_fails(self):
        with self.assertRaises(ValidationError):
            validator("constraint-definition-v0.1.schema.json").validate(
                {
                    "id": canonical("c"),
                    "payload": {
                        "type": "cardinality",
                        "relation": canonical("r"),
                        "direction": "source",
                        "min": 0,
                        "max": 1,
                    },
                    "interface_application_target_kinds": ["field"],
                }
            )

    def test_dependency_range_syntax_is_not_allowed_in_normalized_snapshot(self):
        value = minimal_package()
        value["resolved_dependencies"] = [
            {"package": "@simulation-ontology/base", "version": ">=0.1.0 <0.2.0"}
        ]
        with self.assertRaises(ValidationError):
            validator("ontology-package-normalized-v0.1.schema.json").validate(value)

    def test_semver_numeric_prerelease_leading_zero_is_invalid(self):
        value = minimal_package()
        value["package"]["version"] = "1.0.0-01"
        with self.assertRaises(ValidationError):
            validator("ontology-package-normalized-v0.1.schema.json").validate(value)

    def test_semver_empty_prerelease_identifier_is_invalid(self):
        value = minimal_package()
        value["package"]["version"] = "1.0.0-alpha..1"
        with self.assertRaises(ValidationError):
            validator("ontology-package-normalized-v0.1.schema.json").validate(value)

    def test_semver_valid_prerelease_and_build_passes(self):
        value = minimal_package()
        value["package"]["version"] = "1.0.0-alpha.1+build.7"
        validator("ontology-package-normalized-v0.1.schema.json").validate(value)


if __name__ == "__main__":
    unittest.main()
