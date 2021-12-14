//! https://cloud.google.com/vision/docs/reference/rest/Shared.Types/LatLng

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LatLng {
    pub latitude: isize,
    pub longitude: isize,
}
