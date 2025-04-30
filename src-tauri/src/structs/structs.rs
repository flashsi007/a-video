use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrawlParams {
    pub id: u32,
    pub name: String,
}
