use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::{
    deserialize_datetime_utc_from_milliseconds, deserialize_number_from_string,
};
use serde_repr::Deserialize_repr;

use crate::v3::types::{price::Price, purchases_kind::PurchasesKind};

#[derive(Deserialize, Debug)]
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

    #[serde(rename(deserialize = "autoRenewing"))]
    pub auto_renewing: bool,

    #[serde(rename(deserialize = "priceCurrencyCode"))]
    pub price_currency_code: String,

    #[serde(
        rename(deserialize = "priceAmountMicros"),
        deserialize_with = "deserialize_number_from_string"
    )]
    pub price_amount_micros: usize,

    #[serde(rename(deserialize = "introductoryPriceInfo"))]
    pub introductory_price_info: Option<IntroductoryPriceInfo>,

    #[serde(rename(deserialize = "countryCode"))]
    pub country_code: String,

    #[serde(rename(deserialize = "developerPayload"))]
    pub developer_payload: String,

    #[serde(rename(deserialize = "paymentState"))]
    pub payment_state: Option<PaymentState>,

    #[serde(rename(deserialize = "cancelReason"))]
    pub cancel_reason: Option<CancelReason>,

    #[serde(
        rename(deserialize = "userCancellationTimeMillis"),
        default,
        deserialize_with = "deserialize_datetime_utc_from_milliseconds_option"
    )]
    pub user_cancellation_time: Option<DateTime<Utc>>,

    #[serde(rename(deserialize = "cancelSurveyResult"))]
    pub cancel_survey_result: Option<SubscriptionCancelSurveyResult>,

    #[serde(rename(deserialize = "orderId"))]
    pub order_id: String,

    #[serde(rename(deserialize = "linkedPurchaseToken"))]
    pub linked_purchase_token: Option<String>,

    #[serde(rename(deserialize = "purchaseType"))]
    pub purchase_type: Option<PurchaseType>,

    #[serde(rename(deserialize = "priceChange"))]
    pub price_change: Option<SubscriptionPriceChange>,

    #[serde(rename(deserialize = "profileName"))]
    pub profile_name: Option<String>,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email_address: Option<String>,
    #[serde(rename(deserialize = "givenName"))]
    pub given_name: Option<String>,
    #[serde(rename(deserialize = "familyName"))]
    pub family_name: Option<String>,
    #[serde(rename(deserialize = "profileId"))]
    pub profile_id: Option<String>,

    #[serde(rename(deserialize = "acknowledgementState"))]
    pub acknowledgement_state: AcknowledgementState,

    #[serde(rename(deserialize = "externalAccountId"))]
    pub external_account_id: Option<String>,

    #[serde(rename(deserialize = "promotionType"))]
    pub promotion_type: Option<PromotionType>,
    #[serde(rename(deserialize = "promotionCode"))]
    pub promotion_code: Option<String>,

    #[serde(rename(deserialize = "obfuscatedExternalAccountId"))]
    pub obfuscated_external_account_id: Option<String>,
    #[serde(rename(deserialize = "obfuscatedExternalProfileId"))]
    pub obfuscated_external_profile_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct IntroductoryPriceInfo {
    #[serde(rename(deserialize = "introductoryPriceCurrencyCode"))]
    pub introductory_price_currency_code: String,

    #[serde(
        rename(deserialize = "introductoryPriceAmountMicros"),
        deserialize_with = "deserialize_number_from_string"
    )]
    pub introductory_price_amount_micros: usize,

    #[serde(rename(deserialize = "introductoryPricePeriod"))]
    pub introductory_price_period: String,

    #[serde(rename(deserialize = "introductoryPriceCycles"))]
    pub introductory_price_cycles: usize,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum PaymentState {
    PaymentPending = 0,
    PaymentReceived = 2,
    FreeTrial = 3,
    PendingDeferredUpgradeOrDowngrade = 4,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum CancelReason {
    UserCanceledTheSubscription = 0,
    SubscriptionWasCanceledByTheSystem = 1,
    SubscriptionWasCeplacedWithANewSubscription = 2,
    SubscriptionWasCanceledByTheDeveloper = 3,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionCancelSurveyResult {
    #[serde(rename(deserialize = "cancelSurveyReason"))]
    pub cancel_survey_reason: CancelSurveyReason,

    #[serde(rename(deserialize = "userInputCancelReason"))]
    pub user_input_cancel_reason: Option<String>,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum CancelSurveyReason {
    Other = 0,
    IDoesNotUseThisServiceEnough = 1,
    TechnicalIssues = 2,
    CostRelatedReasons = 3,
    IFoundABetterApp = 4,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum PurchaseType {
    Test = 0,
    Promo = 1,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionPriceChange {
    #[serde(rename(deserialize = "newPrice"))]
    pub new_price: Price,

    pub state: SubscriptionPriceChangeState,
}
#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum SubscriptionPriceChangeState {
    Outstanding = 0,
    Accepted = 1,
}

#[derive(Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum AcknowledgementState {
    YetToBeAcknowledged = 0,
    Acknowledged = 1,
}
#[derive(Deserialize_repr, Debug, PartialEq)]
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
    deserialize_datetime_utc_from_milliseconds(deserializer).map(|x| Some(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn simple() -> io::Result<()> {
        let json = r#"
        {
            "startTimeMillis": "1598679935343",
            "expiryTimeMillis": "1598953529067",
            "autoRenewing": true,
            "priceCurrencyCode": "TWD",
            "priceAmountMicros": "160000000",
            "countryCode": "TW",
            "developerPayload": "",
            "paymentState": 2,
            "orderId": "GPA.0000-0595-6795-14075",
            "acknowledgementState": 1,
            "kind": "androidpublisher#subscriptionPurchase"
        }
        "#;

        let r: SubscriptionPurchase = serde_json::from_str(json)?;

        assert_eq!(r.kind, PurchasesKind::SubscriptionPurchase);
        assert_eq!(r.start_time.timestamp_millis(), 1598679935343);
        assert_eq!(r.expiry_time.timestamp_millis(), 1598953529067);
        assert_eq!(r.auto_renewing, true);
        assert_eq!(r.price_currency_code, "TWD");
        assert_eq!(r.price_amount_micros, 160000000);
        assert_eq!(r.country_code, "TW");
        assert_eq!(r.developer_payload, "");
        assert_eq!(r.payment_state, Some(PaymentState::PaymentReceived));
        assert_eq!(r.order_id, "GPA.0000-0595-6795-14075");
        assert_eq!(r.acknowledgement_state, AcknowledgementState::Acknowledged);

        Ok(())
    }
}
