//! https://cloud.google.com/vision/docs/reference/rest/v1/AnnotateImageRequest

use serde::{Deserialize, Serialize};

use super::{Feature, ImageContext};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnotateImageRequest {
    pub image: Image,
    pub features: Vec<Feature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_context: Option<ImageContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageSource>,
}
impl Image {
    pub fn with_bytes(binary_data: Vec<u8>) -> Self {
        Self {
            content: Some(base64::encode(binary_data)),
            source: None,
        }
    }
    pub fn with_url(url: String) -> Self {
        Self {
            content: None,
            source: Some(ImageSource {
                gcs_image_uri: None,
                image_uri: Some(url),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcs_image_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
}
