//! https://cloud.google.com/vision/docs/reference/rest/v1/Feature

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub r#type: Type,
    pub max_results: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    TYPE_UNSPECIFIED,
    LABEL_DETECTION,
    // TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    #[serde(rename = "builtin/stable")]
    Stable,
    #[serde(rename = "builtin/latest")]
    Latest,
}
impl Default for Model {
    fn default() -> Self {
        Self::Stable
    }
}
