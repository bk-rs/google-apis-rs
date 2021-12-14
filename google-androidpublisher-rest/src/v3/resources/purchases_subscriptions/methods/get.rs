//! [Ref](https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get)

use google_rest_resource_method::http_api_client_endpoint::http::{
    header::{ACCEPT, AUTHORIZATION},
    Error as HttpError, Method, StatusCode,
};
use google_rest_resource_method::ResponseBody;
use serde_json::Error as SerdeJsonError;

use crate::v3::resources::resource_method_prelude::*;

use super::super::SubscriptionPurchase;

pub struct PurchasesSubscriptionsGet {
    package_name: String,
    subscription_id: String,
    token: String,
    access_token: String,
}
impl PurchasesSubscriptionsGet {
    pub fn new(
        package_name: String,
        subscription_id: String,
        token: String,
        access_token: String,
    ) -> Self {
        Self {
            package_name,
            subscription_id,
            token,
            access_token,
        }
    }
}

impl Endpoint for PurchasesSubscriptionsGet {
    type RenderRequestError = PurchasesSubscriptionsGetError;

    type ParseResponseOutput = ResponseBody<SubscriptionPurchase>;
    type ParseResponseError = PurchasesSubscriptionsGetError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/subscriptions/{}/tokens/{}", self.package_name, self.subscription_id, self.token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(PurchasesSubscriptionsGetError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        match response.status() {
            StatusCode::OK => {}
            _ => {
                return Err(PurchasesSubscriptionsGetError::StatusMismatch(
                    response.status(),
                ));
            }
        }

        let body: Self::ParseResponseOutput = serde_json::from_slice(response.body())
            .map_err(PurchasesSubscriptionsGetError::DeResponseBodyOkJsonFailed)?;

        Ok(body)
    }
}

impl ResourceMethod for PurchasesSubscriptionsGet {}

#[derive(thiserror::Error, Debug)]
pub enum PurchasesSubscriptionsGetError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("StatusMismatch {0}")]
    StatusMismatch(StatusCode),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
}
