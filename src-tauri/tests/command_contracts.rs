//! TEST-RUST-INTEG-013 / REQ-TAURI-004.0

use aeo_fixture_desk_app::{get_fixture_dashboard_data, get_intent_evidence_data};

#[test]
fn test_req_tauri_004_commands_return_typed_data_and_serializable_errors() {
    let dashboard = get_fixture_dashboard_data().expect("dashboard command should succeed");
    assert_eq!(dashboard.priority.intent_id, "comparison");

    let evidence = get_intent_evidence_data("comparison".to_owned()).expect("evidence command");
    assert_eq!(evidence.evidence_rows.len(), 6);

    let missing_intent_error = get_intent_evidence_data("not-a-fixture-intent".to_owned())
        .expect_err("unknown intent should fail");
    let serialized_error = serde_json::to_value(missing_intent_error).expect("serializable error");
    assert_eq!(serialized_error["code"], "IntentNotFound");
    assert_eq!(
        serialized_error["message"],
        "That intent is not available in this fixture."
    );
}
