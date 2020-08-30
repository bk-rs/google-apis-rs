use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::{
    deserialize_datetime_utc_from_milliseconds, deserialize_number_from_string,
};
use serde_repr::Deserialize_repr;

use crate::v3::types::{price::Price, purchases_kind::PurchasesKind};

#[derive(Deserialize, Debug)]
pub struct ProductPurchase {
    pub kind: PurchasesKind,
}
