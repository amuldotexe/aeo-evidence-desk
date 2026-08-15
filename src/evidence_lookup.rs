use serde::{Deserialize, Serialize};

use crate::{evaluate_answer_alias_match, AppError, FixtureBundle, MentionOutcome};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IntentEvidenceRow {
    pub observation_id: String,
    pub intent_id: String,
    pub prompt_id: String,
    pub prompt_text: String,
    pub provider_id: String,
    pub provider_label: String,
    pub answer_text: String,
    pub source_type: String,
    pub mention: MentionOutcome,
}

pub fn find_intent_evidence_rows(
    fixture_bundle: &FixtureBundle,
    intent_id: &str,
) -> Result<Vec<IntentEvidenceRow>, AppError> {
    if !fixture_bundle
        .intents
        .iter()
        .any(|intent| intent.id == intent_id)
    {
        return Err(AppError::IntentNotFound {
            intent_id: intent_id.to_owned(),
        });
    }

    let selected_prompts: Vec<_> = fixture_bundle
        .prompts
        .iter()
        .filter(|prompt| prompt.intent_id == intent_id)
        .collect();
    let mut evidence_rows = Vec::new();

    for prompt in selected_prompts {
        for observation in fixture_bundle
            .observations
            .iter()
            .filter(|observation| observation.prompt_id == prompt.id)
        {
            let provider = fixture_bundle
                .providers
                .iter()
                .find(|provider| provider.id == observation.provider_id)
                .ok_or_else(|| create_invalid_evidence_error("Observation provider is missing."))?;
            evidence_rows.push(IntentEvidenceRow {
                observation_id: observation.id.clone(),
                intent_id: intent_id.to_owned(),
                prompt_id: prompt.id.clone(),
                prompt_text: prompt.prompt_text.clone(),
                provider_id: provider.id.clone(),
                provider_label: provider.label.clone(),
                answer_text: observation.answer_text.clone(),
                source_type: observation.source_type.clone(),
                mention: evaluate_answer_alias_match(
                    &observation.answer_text,
                    &fixture_bundle.brand_aliases,
                ),
            });
        }
    }

    Ok(evidence_rows)
}

fn create_invalid_evidence_error(message: &str) -> AppError {
    AppError::InvalidFixture {
        message: message.to_owned(),
    }
}
