use super::config::{PROXY_POOL, USER_AGENTS};
use rand::seq::SliceRandom;
use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn fetch(url: &str, format: &str, token: Option<&str>) -> Result<Value, Box<dyn Error>> {
    let user_agent = USER_AGENTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&USER_AGENTS[0]);
    let client_builder = reqwest::Client::builder();
    let client_builder = if !PROXY_POOL.is_empty() {
        let proxy = PROXY_POOL.choose(&mut rand::thread_rng());
        if let Some(proxy_url) = proxy {
            client_builder.proxy(reqwest::Proxy::all(*proxy_url)?)
        } else {
            client_builder
        }
    } else {
        client_builder
    };
    let client = client_builder.build()?;
    let mut request = client
        .get(url)
        .header("User-Agent", *user_agent)
        .header("Cookie", "sessionid=simulated_cookie_value");
    if let Some(token_value) = token {
        request = request.header("Authorization", format!("Bearer {}", token_value));
    }
    let response: String = request.send().await?.text().await?;
    match format {
        "json" => Ok(serde_json::from_str(&response)?),
        "xml" => Ok(serde_xml_rs::from_str(&response)?),
        "html" => Ok(Value::String(response)),
        _ => Err("Unsupported format".into()),
    }
}
