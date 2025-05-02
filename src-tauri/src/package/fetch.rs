use super::config::{PROXY_POOL, USER_AGENTS};
use rand::seq::SliceRandom;
use reqwest;
use serde_json::Value;
use std::error::Error;
use tokio::time::{sleep, Duration};

pub async fn fetch(url: &str, format: &str, token: Option<&str>) -> Result<Value, Box<dyn Error + Send + Sync>> {
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
    let mut last_err: Option<Box<dyn Error + Send + Sync>> = None;
    let max_retries = 5;
    let retry_delay_secs = 2;
    for attempt in 0..max_retries {
        let mut request = client
            .get(url)
            .header("User-Agent", *user_agent)
            .header("Cookie", "sessionid=simulated_cookie_value");
        if let Some(token_value) = token {
            request = request.header("Authorization", format!("Bearer {}", token_value));
        }
        let result = request.send().await;
        match result {
            Ok(resp) => {
                let response = resp.text().await;
                match response {
                    Ok(response_text) => {
                        let value = match format {
                            "json" => Ok(serde_json::from_str(&response_text)?),
                            "xml" => Ok(serde_xml_rs::from_str(&response_text)?),
                            "html" => Ok(Value::String(response_text)),
                            _ => Err("Unsupported format".to_string().into()),
                        };
                        return value;
                    },
                    Err(e) => {
                        last_err = Some(Box::new(e));
                        if attempt < max_retries - 1 {
                            sleep(Duration::from_secs(retry_delay_secs)).await;
                        }
                    }
                }
            },
            Err(e) => {
                last_err = Some(Box::new(e));
                if attempt < max_retries - 1 {
                    sleep(Duration::from_secs(retry_delay_secs)).await;
                }
            }
        }
    }
    Err(last_err.unwrap_or_else(|| "请求失败且已达最大重试次数".to_string().into()))
}

