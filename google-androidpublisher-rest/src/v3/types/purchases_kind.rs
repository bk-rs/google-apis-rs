use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum PurchasesKind {
    #[serde(rename(deserialize = "androidpublisher#productPurchase"))]
    ProductPurchase,

    #[serde(rename(deserialize = "androidpublisher#subscriptionPurchase"))]
    SubscriptionPurchase,
}
