use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub url: String,
    pub label: String,
    pub base_path: String,
    pub title: String,
}
