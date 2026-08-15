use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

/// Error shape shared by the Rust core and the desktop command boundary.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum AppError {
    #[error("The fixture data is invalid: {message}")]
    InvalidFixture { message: String },
    #[error("The requested intent `{intent_id}` does not exist in this fixture.")]
    IntentNotFound { intent_id: String },
}

impl AppError {
    pub fn get_serializable_error_code(&self) -> &'static str {
        match self {
            Self::InvalidFixture { .. } => "InvalidFixture",
            Self::IntentNotFound { .. } => "IntentNotFound",
        }
    }

    pub fn get_user_readable_message(&self) -> &'static str {
        match self {
            Self::InvalidFixture { .. } => "Fixture data could not be loaded.",
            Self::IntentNotFound { .. } => "That intent is not available in this fixture.",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("code", self.get_serializable_error_code())?;
        state.serialize_field("message", self.get_user_readable_message())?;
        state.end()
    }
}
