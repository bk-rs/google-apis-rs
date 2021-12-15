//! https://developers.google.com/android-publisher/api-ref/rest/v3/Price

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub price_micros: String,
    pub currency: String,
}
