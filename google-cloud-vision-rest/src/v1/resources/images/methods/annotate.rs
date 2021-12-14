//! [Ref](https://cloud.google.com/vision/docs/reference/rest/v1/images/annotate)

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};

use crate::v1::{
    resources::method_common::{
        method_endpoint_parse_response, MethodEndpointError, MethodEndpointRet,
    },
    types::{AnnotateImageRequest, BatchAnnotateImagesResponse},
};

pub const URL: &str = "https://vision.googleapis.com/v1/images:annotate";

#[derive(Debug, Clone)]
pub struct ImagesAnnotate {
    request_body: ImagesAnnotateRequestBody,
    oauth2_access_token: Option<String>,
}

impl ImagesAnnotate {
    pub fn new(
        request_body: ImagesAnnotateRequestBody,
        oauth2_access_token: Option<String>,
    ) -> Self {
        Self {
            request_body,
            oauth2_access_token,
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

        let mut request = Request::builder().method(Method::POST).uri(URL);

        if let Some(oauth2_access_token) = &self.oauth2_access_token {
            request = request.header(AUTHORIZATION, format!("Bearer {}", oauth2_access_token));
        }

        let request = request
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body() {
        match serde_json::from_str::<BatchAnnotateImagesResponse>(include_str!(
            "../../../../../tests/response_body_files/images_annotate_ok.json"
        )) {
            Ok(body) => {
                assert_eq!(body.responses.len(), 1);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
