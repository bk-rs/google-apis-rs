// ref https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get

use google_rest_resource_method::ResponseBody;
use http::{
    header::{ACCEPT, AUTHORIZATION},
    Error as HttpError, Method, StatusCode, Version,
};
use serde_json::Error as SerdeJsonError;

use crate::v3::{resources::resource_method_prelude::*, ProductPurchase};

pub struct PurchasesProductsGet {
    package_name: String,
    product_id: String,
    token: String,
    access_token: String,
}
impl PurchasesProductsGet {
    pub fn new(
        package_name: String,
        product_id: String,
        token: String,
        access_token: String,
    ) -> Self {
        Self {
            package_name,
            product_id,
            token,
            access_token,
        }
    }
}

impl Endpoint for PurchasesProductsGet {
    type RenderRequestError = PurchasesProductsGetError;

    type ParseResponseOutput = ResponseBody<ProductPurchase>;
    type ParseResponseError = PurchasesProductsGetError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/products/{}/tokens/{}", self.package_name, self.product_id, self.token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .version(Version::HTTP_11)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(PurchasesProductsGetError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        match response.status() {
            StatusCode::OK => {}
            _ => {
                return Err(PurchasesProductsGetError::StatusMismatch(response.status()));
            }
        }

        let body: Self::ParseResponseOutput = serde_json::from_slice(response.body())
            .map_err(PurchasesProductsGetError::DeResponseBodyOkJsonFailed)?;

        Ok(body)
    }
}

impl ResourceMethod for PurchasesProductsGet {}

#[derive(thiserror::Error, Debug)]
pub enum PurchasesProductsGetError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("StatusMismatch {0}")]
    StatusMismatch(StatusCode),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
}
