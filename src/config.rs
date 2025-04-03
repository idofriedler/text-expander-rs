use std::{collections::HashMap, fs, path::Path};

pub fn load_shortcuts<P>(path: P) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;
    let mut shortcuts = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            shortcuts.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(shortcuts)
}
