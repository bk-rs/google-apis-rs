use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_datetime_utc_from_milliseconds;
use serde_repr::Deserialize_repr;

use crate::v3::types::purchases_kind::PurchasesKind;

#[derive(Deserialize, Debug)]
pub struct ProductPurchase {
    pub kind: PurchasesKind,

    #[serde(
        rename(deserialize = "purchaseTimeMillis"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub purchase_time: DateTime<Utc>,

    #[serde(rename(deserialize = "purchaseState"))]
    pub purchase_state: PurchaseState,

    #[serde(rename(deserialize = "consumptionState"))]
    pub consumption_state: ConsumptionState,

    #[serde(rename(deserialize = "developerPayload"))]
    pub developer_payload: String,

    #[serde(rename(deserialize = "orderId"))]
    pub order_id: String,

    #[serde(rename(deserialize = "purchaseType"))]
    pub purchase_type: PurchaseType,

    #[serde(rename(deserialize = "acknowledgementState"))]
    pub acknowledgement_state: AcknowledgementState,

    #[serde(rename(deserialize = "purchaseToken"))]
    pub purchase_token: String,

    #[serde(rename(deserialize = "productId"))]
    pub product_id: String,

    pub quantity: usize,

    #[serde(rename(deserialize = "obfuscatedExternalAccountId"))]
    pub obfuscated_external_account_id: Option<String>,
    #[serde(rename(deserialize = "obfuscatedExternalProfileId"))]
    pub obfuscated_external_profile_id: Option<String>,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum PurchaseState {
    Purchased = 0,
    Canceled = 1,
    Pending = 2,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum ConsumptionState {
    YetToBeConsumed = 0,
    Consumed = 1,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum PurchaseType {
    Test = 0,
    Promo = 1,
    Rewarded = 2,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum AcknowledgementState {
    YetToBeAcknowledged = 0,
    Acknowledged = 1,
}
