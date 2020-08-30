// ref https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get

use std::io;

use http::{
    header::{ACCEPT, AUTHORIZATION},
    Method, StatusCode, Version,
};

use serde::Deserialize;

use super::resource_method_prelude::*;

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
    type ParseResponseOutput = ResponseBody;
    type RetryReason = ();

    fn render_request(&self) -> io::Result<Request<Body>> {
        let url = format!("https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/subscriptions/{}/tokens/{}", self.package_name, self.subscription_id, self.token);

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
        println!("{:?}", response.body());
        let body: ResponseBody = serde_json::from_slice(response.body())?;

        Ok(EndpointParseResponseOutput::Done(body))
    }
}

impl ResourceMethod for PurchasesSubscriptionsGet {}

//
//
//
#[derive(Deserialize, Debug)]
pub struct ResponseBody {}
