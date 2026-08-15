//! TEST-RUST-UNIT-001 / REQ-AEO-001.0

use aeo_fixture_desk_core::{
    build_fixture_dashboard_data, build_intent_evidence_data, calculate_intent_presence_rates,
    evaluate_answer_alias_match, find_intent_evidence_rows, load_embedded_fixture_bundle,
    parse_fixture_bundle_text, select_priority_intent_gap, AppError,
};

#[test]
fn test_req_aeo_001_loads_exact_embedded_fixture_counts() {
    let bundle = load_embedded_fixture_bundle().expect("the authored fixture should be valid");

    assert_eq!(bundle.intents.len(), 3);
    assert_eq!(bundle.prompts.len(), 6);
    assert_eq!(bundle.providers.len(), 3);
    assert_eq!(bundle.observations.len(), 18);
    assert_eq!(bundle.expected_scores.len(), 18);
}

/// TEST-RUST-UNIT-002 / REQ-AEO-001.0
#[test]
fn test_req_aeo_001_rejects_invalid_fixture_shapes() {
    assert!(matches!(
        parse_fixture_bundle_text("{ this is not JSON"),
        Err(AppError::InvalidFixture { .. })
    ));

    let mut duplicate_identifier_bundle = load_embedded_fixture_bundle().expect("valid fixture");
    duplicate_identifier_bundle.intents[1].id = "discovery".to_owned();
    assert!(matches!(
        aeo_fixture_desk_core::validate_fixture_bundle_shape(&duplicate_identifier_bundle),
        Err(AppError::InvalidFixture { .. })
    ));

    let mut missing_observation_bundle = load_embedded_fixture_bundle().expect("valid fixture");
    missing_observation_bundle.observations.pop();
    assert!(matches!(
        aeo_fixture_desk_core::validate_fixture_bundle_shape(&missing_observation_bundle),
        Err(AppError::InvalidFixture { .. })
    ));

    let mut missing_link_bundle = load_embedded_fixture_bundle().expect("valid fixture");
    missing_link_bundle.observations[0].prompt_id = "unknown-prompt".to_owned();
    assert!(matches!(
        aeo_fixture_desk_core::validate_fixture_bundle_shape(&missing_link_bundle),
        Err(AppError::InvalidFixture { .. })
    ));
}

/// TEST-RUST-UNIT-003 / REQ-AEO-002.0
#[test]
fn test_req_aeo_002_calculates_committed_fixture_counts() {
    let bundle = load_embedded_fixture_bundle().expect("valid fixture");
    let intent_rates = calculate_intent_presence_rates(&bundle);

    let expected_summary: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/expected_fixture_summary.json"))
            .expect("valid expected summary");

    for intent_rate in &intent_rates {
        let expected_intent = &expected_summary["intent_counts"][&intent_rate.intent_id];
        assert_eq!(
            intent_rate.mentioned as u64,
            expected_intent["mentioned"]
                .as_u64()
                .expect("expected count")
        );
        assert_eq!(
            intent_rate.total as u64,
            expected_intent["total"].as_u64().expect("expected count")
        );

        for provider_cell in &intent_rate.provider_cells {
            let expected_cell = &expected_summary["provider_counts"][&intent_rate.intent_id]
                [&provider_cell.provider_id];
            assert_eq!(
                provider_cell.mentioned as u64,
                expected_cell["mentioned"].as_u64().expect("expected count")
            );
            assert_eq!(
                provider_cell.total as u64,
                expected_cell["total"].as_u64().expect("expected count")
            );
        }
    }

    for observation in &bundle.observations {
        let expected_score = bundle
            .expected_scores
            .iter()
            .find(|score| score.observation_id == observation.id)
            .expect("every observation has an expected score");
        let outcome = evaluate_answer_alias_match(&observation.answer_text, &bundle.brand_aliases);
        assert_eq!(outcome.brand_mentioned, expected_score.brand_mentioned);
    }
}

/// TEST-RUST-UNIT-004 / REQ-AEO-002.0
#[test]
fn test_req_aeo_002_matches_aliases_case_insensitively_without_substrings() {
    let bundle = load_embedded_fixture_bundle().expect("valid fixture");

    let case_insensitive = evaluate_answer_alias_match(
        "EVERPURE is mentioned here as fixture text.",
        &bundle.brand_aliases,
    );
    assert!(case_insensitive.brand_mentioned);
    assert_eq!(case_insensitive.matched_alias.as_deref(), Some("EverPure"));

    let substring_only = evaluate_answer_alias_match(
        "EverPurest must not count as the configured alias.",
        &bundle.brand_aliases,
    );
    assert!(!substring_only.brand_mentioned);
    assert_eq!(substring_only.matched_alias, None);
}

/// TEST-RUST-UNIT-005 and TEST-RUST-UNIT-006 / REQ-AEO-003.0
#[test]
fn test_req_aeo_003_selects_lowest_rate_and_stable_tie_break() {
    let bundle = load_embedded_fixture_bundle().expect("valid fixture");
    let intent_rates = calculate_intent_presence_rates(&bundle);
    let priority = select_priority_intent_gap(&intent_rates, &bundle.intents).expect("priority");

    assert_eq!(priority.intent_id, "comparison");
    assert_eq!(priority.mentioned, 1);
    assert_eq!(priority.total, 6);
    assert!(priority
        .action_template
        .starts_with("Investigate comparison content"));

    let mut tied_rates = intent_rates;
    for rate in &mut tied_rates {
        rate.mentioned = 1;
        rate.total = 6;
    }
    let tied_priority = select_priority_intent_gap(&tied_rates, &bundle.intents).expect("priority");
    assert_eq!(tied_priority.intent_id, "comparison");
}

/// TEST-RUST-INTEG-007 and TEST-RUST-INTEG-008 / REQ-AEO-004.0
#[test]
fn test_req_aeo_004_returns_six_inspectable_rows_or_typed_error() {
    let bundle = load_embedded_fixture_bundle().expect("valid fixture");
    let evidence_rows =
        find_intent_evidence_rows(&bundle, "comparison").expect("comparison evidence");

    assert_eq!(evidence_rows.len(), 6);
    assert_eq!(
        evidence_rows
            .iter()
            .filter(|row| row.mention.brand_mentioned)
            .count(),
        1
    );
    assert!(evidence_rows
        .iter()
        .all(|row| row.source_type == "authored_demo_fixture"));

    assert!(matches!(
        find_intent_evidence_rows(&bundle, "missing-intent"),
        Err(AppError::IntentNotFound { intent_id }) if intent_id == "missing-intent"
    ));
}

/// Core presentation contract used by TEST-RUST-INTEG-013 and the Tauri UI tests.
#[test]
fn test_fixture_presentation_data_keeps_provenance_and_action_visible() {
    let dashboard = build_fixture_dashboard_data().expect("dashboard data");
    assert_eq!(dashboard.intent_rates.len(), 3);
    assert_eq!(dashboard.providers.len(), 3);
    assert_eq!(dashboard.priority.intent_id, "comparison");
    assert_eq!(dashboard.priority.mentioned, 1);
    assert_eq!(dashboard.priority.total, 6);
    assert!(dashboard.fixture_notice.starts_with("Fixture analysis"));
    assert!(dashboard
        .scoring_rule
        .contains("case-insensitive whole-word"));

    let evidence = build_intent_evidence_data("comparison").expect("evidence data");
    assert_eq!(evidence.intent.intent_id, "comparison");
    assert_eq!(evidence.evidence_rows.len(), 6);
    assert_eq!(evidence.priority, dashboard.priority);
    assert_eq!(evidence.fixture_notice, dashboard.fixture_notice);
}
