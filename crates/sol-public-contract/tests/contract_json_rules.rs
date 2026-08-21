use sol_public_contract::{
    require_closed_enum_value, CanonicalDocument, ContractDocumentError, ContractVersion,
};

const VALID_EXTENSIBLE: &str = include_str!(
    "../../../fixtures/public-contract/0.1/valid-extensible-document.json"
);
const MISSING_VERSION: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-missing-version.json"
);
const MALFORMED_VERSION: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-malformed-version.json"
);
const UNSUPPORTED_VERSION: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-unsupported-version.json"
);

#[test]
fn supported_contract_version_is_accepted() {
    let document = CanonicalDocument::parse(VALID_EXTENSIBLE).unwrap();
    assert_eq!(document.version(), ContractVersion::supported());
}

#[test]
fn missing_contract_version_is_distinct_from_ontology_version() {
    assert_eq!(
        CanonicalDocument::parse(MISSING_VERSION),
        Err(ContractDocumentError::MissingVersion)
    );
}

#[test]
fn malformed_contract_version_is_rejected_without_fallback() {
    assert_eq!(
        CanonicalDocument::parse(MALFORMED_VERSION),
        Err(ContractDocumentError::MalformedVersion("0.1.0".to_owned()))
    );
}

#[test]
fn unsupported_contract_version_is_rejected_without_downgrade() {
    assert_eq!(
        CanonicalDocument::parse(UNSUPPORTED_VERSION),
        Err(ContractDocumentError::UnsupportedVersion("0.2".to_owned()))
    );
}

#[test]
fn unknown_optional_fields_survive_generic_canonicalization() {
    let document = CanonicalDocument::parse(VALID_EXTENSIBLE).unwrap();
    let canonical = document.to_canonical_json().unwrap();

    assert!(canonical.contains("\"future_optional\""));
    assert!(canonical.contains("\"opaque\":true"));
}

#[test]
fn object_key_order_is_not_semantic() {
    let left = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","payload":{"b":2,"a":1}}"#,
    )
    .unwrap();
    let right = CanonicalDocument::parse(
        r#"{"payload":{"a":1,"b":2},"public_contract_version":"0.1"}"#,
    )
    .unwrap();

    assert_eq!(left.to_canonical_json().unwrap(), right.to_canonical_json().unwrap());
}

#[test]
fn array_order_is_preserved_by_default() {
    let left = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","items":["a","b"]}"#,
    )
    .unwrap();
    let right = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","items":["b","a"]}"#,
    )
    .unwrap();

    assert_ne!(left.to_canonical_json().unwrap(), right.to_canonical_json().unwrap());
}

#[test]
fn generic_normalization_does_not_coerce_json_types() {
    let string_value = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","value":"1"}"#,
    )
    .unwrap();
    let numeric_value = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","value":1}"#,
    )
    .unwrap();

    assert_ne!(
        string_value.to_canonical_json().unwrap(),
        numeric_value.to_canonical_json().unwrap()
    );
}

#[test]
fn absent_optional_field_is_distinct_from_explicit_null() {
    let absent = CanonicalDocument::parse(r#"{"public_contract_version":"0.1"}"#).unwrap();
    let explicit_null = CanonicalDocument::parse(
        r#"{"public_contract_version":"0.1","optional":null}"#,
    )
    .unwrap();

    assert_ne!(absent.to_canonical_json().unwrap(), explicit_null.to_canonical_json().unwrap());
}

#[test]
fn unknown_closed_enum_value_is_rejected_not_mapped_to_lifecycle_unknown() {
    assert_eq!(
        require_closed_enum_value("pass", &["pass", "fail", "blocked", "indeterminate"]),
        Ok("pass")
    );
    assert!(require_closed_enum_value(
        "future_state",
        &["pass", "fail", "blocked", "indeterminate"]
    )
    .is_err());
}
