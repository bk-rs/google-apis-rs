/*
RUST_BACKTRACE=full RUST_LOG=trace cargo run -p google-androidpublisher-rest-demo --bin google_androidpublisher_rest_verify_purchases_product 'YOUR_GOOGLE_IAP_ACCESS_TOKEN' 'package_name' 'product_id' 'token'
*/

use std::{env, error};

use futures_lite::future::block_on;
use google_androidpublisher_rest::v3::PurchasesProductsGet;
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let google_iap_access_token = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("GOOGLE_IAP_ACCESS_TOKEN").unwrap());
    let package_name = env::args().nth(2).unwrap();
    let product_id = env::args().nth(3).unwrap();
    let token = env::args().nth(4).unwrap();

    let isahc_client = IsahcClient::new()?;

    //
    let resource_method =
        PurchasesProductsGet::new(package_name, product_id, token, google_iap_access_token);

    let ret = isahc_client.respond_endpoint(&resource_method).await?;

    println!("{:?}", ret);

    Ok(())
}
