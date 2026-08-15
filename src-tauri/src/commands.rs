//! Thin Tauri command boundary for the fixture-backed core.

use aeo_fixture_desk_core::{
    build_fixture_dashboard_data, build_intent_evidence_data, AppError, FixtureDashboardData,
    IntentEvidenceData,
};

#[tauri::command]
pub fn get_fixture_dashboard_data() -> Result<FixtureDashboardData, AppError> {
    build_fixture_dashboard_data()
}

#[tauri::command]
pub fn get_intent_evidence_data(intent_id: String) -> Result<IntentEvidenceData, AppError> {
    build_intent_evidence_data(&intent_id)
}
