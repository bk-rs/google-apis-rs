pub mod images;
pub mod projects;

pub mod method_common;

//
//
//
pub(crate) mod resource_method_prelude {
    pub(crate) use http_api_client_endpoint::{Body, Endpoint, Request, Response};
}
