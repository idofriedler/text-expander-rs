// config.rs
use std::{collections::HashMap, fs, path::Path};

#[allow(dead_code)]
pub struct AppPaths {
    pub data_dir: String,
    pub state_dir: String,
    pub log_file: String,
    pub shortcuts_file: String,
}

pub fn load_shortcuts(path: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    if !path.exists() {
        fs::write(path, "")?;
        log::info!("✨ Created shortcuts file at {}", path.display());
    }
    let contents = fs::read_to_string(path)?;
    let map = contents
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();
    Ok(map)
}

pub fn save_shortcuts_to_file(
    path: &Path,
    shortcuts: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data: String = shortcuts
        .iter()
        .map(|(k, v)| format!("{}:{}\n", k, v))
        .collect();
    fs::write(path, data)?;
    log::info!("💾 Shortcuts saved to {}", path.display());
    Ok(())
}
