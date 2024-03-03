use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    /// Errors
    pub errors: HashMap<String, Vec<String>>,
}
