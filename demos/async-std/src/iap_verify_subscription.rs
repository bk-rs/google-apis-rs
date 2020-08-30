/*
cargo run -p googleapis-demo-async-std --bin iap_verify_subscription 'YOUR_GOOGLE_IAP_ACCESS_TOKEN' 'package_name' 'subscription_id' 'token'
*/

use std::env;
use std::io;

use google_androidpublisher_rest::v3::PurchasesSubscriptionsGet;
use google_rest_isahc_client::{Client, IsahcClient};

#[async_std::main]
async fn main() -> io::Result<()> {
    run().await
}

async fn run() -> io::Result<()> {
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
    let mut verify_receipt = PurchasesSubscriptionsGet::new(
        package_name,
        subscription_id,
        token,
        google_iap_access_token,
    );

    let isahc_client = IsahcClient::new()?;

    let response_body = isahc_client
        .respond_endpoint_until_done(&mut verify_receipt, None)
        .await?;

    println!("{:?}", response_body);

    println!("done");

    Ok(())
}
