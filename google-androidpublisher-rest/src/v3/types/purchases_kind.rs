use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PurchasesKind {
    #[serde(rename(deserialize = "androidpublisher#productPurchase"))]
    ProductPurchase,

    #[serde(rename(deserialize = "androidpublisher#subscriptionPurchase"))]
    SubscriptionPurchase,
}
