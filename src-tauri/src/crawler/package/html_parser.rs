use scraper::{Html, Selector};
// use regex::Regex;

pub fn extract_disabled_text(html: &str, selector_str: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector_str).ok()?;

    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>())
}

pub fn extract_total_pages_from_text(text: &str) -> Option<usize> {
    let re = regex::Regex::new(r"/(\d+)页").ok()?;
    re.captures(text)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<usize>().ok())
}

pub fn extract_all_links(html: &str, selector_str: &str) -> Option<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector_str).ok()?;

    let urls = document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .map(|href| href.to_string())
        .collect::<Vec<_>>();

    if urls.is_empty() {
        None
    } else {
        Some(urls)
    }
}

pub fn extract_video_urls(html: &str, selector_str: &str, base_url: &str) -> Option<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector_str).ok()?;

    let urls = document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .map(|href| format!("{}{}", base_url, href))
        .collect::<Vec<_>>();

    if urls.is_empty() {
        None
    } else {
        Some(urls)
    }
}

pub fn extract_img_src(html: &str, selector_str: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector_str).ok()?;

    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("src"))
        .map(|src| src.to_string())
}
