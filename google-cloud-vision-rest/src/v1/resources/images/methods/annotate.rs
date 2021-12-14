//! [Ref](https://cloud.google.com/vision/docs/reference/rest/v1/images/annotate)

use google_rest_resource_method::http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION},
        Error as HttpError, Method, StatusCode,
    },
    MIME_APPLICATION_JSON,
};
use google_rest_resource_method::ResponseBody;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

use crate::v1::{
    resources::resource_method_prelude::*,
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
    type RenderRequestError = ImagesAnnotateError;

    type ParseResponseOutput = ResponseBody<BatchAnnotateImagesResponse>;
    type ParseResponseError = ImagesAnnotateError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = serde_json::to_vec(&self.request_body)
            .map_err(ImagesAnnotateError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(body)
            .map_err(ImagesAnnotateError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        match response.status() {
            StatusCode::OK => Ok(serde_json::from_slice(response.body())
                .map_err(ImagesAnnotateError::DeResponseBodyFailed)?),
            _ => Err(ImagesAnnotateError::StatusMismatch(response.status())),
        }
    }
}

impl ResourceMethod for ImagesAnnotate {}

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImagesAnnotateRequestBody {
    pub requests: Vec<AnnotateImageRequest>,
    pub parent: Option<String>,
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum ImagesAnnotateError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("StatusMismatch {0}")]
    StatusMismatch(StatusCode),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
