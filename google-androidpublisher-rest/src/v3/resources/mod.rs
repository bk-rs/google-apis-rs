pub mod purchases_products;
pub mod purchases_subscriptions;

//
//
//
pub(crate) mod resource_method_prelude {
    pub(crate) use google_rest_resource_method::{
        http_api_client_endpoint::{http, Body, Endpoint, Request, Response},
        ResourceMethod,
    };
}
