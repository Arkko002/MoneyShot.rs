use std::path::PathBuf;

use project_root::get_project_root;
use tokio::fs;

pub fn append_path_to_root(path: &str) -> PathBuf {
    match get_project_root() {
        Ok(mut root_path) => {
            root_path.push(path);
            root_path
        }
        Err(e) => panic!("Cannot read config: {:?}", e),
    }
}

// TODO Model for config
pub async fn get_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
            let path = append_path_to_root("config.json");
            let config_str = fs::read_to_string(path).await?;

            Ok(serde_json::from_str(&config_str).unwrap())
}
