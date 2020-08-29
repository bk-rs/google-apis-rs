pub use isahc;

//
//
//
use std::io;
use std::result;
use std::time::Duration;

use futures_lite::io::AsyncReadExt;
pub use google_rest_client::Client;
use google_rest_client::{async_trait, Body, Request, Response};
use isahc::{config::Configurable, HttpClient};

pub struct IsahcClient {
    http_client: HttpClient,
}

impl IsahcClient {
    pub fn new() -> result::Result<Self, isahc::Error> {
        Ok(Self::with(
            HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()?,
        ))
    }

    pub fn with(http_client: HttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl Client for IsahcClient {
    async fn respond(&self, request: Request<Body>) -> io::Result<Response<Body>> {
        let res = self.http_client.send_async(request).await?;
        let (head, mut body) = res.into_parts();

        let mut body_buf = Vec::with_capacity(body.len().unwrap_or_else(|| 4 * 1024) as usize);

        body.read_to_end(&mut body_buf).await?;

        let res = Response::from_parts(head, body_buf);
        Ok(res)
    }
}
