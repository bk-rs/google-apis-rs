pub use google_rest_endpoint::*;

//
//
//
pub trait ResourceMethod: Endpoint {}

pub mod response_body;
pub use response_body::{ErrorResponseBody, ResponseBody};
