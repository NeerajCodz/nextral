use crate::contracts::CoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageError {
    pub code: String,
    pub message: String,
}

impl From<CoreError> for PackageError {
    fn from(error: CoreError) -> Self {
        let code = match &error {
            CoreError::InvalidInput(_) => "invalid_input",
            CoreError::NotFound(_) => "not_found",
            CoreError::Conflict(_) => "conflict",
            CoreError::Io(_) => "io",
            CoreError::Serialization(_) => "serialization",
        };
        Self {
            code: code.to_string(),
            message: error.to_string(),
        }
    }
}
