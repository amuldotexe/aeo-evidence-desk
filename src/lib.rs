//! Deterministic, fixture-backed core for the AEO Fixture Evidence Desk.

mod app_error;
mod evidence_lookup;
mod fixture_data;
mod presence_metrics;
mod presentation_data;
mod priority_action;

pub use app_error::AppError;
pub use evidence_lookup::{find_intent_evidence_rows, IntentEvidenceRow};
pub use fixture_data::{
    load_embedded_fixture_bundle, parse_fixture_bundle_text, validate_fixture_bundle_shape,
    BrandAlias, ExpectedScore, FixtureBundle, IntentCohort, PromptCase, ProviderLabel,
    ResponseObservation, Vertical,
};
pub use presence_metrics::{
    calculate_intent_presence_rates, evaluate_answer_alias_match, IntentPresenceRate,
    MentionOutcome, ProviderPresenceCell,
};
pub use presentation_data::{
    build_fixture_dashboard_data, build_intent_evidence_data, FixtureDashboardData,
    IntentEvidenceData, FIXTURE_NOTICE_TEXT, SCORING_RULE_TEXT,
};
pub use priority_action::{select_priority_intent_gap, PriorityIntentGap};
