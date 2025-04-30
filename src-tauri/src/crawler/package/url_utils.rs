use crate::crawler::package::config::API_CLASS_URL;
  
 
use crate::structs::structs::CrawlParams; 
 

pub fn build_type_urls(params:CrawlParams) -> Result<Vec<String>, Box<dyn std::error::Error>> { 

   // 创建一个 Vec 包含 params
   let params_vec = vec![params];
    
   // 将 Vector 转换为 JSON 字符串
   let json_string = serde_json::to_string(&params_vec)?;
   let types: Vec<CrawlParams> = serde_json::from_str(&json_string)?;
   
 
    let urls = types
        .into_iter()
        .map(|vod| format!("{}{}{}", API_CLASS_URL, vod.id, "/1.html"))
        .collect();
  
      
    Ok(urls)
}

pub fn generate_paged_urls(base_url: &str, total_pages: u32) -> Vec<String> {
    (1..=total_pages)
        .map(|page| {
            let base = base_url.trim_end_matches("/1.html");
            format!("{}/page/{}.html", base, page)
        })
        .collect()
}