//! [Ref](https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get)

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION},
        Method,
    },
    Body, Endpoint, Request, Response,
};

use crate::v3::resources::method_common::{
    method_endpoint_parse_response, MethodEndpointError, MethodEndpointRet,
};

use super::super::SubscriptionPurchase;

pub struct PurchasesSubscriptionsGet {
    package_name: String,
    subscription_id: String,
    token: String,
    oauth2_access_token: String,
}
impl PurchasesSubscriptionsGet {
    pub fn new(
        package_name: String,
        subscription_id: String,
        token: String,
        oauth2_access_token: String,
    ) -> Self {
        Self {
            package_name,
            subscription_id,
            token,
            oauth2_access_token,
        }
    }
}

impl Endpoint for PurchasesSubscriptionsGet {
    type RenderRequestError = MethodEndpointError;

    type ParseResponseOutput = MethodEndpointRet<SubscriptionPurchase>;
    type ParseResponseError = MethodEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/subscriptions/{}/tokens/{}", self.package_name, self.subscription_id, self.token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth2_access_token),
            )
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(MethodEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        method_endpoint_parse_response(response)
    }
}
