/*
cargo run -p googleapis-demo-async-std --bin iap_verify_subscription 'YOUR_GOOGLE_IAP_ACCESS_TOKEN' 'package_name' 'subscription_id' 'token'
*/

use std::{env, error};

use google_androidpublisher_rest::v3::PurchasesSubscriptionsGet;
use http_api_isahc_client::{Client as _, IsahcClient};

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let google_iap_access_token = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("GOOGLE_IAP_ACCESS_TOKEN").unwrap());
    let package_name = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PACKAGE_NAME").unwrap());
    let subscription_id = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("SUBSCRIPTION_ID").unwrap());
    let token = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("TOKEN").unwrap());

    println!("iap_verify_subscription");

    //
    let resource_method = PurchasesSubscriptionsGet::new(
        package_name,
        subscription_id,
        token,
        google_iap_access_token,
    );

    let isahc_client = IsahcClient::new()?;

    let resource = isahc_client.respond_endpoint(&resource_method).await?;

    println!("{:?}", resource);

    println!("done");

    Ok(())
}
