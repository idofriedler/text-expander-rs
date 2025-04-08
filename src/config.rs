// config.rs
use std::{collections::HashMap, fs, path::Path, path::PathBuf};
use directories::ProjectDirs;


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


pub fn setup_paths() -> (PathBuf, PathBuf) {
    // Try to get system's appropriate project directories
    let proj_dirs = ProjectDirs::from("com", "YourName", "TextExpander")
        .expect("❌ Couldn't get a valid project directory (unsupported OS?)");

    // Preferred directory for data (shortcuts.txt, etc.)
    let data_dir = proj_dirs.data_local_dir();

    // Preferred state dir for runtime logs, or fallback to data_dir
    let state_dir = proj_dirs
        .state_dir()
        .unwrap_or_else(|| proj_dirs.data_local_dir());

    // Create directories if they don't exist
    fs::create_dir_all(data_dir).expect("❌ Failed to create data_dir");
    fs::create_dir_all(state_dir).expect("❌ Failed to create state_dir");

    (data_dir.to_path_buf(), state_dir.to_path_buf())
}
