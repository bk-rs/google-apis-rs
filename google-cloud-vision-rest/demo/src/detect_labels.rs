/*
RUST_BACKTRACE=full RUST_LOG=trace cargo run -p google-cloud-vision-rest-demo --bin google_cloud_vision_rest_detect_labels 'YOUR_API_KEY'
*/

use std::{env, error};

use futures_lite::future::block_on;
use google_cloud_vision_rest::v1::{
    types::{annotate_image_request::Image, feature::Type, AnnotateImageRequest, Feature},
    ImagesAnnotate, ImagesAnnotateRequestBody,
};
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let api_key = env::args().nth(1).unwrap();

    let image_bytes = include_bytes!("../../tests/image_files/setagaya_small.jpeg");

    //
    let resource_method = ImagesAnnotate::new(
        ImagesAnnotateRequestBody {
            requests: vec![AnnotateImageRequest {
                image: Image::with_bytes(image_bytes.to_vec()),
                features: vec![Feature {
                    r#type: Type::LABEL_DETECTION,
                    max_results: Some(5),
                    model: None,
                }],
                image_context: None,
            }],
            parent: None,
        },
        None,
    );

    let isahc_client = IsahcClient::new()?;

    let response_body = isahc_client
        .respond_endpoint_with_callback(
            &resource_method,
            |mut req| {
                *req.uri_mut() = format!("{}?key={}", req.uri(), api_key).parse().unwrap();

                req
            },
            |_| {},
        )
        .await?;

    println!("{:?}", response_body);

    Ok(())
}
