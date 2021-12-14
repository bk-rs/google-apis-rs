pub mod images;
pub mod projects;

//
//
//
pub(crate) mod resource_method_prelude {
    pub(crate) use google_rest_resource_method::{
        http_api_client_endpoint::{Body, Endpoint, Request, Response},
        ResourceMethod,
    };
}
