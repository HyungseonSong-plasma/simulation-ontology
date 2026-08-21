use sol_adapter_protocol::{
    assess_compatibility, parse_bootstrap, AdapterDescription, CompatibilityOutcome,
    CompatibilitySupport, ProtocolError,
};

const DUAL_COMPATIBLE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/dual-compatible-description.json");
const HIGHEST_COMMON: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/highest-common-description.json");
const VERSION_SAFE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/version-safe-bootstrap.json");
const PROTOCOL_INCOMPATIBLE: &str =
    include_str!("../../../fixtures/counterexamples/adapter-protocol-protocol-incompatible.json");
const PUBLIC_CONTRACT_INCOMPATIBLE: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-public-contract-incompatible.json"
);
const MISSING_PROTOCOL: &str =
    include_str!("../../../fixtures/counterexamples/adapter-protocol-missing-protocol-support.json");
const MISSING_PUBLIC_CONTRACT: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-missing-public-contract-support.json"
);
const MALFORMED_VERSION: &str =
    include_str!("../../../fixtures/counterexamples/adapter-protocol-malformed-version.json");
const MALFORMED_SUPPORT_TYPE: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-malformed-support-type.json"
);
const TRANSPORT_LEAKAGE: &str =
    include_str!("../../../fixtures/counterexamples/adapter-protocol-transport-leakage.json");

#[test]
fn exact_dual_compatibility_is_required() {
    let bootstrap = parse_bootstrap(DUAL_COMPATIBLE).unwrap();
    let assessment = assess_compatibility(&CompatibilitySupport::current(), &bootstrap).unwrap();

    assert_eq!(
        assessment.adapter_protocol.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        assessment.adapter_protocol.selected_version.as_deref(),
        Some("0.1")
    );
    assert_eq!(
        assessment.public_contract.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        assessment.public_contract.selected_version.as_deref(),
        Some("0.1")
    );
    assert_eq!(assessment.overall, CompatibilityOutcome::Compatible);
}

#[test]
fn highest_common_version_is_selected_deterministically() {
    let bootstrap = parse_bootstrap(HIGHEST_COMMON).unwrap();
    let core = CompatibilitySupport {
        adapter_protocol_versions: Some(vec!["0.1".into(), "0.2".into()]),
        public_contract_versions: Some(vec!["0.1".into(), "0.2".into()]),
    };
    let assessment = assess_compatibility(&core, &bootstrap).unwrap();

    assert_eq!(
        assessment.adapter_protocol.selected_version.as_deref(),
        Some("0.2")
    );
    assert_eq!(
        assessment.public_contract.selected_version.as_deref(),
        Some("0.2")
    );
    assert_eq!(assessment.overall, CompatibilityOutcome::Compatible);
}

#[test]
fn protocol_compatible_is_insufficient_when_public_contract_is_incompatible() {
    let bootstrap = parse_bootstrap(PUBLIC_CONTRACT_INCOMPATIBLE).unwrap();
    let assessment = assess_compatibility(&CompatibilitySupport::current(), &bootstrap).unwrap();

    assert_eq!(
        assessment.adapter_protocol.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        assessment.public_contract.outcome,
        CompatibilityOutcome::Incompatible
    );
    assert_eq!(assessment.overall, CompatibilityOutcome::Incompatible);
}

#[test]
fn public_contract_compatible_is_insufficient_when_protocol_is_incompatible() {
    let bootstrap = parse_bootstrap(PROTOCOL_INCOMPATIBLE).unwrap();
    let assessment = assess_compatibility(&CompatibilitySupport::current(), &bootstrap).unwrap();

    assert_eq!(
        assessment.adapter_protocol.outcome,
        CompatibilityOutcome::Incompatible
    );
    assert_eq!(
        assessment.public_contract.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(assessment.overall, CompatibilityOutcome::Incompatible);
}

#[test]
fn missing_support_information_is_unknown_not_compatible() {
    for fixture in [MISSING_PROTOCOL, MISSING_PUBLIC_CONTRACT] {
        let bootstrap = parse_bootstrap(fixture).unwrap();
        let assessment = assess_compatibility(&CompatibilitySupport::current(), &bootstrap).unwrap();
        assert_eq!(assessment.overall, CompatibilityOutcome::Unknown);
    }
}

#[test]
fn malformed_version_is_bootstrap_error_not_unknown() {
    assert!(matches!(
        parse_bootstrap(MALFORMED_VERSION),
        Err(ProtocolError::MalformedProtocolVersion(version)) if version == "0.1.0"
    ));
}

#[test]
fn malformed_support_type_is_bootstrap_error_not_unknown() {
    assert!(matches!(
        parse_bootstrap(MALFORMED_SUPPORT_TYPE),
        Err(ProtocolError::InvalidBootstrap(_))
    ));
}

#[test]
fn adapter_version_never_implies_semantic_compatibility() {
    let bootstrap = parse_bootstrap(PROTOCOL_INCOMPATIBLE).unwrap();
    assert_eq!(bootstrap.adapter_version, "1.4.0");

    let assessment = assess_compatibility(&CompatibilitySupport::current(), &bootstrap).unwrap();
    assert_eq!(assessment.overall, CompatibilityOutcome::Incompatible);
}

#[test]
fn bootstrap_can_be_parsed_before_incompatible_full_description_shape() {
    let bootstrap = parse_bootstrap(VERSION_SAFE).unwrap();
    assert_eq!(bootstrap.adapter_id, "adapter.future_mock");

    assert!(matches!(
        AdapterDescription::from_json(VERSION_SAFE),
        Err(ProtocolError::InvalidDescription(_))
    ));
}

#[test]
fn transport_markers_are_rejected_from_protocol_semantics() {
    assert!(matches!(
        parse_bootstrap(TRANSPORT_LEAKAGE),
        Err(ProtocolError::TransportLeakage(field)) if field == "jsonrpc"
    ));
}

#[test]
fn description_capabilities_are_descriptor_scoped_and_canonicalized() {
    let description = AdapterDescription::from_json(DUAL_COMPATIBLE).unwrap();
    assert_eq!(description.targets.len(), 1);
    assert_eq!(description.targets[0].target, "mock");
    assert_eq!(
        description.targets[0]
            .capabilities
            .iter()
            .map(|capability| capability.capability.as_str())
            .collect::<Vec<_>>(),
        vec!["thermal.domain", "thermal.material", "thermal.solve"]
    );

    let canonical = description.to_canonical_json().unwrap();
    assert!(!canonical.contains("jsonrpc"));
    assert!(!canonical.contains("bootstrap_version"));
}
