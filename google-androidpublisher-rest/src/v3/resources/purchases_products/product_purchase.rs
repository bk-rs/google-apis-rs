use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_datetime_utc_from_milliseconds;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::v3::types::purchases_kind::PurchasesKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductPurchase {
    pub kind: PurchasesKind,

    #[serde(
        rename(deserialize = "purchaseTimeMillis"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub purchase_time: DateTime<Utc>,

    pub purchase_state: PurchaseState,

    pub consumption_state: ConsumptionState,

    pub developer_payload: String,

    pub order_id: String,

    pub purchase_type: PurchaseType,

    pub acknowledgement_state: AcknowledgementState,

    pub purchase_token: String,

    pub product_id: String,

    pub quantity: usize,

    pub obfuscated_external_account_id: Option<String>,
    pub obfuscated_external_profile_id: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PurchaseState {
    Purchased = 0,
    Canceled = 1,
    Pending = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsumptionState {
    YetToBeConsumed = 0,
    Consumed = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PurchaseType {
    Test = 0,
    Promo = 1,
    Rewarded = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AcknowledgementState {
    YetToBeAcknowledged = 0,
    Acknowledged = 1,
}
