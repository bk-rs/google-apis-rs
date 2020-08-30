// ref https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get

use std::io;

use http::{
    header::{ACCEPT, AUTHORIZATION},
    Method, StatusCode, Version,
};

use crate::v3::resources::resource_method_prelude::*;
use crate::v3::ProductPurchase;

pub struct PurchasesproductsGet {
    package_name: String,
    product_id: String,
    token: String,
    access_token: String,
}
impl PurchasesproductsGet {
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

impl Endpoint for PurchasesproductsGet {
    type ParseResponseOutput = ProductPurchase;
    type RetryReason = ();

    fn render_request(&self) -> io::Result<Request<Body>> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/products/{}/tokens/{}", self.package_name, self.product_id, self.token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .version(Version::HTTP_11)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(request)
    }

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>> {
        match response.status() {
            StatusCode::OK => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("status [{}] mismatch", response.status()),
                ));
            }
        }

        let body: ProductPurchase = serde_json::from_slice(response.body())?;

        Ok(EndpointParseResponseOutput::Done(body))
    }
}

impl ResourceMethod for PurchasesproductsGet {}
