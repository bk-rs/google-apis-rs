// ref https://developers.google.com/android-publisher/api-ref/rest/v3/purchases.subscriptions/get

use std::io;

use http::{
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
    Method, StatusCode, Version,
};

use serde::Deserialize;

use super::resource_method_prelude::*;

pub struct PurchasesSubscriptionsGet {
    package_name: String,
    subscription_id: String,
    token: String,
}
impl PurchasesSubscriptionsGet {
    pub fn new(package_name: String, subscription_id: String, token: String) -> Self {
        Self {
            package_name,
            subscription_id,
            token,
        }
    }
}

impl Endpoint for PurchasesSubscriptionsGet {
    type ParseResponseOutput = ResponseBody;
    type RetryReason = ();

    fn render_request(&self) -> io::Result<Request<Body>> {
        todo!()
    }

    fn parse_response(
        &mut self,
        response: Response<Body>,
    ) -> io::Result<EndpointParseResponseOutput<Self::ParseResponseOutput, Self::RetryReason>> {
        todo!()
    }
}

impl ResourceMethod for PurchasesSubscriptionsGet {}

//
//
//
#[derive(Deserialize, Debug)]
pub struct ResponseBody {}
