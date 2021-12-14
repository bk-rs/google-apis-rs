//! https://cloud.google.com/vision/docs/reference/rest/v1/BatchAnnotateImagesResponse

use serde::{Deserialize, Serialize};

use super::AnnotateImageResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchAnnotateImagesResponse {
    pub responses: Vec<AnnotateImageResponse>,
}
