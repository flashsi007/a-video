use std::fs;
use serde::Deserialize;
use crate::crawler::package::config::API_CLASS_URL;

#[derive(Deserialize)]
struct VodType {
    id: u32,
}

pub fn build_type_urls(json_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    
    let data = fs::read_to_string(json_path)?;
    
    
    let types: Vec<VodType> = serde_json::from_str(&data)?;
    let urls = types
        .into_iter()
        .map(|vod| format!("{}{}{}", API_CLASS_URL, vod.id, "/1.html"))
        .collect();
    Ok(urls)
}

pub fn generate_paged_urls(base_url: &str, total_pages: u32) -> Vec<String> {
    (1..=total_pages)
        .map(|page| {
            let base = base_url.trim_end_matches("1.html");
            format!("{}{}.html", base, page)
        })
        .collect()
}