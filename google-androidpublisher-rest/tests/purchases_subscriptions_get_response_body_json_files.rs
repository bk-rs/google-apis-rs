use std::fs;
use std::io;
use std::path::PathBuf;

use google_androidpublisher_rest::v3::types::purchases_kind::PurchasesKind;
use google_androidpublisher_rest::v3::ProductPurchase;
use google_rest_resource_method::ResponseBody;

#[test]
fn de_all() -> io::Result<()> {
    let dir = PathBuf::new().join("tests/purchases_subscriptions_get_response_body_json_files");
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && Some(Some("json")) == path.extension().map(|x| x.to_str()) {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<ResponseBody<ProductPurchase>>(&content) {
                Ok(response_body) => match response_body {
                    ResponseBody::Success(resource) => {
                        assert_eq!(resource.kind, PurchasesKind::SubscriptionPurchase);

                        println!("path {:?} de successful", path);
                    }
                    ResponseBody::Error(body) => {
                        println!("path {:?} de successful, body: {:?}", path, body);
                    }
                },
                Err(err) => {
                    eprintln!("path {:?} de failed, err: {:?}", path, err);
                    return Err(io::Error::new(io::ErrorKind::Other, err));
                }
            }
        }
    }

    Ok(())
}
