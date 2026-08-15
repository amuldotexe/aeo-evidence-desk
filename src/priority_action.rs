use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{AppError, IntentCohort, IntentPresenceRate};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PriorityIntentGap {
    pub intent_id: String,
    pub intent_label: String,
    pub mentioned: usize,
    pub total: usize,
    pub action_template: String,
    pub boundary_label: String,
}

pub fn select_priority_intent_gap(
    intent_rates: &[IntentPresenceRate],
    intent_cohorts: &[IntentCohort],
) -> Result<PriorityIntentGap, AppError> {
    let priority_rate = intent_rates
        .iter()
        .min_by(|left_rate, right_rate| compare_presence_rate_values(left_rate, right_rate))
        .ok_or_else(|| create_invalid_priority_error("No intent rates are available."))?;
    let priority_cohort = intent_cohorts
        .iter()
        .find(|intent| intent.id == priority_rate.intent_id)
        .ok_or_else(|| {
            create_invalid_priority_error("The priority rate has no matching intent.")
        })?;

    Ok(PriorityIntentGap {
        intent_id: priority_rate.intent_id.clone(),
        intent_label: priority_rate.intent_label.clone(),
        mentioned: priority_rate.mentioned,
        total: priority_rate.total,
        action_template: priority_cohort.action_template.clone(),
        boundary_label: "This is a content-investigation hypothesis, not proof of causal lift."
            .to_owned(),
    })
}

fn compare_presence_rate_values(
    left_rate: &IntentPresenceRate,
    right_rate: &IntentPresenceRate,
) -> Ordering {
    let rate_ordering = ((left_rate.mentioned as u128) * (right_rate.total as u128))
        .cmp(&((right_rate.mentioned as u128) * (left_rate.total as u128)));

    rate_ordering.then_with(|| left_rate.intent_id.cmp(&right_rate.intent_id))
}

fn create_invalid_priority_error(message: &str) -> AppError {
    AppError::InvalidFixture {
        message: message.to_owned(),
    }
}
