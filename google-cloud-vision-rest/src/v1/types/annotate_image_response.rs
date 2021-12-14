//! https://cloud.google.com/vision/docs/reference/rest/v1/AnnotateImageResponse

use serde::{Deserialize, Serialize};

use crate::{
    shared_types::LatLng, v1::resources::projects_locations_products_reference_images::BoundingPoly,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnotateImageResponse {
    // TODO
    pub label_annotations: Option<Vec<EntityAnnotation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntityAnnotation {
    pub mid: String,
    pub locale: Option<String>,
    pub description: String,
    pub score: f64,
    pub topicality: f64,
    pub bounding_poly: Option<BoundingPoly>,
    pub locations: Option<Vec<LocationInfo>>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationInfo {
    pub lat_lng: LatLng,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub value: Option<String>,
    pub uint64_value: Option<usize>,
}
