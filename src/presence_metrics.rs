use serde::{Deserialize, Serialize};

use crate::{BrandAlias, FixtureBundle};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MentionOutcome {
    pub brand_mentioned: bool,
    pub matched_alias: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProviderPresenceCell {
    pub provider_id: String,
    pub provider_label: String,
    pub mentioned: usize,
    pub total: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IntentPresenceRate {
    pub intent_id: String,
    pub intent_label: String,
    pub mentioned: usize,
    pub total: usize,
    pub provider_cells: Vec<ProviderPresenceCell>,
}

pub fn evaluate_answer_alias_match(
    answer_text: &str,
    brand_aliases: &[BrandAlias],
) -> MentionOutcome {
    for brand_alias in brand_aliases {
        if find_alias_boundary_match(answer_text, &brand_alias.alias_text) {
            return MentionOutcome {
                brand_mentioned: true,
                matched_alias: Some(brand_alias.alias_text.clone()),
            };
        }
    }

    MentionOutcome {
        brand_mentioned: false,
        matched_alias: None,
    }
}

pub fn calculate_intent_presence_rates(fixture_bundle: &FixtureBundle) -> Vec<IntentPresenceRate> {
    fixture_bundle
        .intents
        .iter()
        .map(|intent| {
            let intent_prompt_identifiers: Vec<&str> = fixture_bundle
                .prompts
                .iter()
                .filter(|prompt| prompt.intent_id == intent.id)
                .map(|prompt| prompt.id.as_str())
                .collect();
            let provider_cells = fixture_bundle
                .providers
                .iter()
                .map(|provider| {
                    let provider_observations: Vec<_> = fixture_bundle
                        .observations
                        .iter()
                        .filter(|observation| {
                            observation.provider_id == provider.id
                                && intent_prompt_identifiers
                                    .contains(&observation.prompt_id.as_str())
                        })
                        .collect();
                    let mentioned = provider_observations
                        .iter()
                        .filter(|observation| {
                            evaluate_answer_alias_match(
                                &observation.answer_text,
                                &fixture_bundle.brand_aliases,
                            )
                            .brand_mentioned
                        })
                        .count();

                    ProviderPresenceCell {
                        provider_id: provider.id.clone(),
                        provider_label: provider.label.clone(),
                        mentioned,
                        total: provider_observations.len(),
                    }
                })
                .collect::<Vec<_>>();
            let mentioned = provider_cells.iter().map(|cell| cell.mentioned).sum();
            let total = provider_cells.iter().map(|cell| cell.total).sum();

            IntentPresenceRate {
                intent_id: intent.id.clone(),
                intent_label: intent.label.clone(),
                mentioned,
                total,
                provider_cells,
            }
        })
        .collect()
}

fn find_alias_boundary_match(answer_text: &str, alias_text: &str) -> bool {
    let normalized_answer = answer_text.to_lowercase();
    let normalized_alias = alias_text.to_lowercase();

    if normalized_alias.is_empty() {
        return false;
    }

    normalized_answer
        .match_indices(&normalized_alias)
        .any(|(match_start, _)| {
            let match_end = match_start + normalized_alias.len();
            let leading_character = normalized_answer[..match_start].chars().next_back();
            let trailing_character = normalized_answer[match_end..].chars().next();
            let has_left_boundary =
                leading_character.map_or(true, |character| !character.is_alphanumeric());
            let has_right_boundary =
                trailing_character.map_or(true, |character| !character.is_alphanumeric());

            has_left_boundary && has_right_boundary
        })
}
