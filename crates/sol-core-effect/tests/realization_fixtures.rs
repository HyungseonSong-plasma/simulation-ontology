use serde::Deserialize;
use sol_core_effect::{
    compare_semantics, RealizationEffect, RealizationQuality, SemanticComparison,
};
use sol_core_mapping::MappingSubjectRef;

#[derive(Debug, Deserialize)]
struct RealizationFixture {
    subject: String,
    quality: String,
    detail: String,
}

fn load_fixture(raw: &str) -> RealizationEffect {
    let fixture: RealizationFixture = serde_json::from_str(raw).unwrap();
    let quality = match fixture.quality.as_str() {
        "exact" => RealizationQuality::Exact,
        "degraded" => RealizationQuality::Degraded,
        other => panic!("unexpected realization quality: {other}"),
    };

    RealizationEffect::new(
        MappingSubjectRef::Entity(fixture.subject.parse().unwrap()),
        quality,
    )
    .with_detail(fixture.detail)
}

#[test]
fn exact_and_degraded_thermal_fixtures_are_distinguishable() {
    let exact = load_fixture(include_str!(
        "../../../fixtures/golden/thermal-realization-exact.json"
    ));
    let degraded = load_fixture(include_str!(
        "../../../fixtures/counterexamples/thermal-realization-degraded.json"
    ));
    let intended = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());

    assert_eq!(
        compare_semantics(&intended, &exact),
        SemanticComparison::Exact
    );
    assert_eq!(
        compare_semantics(&intended, &degraded),
        SemanticComparison::Degraded
    );
    assert_ne!(exact.quality, degraded.quality);
}
