use serde::{Deserialize, Serialize};

use crate::{
    calculate_intent_presence_rates, find_intent_evidence_rows, load_embedded_fixture_bundle,
    select_priority_intent_gap, AppError, FixtureBundle, IntentEvidenceRow, IntentPresenceRate,
    PriorityIntentGap, ProviderLabel,
};

pub const FIXTURE_NOTICE_TEXT: &str =
    "Fixture analysis — authored demonstration data; not a live provider measurement.";
pub const SCORING_RULE_TEXT: &str =
    "An answer is counted when it contains a configured alias using case-insensitive whole-word matching.";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FixtureDashboardData {
    pub fixture_notice: String,
    pub scope_label: String,
    pub scoring_rule: String,
    pub providers: Vec<ProviderLabel>,
    pub intent_rates: Vec<IntentPresenceRate>,
    pub priority: PriorityIntentGap,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IntentEvidenceData {
    pub fixture_notice: String,
    pub scope_label: String,
    pub scoring_rule: String,
    pub intent: IntentPresenceRate,
    pub priority: PriorityIntentGap,
    pub evidence_rows: Vec<IntentEvidenceRow>,
}

pub fn build_fixture_dashboard_data() -> Result<FixtureDashboardData, AppError> {
    let fixture_bundle = load_embedded_fixture_bundle()?;
    let decision_data = calculate_fixture_decision_data(&fixture_bundle)?;

    Ok(FixtureDashboardData {
        fixture_notice: FIXTURE_NOTICE_TEXT.to_owned(),
        scope_label: fixture_bundle.scope_label,
        scoring_rule: SCORING_RULE_TEXT.to_owned(),
        providers: fixture_bundle.providers,
        intent_rates: decision_data.intent_rates,
        priority: decision_data.priority,
    })
}

pub fn build_intent_evidence_data(intent_id: &str) -> Result<IntentEvidenceData, AppError> {
    let fixture_bundle = load_embedded_fixture_bundle()?;
    let decision_data = calculate_fixture_decision_data(&fixture_bundle)?;
    let evidence_rows = find_intent_evidence_rows(&fixture_bundle, intent_id)?;
    let intent = decision_data
        .intent_rates
        .iter()
        .find(|intent_rate| intent_rate.intent_id == intent_id)
        .cloned()
        .ok_or_else(|| create_missing_rate_error(intent_id))?;

    Ok(IntentEvidenceData {
        fixture_notice: FIXTURE_NOTICE_TEXT.to_owned(),
        scope_label: fixture_bundle.scope_label,
        scoring_rule: SCORING_RULE_TEXT.to_owned(),
        intent,
        priority: decision_data.priority,
        evidence_rows,
    })
}

struct FixtureDecisionData {
    intent_rates: Vec<IntentPresenceRate>,
    priority: PriorityIntentGap,
}

fn calculate_fixture_decision_data(
    fixture_bundle: &FixtureBundle,
) -> Result<FixtureDecisionData, AppError> {
    let intent_rates = calculate_intent_presence_rates(fixture_bundle);
    let priority = select_priority_intent_gap(&intent_rates, &fixture_bundle.intents)?;

    Ok(FixtureDecisionData {
        intent_rates,
        priority,
    })
}

fn create_missing_rate_error(intent_id: &str) -> AppError {
    AppError::InvalidFixture {
        message: format!("The fixture has no calculated rate for `{intent_id}`."),
    }
}
