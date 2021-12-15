use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::field_attributes::{
    deserialize_datetime_utc_from_milliseconds, deserialize_number_from_string,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::v3::types::{price::Price, purchases_kind::PurchasesKind};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPurchase {
    pub kind: PurchasesKind,

    #[serde(
        rename(deserialize = "startTimeMillis"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub start_time: DateTime<Utc>,

    #[serde(
        rename(deserialize = "expiryTimeMillis"),
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    pub expiry_time: DateTime<Utc>,

    #[serde(
        rename(deserialize = "autoResumeTimeMillis"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub auto_resume_time: Option<DateTime<Utc>>,

    pub auto_renewing: bool,

    pub price_currency_code: String,

    #[serde(
        rename(deserialize = "priceAmountMicros"),
        deserialize_with = "deserialize_number_from_string"
    )]
    pub price_amount_micros: usize,

    pub introductory_price_info: Option<IntroductoryPriceInfo>,

    pub country_code: String,

    pub developer_payload: String,

    pub payment_state: Option<PaymentState>,

    pub cancel_reason: Option<CancelReason>,

    #[serde(
        rename(deserialize = "userCancellationTimeMillis"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub user_cancellation_time: Option<DateTime<Utc>>,

    pub cancel_survey_result: Option<SubscriptionCancelSurveyResult>,

    pub order_id: String,

    pub linked_purchase_token: Option<String>,

    pub purchase_type: Option<PurchaseType>,

    pub price_change: Option<SubscriptionPriceChange>,

    pub profile_name: Option<String>,
    pub email_address: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub profile_id: Option<String>,

    pub acknowledgement_state: AcknowledgementState,

    pub external_account_id: Option<String>,

    pub promotion_type: Option<PromotionType>,
    pub promotion_code: Option<String>,

    pub obfuscated_external_account_id: Option<String>,
    pub obfuscated_external_profile_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntroductoryPriceInfo {
    pub introductory_price_currency_code: String,

    #[serde(
        rename(deserialize = "introductoryPriceAmountMicros"),
        deserialize_with = "deserialize_number_from_string"
    )]
    pub introductory_price_amount_micros: usize,

    pub introductory_price_period: String,

    pub introductory_price_cycles: usize,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PaymentState {
    PaymentPending = 0,
    PaymentReceived = 1,
    FreeTrial = 2,
    PendingDeferredUpgradeOrDowngrade = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CancelReason {
    UserCanceledTheSubscription = 0,
    SubscriptionWasCanceledByTheSystem = 1,
    SubscriptionWasCeplacedWithANewSubscription = 2,
    SubscriptionWasCanceledByTheDeveloper = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCancelSurveyResult {
    pub cancel_survey_reason: CancelSurveyReason,

    pub user_input_cancel_reason: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CancelSurveyReason {
    Other = 0,
    IDoesNotUseThisServiceEnough = 1,
    TechnicalIssues = 2,
    CostRelatedReasons = 3,
    IFoundABetterApp = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PurchaseType {
    Test = 0,
    Promo = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPriceChange {
    pub new_price: Price,

    pub state: SubscriptionPriceChangeState,
}
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SubscriptionPriceChangeState {
    Outstanding = 0,
    Accepted = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AcknowledgementState {
    YetToBeAcknowledged = 0,
    Acknowledged = 1,
}
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PromotionType {
    OneTimeCode = 0,
    VanityCode = 1,
}

//
//
//
fn deserialize_datetime_utc_from_milliseconds_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_datetime_utc_from_milliseconds(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_renewing_is_true() {
        let json = r#"
        {
            "kind": "androidpublisher#subscriptionPurchase",
            "startTimeMillis": "1565173322987",
            "expiryTimeMillis": "1565173847452",
            "autoRenewing": true,
            "priceCurrencyCode": "USD",
            "priceAmountMicros": "6490000",
            "countryCode": "US",
            "developerPayload": "",
            "paymentState": 1,
            "orderId": "GPA.0000-9829-1189-24947",
            "acknowledgementState": 1
        }
        "#;

        let r: SubscriptionPurchase = serde_json::from_str(json).unwrap();

        assert_eq!(r.kind, PurchasesKind::SubscriptionPurchase);
        assert_eq!(r.start_time.timestamp_millis(), 1565173322987);
        assert_eq!(r.expiry_time.timestamp_millis(), 1565173847452);
        assert!(r.auto_renewing);
        assert_eq!(r.price_currency_code, "USD");
        assert_eq!(r.price_amount_micros, 6490000);
        assert_eq!(r.country_code, "US");
        assert_eq!(r.developer_payload, "");
        assert_eq!(r.payment_state, Some(PaymentState::PaymentReceived));
        assert_eq!(r.order_id, "GPA.0000-9829-1189-24947");
        assert_eq!(r.acknowledgement_state, AcknowledgementState::Acknowledged);
    }

    #[test]
    fn auto_renewing_is_false() {
        let json = r#"
        {
            "kind": "androidpublisher#subscriptionPurchase",
            "startTimeMillis": "1565163156123",
            "expiryTimeMillis": "1565165367979",
            "autoRenewing": false,
            "priceCurrencyCode": "USD",
            "priceAmountMicros": "6490000",
            "countryCode": "US",
            "developerPayload": "",
            "cancelReason": 1,
            "orderId": "GPA.0000-2151-5181-25913..5",
            "acknowledgementState": 1
        }
        "#;

        let r: SubscriptionPurchase = serde_json::from_str(json).unwrap();

        assert_eq!(r.kind, PurchasesKind::SubscriptionPurchase);
        assert_eq!(r.start_time.timestamp_millis(), 1565163156123);
        assert_eq!(r.expiry_time.timestamp_millis(), 1565165367979);
        assert!(!r.auto_renewing);
        assert_eq!(r.price_currency_code, "USD");
        assert_eq!(r.price_amount_micros, 6490000);
        assert_eq!(r.country_code, "US");
        assert_eq!(r.developer_payload, "");
        assert_eq!(
            r.cancel_reason,
            Some(CancelReason::SubscriptionWasCanceledByTheSystem)
        );
        assert_eq!(r.order_id, "GPA.0000-2151-5181-25913..5");
        assert_eq!(r.acknowledgement_state, AcknowledgementState::Acknowledged);
    }
}
