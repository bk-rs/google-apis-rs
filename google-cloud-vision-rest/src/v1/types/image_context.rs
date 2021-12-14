//! https://cloud.google.com/vision/docs/reference/rest/v1/ImageContext

use serde::{Deserialize, Serialize};

use crate::{
    shared_types::LatLng, v1::resources::projects_locations_products_reference_images::BoundingPoly,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_long_rect: Option<LatLongRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_hints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_hints_params: Option<CropHintsParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_search_params: Option<ProductSearchParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_detection_params: Option<WebDetectionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detection_params: Option<TextDetectionParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LatLongRect {
    pub min_lat_lng: LatLng,
    pub max_lat_lng: LatLng,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CropHintsParams {
    pub aspect_ratios: Vec<isize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductSearchParams {
    pub bounding_poly: BoundingPoly,
    pub product_set: String,
    pub product_categories: Vec<String>,
    pub filter: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebDetectionParams {
    pub include_geo_results: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextDetectionParams {
    pub enable_text_detection_confidence_score: bool,
}
