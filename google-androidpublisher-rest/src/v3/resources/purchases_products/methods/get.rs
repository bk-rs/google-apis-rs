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

use super::super::ProductPurchase;

pub struct PurchasesProductsGet {
    package_name: String,
    product_id: String,
    token: String,
    oauth2_access_token: String,
}
impl PurchasesProductsGet {
    pub fn new(
        package_name: String,
        product_id: String,
        token: String,
        oauth2_access_token: String,
    ) -> Self {
        Self {
            package_name,
            product_id,
            token,
            oauth2_access_token,
        }
    }
}

impl Endpoint for PurchasesProductsGet {
    type RenderRequestError = MethodEndpointError;

    type ParseResponseOutput = MethodEndpointRet<ProductPurchase>;
    type ParseResponseError = MethodEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/products/{}/tokens/{}", self.package_name, self.product_id, self.token);

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
