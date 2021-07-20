use std::time::Duration;

use actix_http::encoding::Decoder;
use actix_http::Payload;
use anyhow::Result as AResult;
use awc::{Client, ClientResponse};
use url::Url;

#[actix_web::main]
async fn main() -> AResult<()> {
    let client = awc::ClientBuilder::new().timeout(Duration::from_secs(10)).finish();
    for _ in 1..100 {
        cause_bug(&client, "should_fail").await?;
    }
    cause_bug(&client, "my_enum_value").await?;

    Ok(())
}

pub async fn post(client: &Client, url: Url, body: &serde_json::Value) -> ClientResponse<Decoder<Payload>> {
    client.post(url.to_string()).send_json(body).await.unwrap()
}

async fn cause_bug(client: &Client, mystr: &str) -> AResult<()> {
    let url = Url::parse("http://localhost:37123")?;
    let url = url.join(&format!("/mypath/{}/{}", 33, mystr))?;
    let body = serde_json::Value::String("ignored_input".to_owned());
    let mut resp = post(client, url, &body).await;
    let json: serde_json::Value = resp.json().await?;
    let s = serde_json::to_string(&json)?;
    let _: serde_json::Value = serde_json::from_str(&s)?;

    Ok(())
}
