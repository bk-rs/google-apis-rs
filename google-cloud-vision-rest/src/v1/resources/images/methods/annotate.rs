//! [Ref](https://cloud.google.com/vision/docs/reference/rest/v1/images/annotate)

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION},
        Method,
    },
    MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};

use crate::v1::{
    resources::{
        method_common::{method_endpoint_parse_response, MethodEndpointError, MethodEndpointRet},
        resource_method_prelude::*,
    },
    types::{AnnotateImageRequest, BatchAnnotateImagesResponse},
};

pub const URL: &str = "https://vision.googleapis.com/v1/images:annotate";

pub struct ImagesAnnotate {
    request_body: ImagesAnnotateRequestBody,
    access_token: String,
}
impl ImagesAnnotate {
    pub fn new(request_body: ImagesAnnotateRequestBody, access_token: String) -> Self {
        Self {
            request_body,
            access_token,
        }
    }
}

impl Endpoint for ImagesAnnotate {
    type RenderRequestError = MethodEndpointError;

    type ParseResponseOutput = MethodEndpointRet<BatchAnnotateImagesResponse>;
    type ParseResponseError = MethodEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = serde_json::to_vec(&self.request_body)
            .map_err(MethodEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(body)
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

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImagesAnnotateRequestBody {
    pub requests: Vec<AnnotateImageRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}
