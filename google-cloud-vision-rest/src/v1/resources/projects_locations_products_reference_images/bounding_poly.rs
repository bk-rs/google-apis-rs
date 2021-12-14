use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoundingPoly {
    pub vertices: Vec<Vertex>,
    pub normalized_vertices: Vec<NormalizedVertex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vertex {
    pub x: isize,
    pub y: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedVertex {
    pub x: f64,
    pub y: f64,
}
