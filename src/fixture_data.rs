use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::AppError;

const EMBEDDED_FIXTURE_TEXT: &str = include_str!("../fixtures/beauty_loreal_fixture.json");

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FixtureBundle {
    pub scope_label: String,
    pub market: String,
    pub vertical: Vertical,
    pub intents: Vec<IntentCohort>,
    pub prompts: Vec<PromptCase>,
    pub providers: Vec<ProviderLabel>,
    pub brand_aliases: Vec<BrandAlias>,
    pub observations: Vec<ResponseObservation>,
    pub expected_scores: Vec<ExpectedScore>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Vertical {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct IntentCohort {
    pub id: String,
    pub label: String,
    pub intent_type: String,
    pub action_template: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PromptCase {
    pub id: String,
    pub intent_id: String,
    pub prompt_text: String,
    pub market: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProviderLabel {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BrandAlias {
    pub id: String,
    pub alias_text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResponseObservation {
    pub id: String,
    pub prompt_id: String,
    pub provider_id: String,
    pub answer_text: String,
    pub source_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExpectedScore {
    pub observation_id: String,
    pub brand_mentioned: bool,
}

pub fn load_embedded_fixture_bundle() -> Result<FixtureBundle, AppError> {
    parse_fixture_bundle_text(EMBEDDED_FIXTURE_TEXT)
}

pub fn parse_fixture_bundle_text(raw_fixture_text: &str) -> Result<FixtureBundle, AppError> {
    let fixture_bundle =
        serde_json::from_str(raw_fixture_text).map_err(|_| AppError::InvalidFixture {
            message: "The fixture is not valid JSON.".to_owned(),
        })?;

    validate_fixture_bundle_shape(&fixture_bundle)?;
    Ok(fixture_bundle)
}

pub fn validate_fixture_bundle_shape(fixture_bundle: &FixtureBundle) -> Result<(), AppError> {
    validate_fixture_entity_count("intent cohorts", fixture_bundle.intents.len(), 3)?;
    validate_fixture_entity_count("prompt cases", fixture_bundle.prompts.len(), 6)?;
    validate_fixture_entity_count("provider labels", fixture_bundle.providers.len(), 3)?;
    validate_fixture_entity_count("brand aliases", fixture_bundle.brand_aliases.len(), 4)?;
    validate_fixture_entity_count(
        "response observations",
        fixture_bundle.observations.len(),
        18,
    )?;
    validate_fixture_entity_count("expected scores", fixture_bundle.expected_scores.len(), 18)?;

    if fixture_bundle.market != "en-US" {
        return Err(create_invalid_fixture_error(
            "The fixture market must be en-US.",
        ));
    }

    ensure_unique_entity_identifiers(
        fixture_bundle
            .intents
            .iter()
            .map(|intent| intent.id.as_str()),
        "intent cohort",
    )?;
    ensure_unique_entity_identifiers(
        fixture_bundle
            .prompts
            .iter()
            .map(|prompt| prompt.id.as_str()),
        "prompt case",
    )?;
    ensure_unique_entity_identifiers(
        fixture_bundle
            .providers
            .iter()
            .map(|provider| provider.id.as_str()),
        "provider label",
    )?;
    ensure_unique_entity_identifiers(
        fixture_bundle
            .brand_aliases
            .iter()
            .map(|brand_alias| brand_alias.id.as_str()),
        "brand alias",
    )?;
    ensure_unique_entity_identifiers(
        fixture_bundle
            .observations
            .iter()
            .map(|observation| observation.id.as_str()),
        "response observation",
    )?;
    ensure_unique_entity_identifiers(
        fixture_bundle
            .expected_scores
            .iter()
            .map(|score| score.observation_id.as_str()),
        "expected score observation",
    )?;

    validate_prompt_reference_links(fixture_bundle)?;
    validate_observation_reference_links(fixture_bundle)?;
    validate_score_reference_links(fixture_bundle)?;
    Ok(())
}

fn validate_fixture_entity_count(
    entity_name: &str,
    actual_count: usize,
    expected_count: usize,
) -> Result<(), AppError> {
    if actual_count == expected_count {
        return Ok(());
    }

    Err(create_invalid_fixture_error(&format!(
        "Expected {expected_count} {entity_name}, found {actual_count}."
    )))
}

fn ensure_unique_entity_identifiers<'a>(
    identifiers: impl Iterator<Item = &'a str>,
    entity_name: &str,
) -> Result<(), AppError> {
    let mut seen_identifiers = HashSet::new();
    for identifier in identifiers {
        if !seen_identifiers.insert(identifier) {
            return Err(create_invalid_fixture_error(&format!(
                "Duplicate {entity_name} identifier `{identifier}`."
            )));
        }
    }
    Ok(())
}

fn validate_prompt_reference_links(fixture_bundle: &FixtureBundle) -> Result<(), AppError> {
    let intent_identifiers: HashSet<&str> = fixture_bundle
        .intents
        .iter()
        .map(|intent| intent.id.as_str())
        .collect();

    for prompt in &fixture_bundle.prompts {
        if !intent_identifiers.contains(prompt.intent_id.as_str()) {
            return Err(create_invalid_fixture_error(&format!(
                "Prompt `{}` references an unknown intent.",
                prompt.id
            )));
        }
        if prompt.market != fixture_bundle.market {
            return Err(create_invalid_fixture_error(&format!(
                "Prompt `{}` uses a different market.",
                prompt.id
            )));
        }
    }
    Ok(())
}

fn validate_observation_reference_links(fixture_bundle: &FixtureBundle) -> Result<(), AppError> {
    let prompt_identifiers: HashSet<&str> = fixture_bundle
        .prompts
        .iter()
        .map(|prompt| prompt.id.as_str())
        .collect();
    let provider_identifiers: HashSet<&str> = fixture_bundle
        .providers
        .iter()
        .map(|provider| provider.id.as_str())
        .collect();
    let mut observation_pairs = HashSet::new();

    for observation in &fixture_bundle.observations {
        if !prompt_identifiers.contains(observation.prompt_id.as_str()) {
            return Err(create_invalid_fixture_error(&format!(
                "Observation `{}` references an unknown prompt.",
                observation.id
            )));
        }
        if !provider_identifiers.contains(observation.provider_id.as_str()) {
            return Err(create_invalid_fixture_error(&format!(
                "Observation `{}` references an unknown provider.",
                observation.id
            )));
        }
        if observation.source_type != "authored_demo_fixture" {
            return Err(create_invalid_fixture_error(&format!(
                "Observation `{}` has an unsupported source type.",
                observation.id
            )));
        }
        if !observation_pairs.insert((&observation.prompt_id, &observation.provider_id)) {
            return Err(create_invalid_fixture_error(&format!(
                "Observation `{}` duplicates a prompt/provider pair.",
                observation.id
            )));
        }
    }
    Ok(())
}

fn validate_score_reference_links(fixture_bundle: &FixtureBundle) -> Result<(), AppError> {
    let observation_identifiers: HashSet<&str> = fixture_bundle
        .observations
        .iter()
        .map(|observation| observation.id.as_str())
        .collect();

    for expected_score in &fixture_bundle.expected_scores {
        if !observation_identifiers.contains(expected_score.observation_id.as_str()) {
            return Err(create_invalid_fixture_error(&format!(
                "Expected score references unknown observation `{}`.",
                expected_score.observation_id
            )));
        }
    }
    Ok(())
}

fn create_invalid_fixture_error(message: &str) -> AppError {
    AppError::InvalidFixture {
        message: message.to_owned(),
    }
}
