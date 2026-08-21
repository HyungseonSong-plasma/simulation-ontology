use sol_adapter_protocol::{
    assess_compatibility, AdapterDescription, CompatibilityOutcome, CompatibilitySupport,
};
use sol_mock_adapter::{MockAdapter, MOCK_ADAPTER_ID, MOCK_CAPABILITY_REVISION, MOCK_TARGET};

const PUBLISHED_DESCRIPTION: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/dual-compatible-description.json");

#[test]
fn mock_adapter_description_reuses_published_protocol_shape() {
    let adapter = MockAdapter::thermal();
    let description = adapter.describe_adapter().unwrap();
    let published = AdapterDescription::from_json(PUBLISHED_DESCRIPTION).unwrap();

    assert_eq!(description.bootstrap.adapter_id, MOCK_ADAPTER_ID);
    assert_eq!(description.targets.len(), 1);
    assert_eq!(description.targets[0].target, MOCK_TARGET);

    let actual = description.targets[0]
        .capabilities
        .iter()
        .map(|capability| (&capability.capability, capability.revision.as_deref()))
        .collect::<Vec<_>>();
    let expected = published.targets[0]
        .capabilities
        .iter()
        .map(|capability| (&capability.capability, capability.revision.as_deref()))
        .collect::<Vec<_>>();

    assert_eq!(actual, expected);
    assert_eq!(
        description.targets[0].capabilities[2].revision.as_deref(),
        Some(MOCK_CAPABILITY_REVISION)
    );
}

#[test]
fn mock_adapter_description_is_dual_compatible_with_current_core_support() {
    let description = MockAdapter::thermal().describe_adapter().unwrap();
    let assessment =
        assess_compatibility(&CompatibilitySupport::current(), &description.bootstrap).unwrap();

    assert_eq!(assessment.adapter_protocol.outcome, CompatibilityOutcome::Compatible);
    assert_eq!(assessment.public_contract.outcome, CompatibilityOutcome::Compatible);
    assert_eq!(assessment.overall, CompatibilityOutcome::Compatible);
    assert_eq!(assessment.adapter_protocol.selected_version.as_deref(), Some("0.1"));
    assert_eq!(assessment.public_contract.selected_version.as_deref(), Some("0.1"));
}

#[test]
fn adapter_package_version_does_not_imply_or_change_semantic_compatibility() {
    let description = MockAdapter::thermal().describe_adapter().unwrap();
    let mut changed = description.bootstrap.clone();
    changed.adapter_version = "9999.42-local-build".to_owned();

    let original =
        assess_compatibility(&CompatibilitySupport::current(), &description.bootstrap).unwrap();
    let changed = assess_compatibility(&CompatibilitySupport::current(), &changed).unwrap();

    assert_eq!(original.overall, CompatibilityOutcome::Compatible);
    assert_eq!(changed.overall, CompatibilityOutcome::Compatible);
    assert_eq!(original.adapter_protocol, changed.adapter_protocol);
    assert_eq!(original.public_contract, changed.public_contract);
}

#[test]
fn missing_or_incompatible_support_is_not_treated_as_compatible() {
    let description = MockAdapter::thermal().describe_adapter().unwrap();

    let mut missing = description.bootstrap.clone();
    missing.supported_adapter_protocol_versions = None;
    let missing = assess_compatibility(&CompatibilitySupport::current(), &missing).unwrap();
    assert_eq!(missing.overall, CompatibilityOutcome::Unknown);

    let mut incompatible = description.bootstrap.clone();
    incompatible.supported_adapter_protocol_versions = Some(vec!["9.9".to_owned()]);
    let incompatible =
        assess_compatibility(&CompatibilitySupport::current(), &incompatible).unwrap();
    assert_eq!(incompatible.adapter_protocol.outcome, CompatibilityOutcome::Incompatible);
    assert_eq!(incompatible.overall, CompatibilityOutcome::Incompatible);
}

#[test]
fn description_is_canonical_and_deterministic() {
    let first = MockAdapter::thermal()
        .describe_adapter()
        .unwrap()
        .to_canonical_json()
        .unwrap();
    let second = MockAdapter::thermal()
        .describe_adapter()
        .unwrap()
        .to_canonical_json()
        .unwrap();

    assert_eq!(first, second);
    assert!(first.contains("thermal.domain"));
    assert!(first.contains("thermal.material"));
    assert!(first.contains("thermal.solve"));
    assert!(!first.contains("jsonrpc"));
    assert!(!first.contains("stdio"));
    assert!(!first.contains("backend_object"));
}
