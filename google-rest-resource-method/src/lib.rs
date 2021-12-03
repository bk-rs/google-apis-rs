pub use http_api_client_endpoint::*;

//
//
//
pub trait ResourceMethod: Endpoint {}

pub mod response_body;
pub use response_body::{ErrorResponseBody, ResponseBody};
