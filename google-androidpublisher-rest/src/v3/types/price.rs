use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Price {
    #[serde(rename(deserialize = "priceMicros"))]
    pub price_micros: String,

    #[serde(rename(deserialize = "currency"))]
    pub currency: String,
}
