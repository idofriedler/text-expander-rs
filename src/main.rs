use simplelog::*;
use std::collections::HashMap;
use std::{
    fs::File,
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex},
    thread,
};

mod config;
mod listener;
mod expander;

use eframe::egui;

struct AppUI {
    is_enabled: Arc<AtomicBool>,
    shortcuts: Arc<Mutex<HashMap<String, String>>>,
    new_key: String,
    new_value: String,

}

impl AppUI {
    fn new(is_enabled: Arc<AtomicBool>, shortcuts: Arc<Mutex<HashMap<String, String>>> ) -> Self {
        Self {
            is_enabled,
            shortcuts,
            new_key: String::new(),
            new_value: String::new(),
        }
    }
}


impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let currently_enabled = self.is_enabled.load(Ordering::Relaxed);
            let button_text = if currently_enabled { "✅ Enabled" } else { "❌ Disabled" };

            if ui.button(button_text).clicked() {
                self.is_enabled.store(!currently_enabled, Ordering::Relaxed);
                log::info!("Toggled expander to {}", !currently_enabled);
            }

            ui.separator();
            ui.heading("📋 Shortcuts");
            
            let mut to_remove: Option<String> = None;

            // ✅ Lock the mutex before accessing
            if let Ok(map) = self.shortcuts.lock() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (key, value) in map.iter() {
                        ui.horizontal(|ui| {
                            ui.monospace(format!("{} → {}", key, value));
                            if ui.button("❌").clicked() {
                                to_remove = Some(key.clone());
                            }
                        });
                    }
                });
            } else {
                ui.label("⚠️ Failed to load shortcuts (lock error)");
            }

            if let Some(key) = to_remove {
                if let Ok(mut map) = self.shortcuts.lock() {
                    if map.remove(&key).is_some() {
                        log::info!("🗑️ Removed shortcut '{}'", key);
                        if let Err(e) = save_shortcuts_to_file(&map) {
                            log::warn!("⚠️ Failed to save shortcuts after deletion: {}", e);
                        }
                    }
                }
            }


            ui.separator();
            ui.heading("➕ Add New Shortcut");

            ui.horizontal(|ui| {
                ui.label("Shortcut:");
                ui.text_edit_singleline(&mut self.new_key);
            });

            ui.horizontal(|ui| {
                ui.label("Expansion:");
                ui.text_edit_singleline(&mut self.new_value);
            });

            if ui.button("💾 Save").clicked() {
                if !self.new_key.is_empty() && !self.new_value.is_empty() {
                    if let Ok(mut map) = self.shortcuts.lock() {
                        map.insert(self.new_key.clone(), self.new_value.clone());
                        log::info!(
                            "✅ New shortcut added: '{}' → '{}'. Total shortcuts: {}",
                            self.new_key,
                            self.new_value,
                            map.len()
                        );
                        if let Err(e) = save_shortcuts_to_file(&map) {
                            log::error!("Failed to save shortcuts: {}", e);
                        } else {
                            log::info!("Added shortcut: {} → {}", self.new_key, self.new_value);
                            self.new_key.clear();
                            self.new_value.clear();
                        }
                    }
                }
            }
        });
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/home/ido/logs/text_expander.log").unwrap(),
        ),
    ])
    .unwrap();

    log::info!("Text Expander started.");
    let shortcuts: Arc<Mutex<HashMap<String, String>>> =
    Arc::new(Mutex::new(config::load_shortcuts("/home/ido/learn_rust/shortcuts.txt")?));

    if let Ok(map) = shortcuts.lock() {
        log::info!("📋 Loaded {} shortcut(s) from file:", map.len());
    
        for (k, v) in map.iter() {
            log::info!("• '{}' → '{}'", k, v);
        }
    } else {
        log::warn!("⚠️ Failed to acquire lock to print loaded shortcuts");
    }
    
    let is_enabled = Arc::new(AtomicBool::new(true)); // Shared toggle
    let thread_enabled = Arc::clone(&is_enabled);
    let thread_shortcuts = Arc::clone(&shortcuts);

    // Run listener in background
    thread::spawn(move || {
        listener::start_listening(thread_shortcuts, thread_enabled);
    });

    

    // Launch GUI
    let options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "Text Expander",
        options,
        Box::new(|_cc| Box::new(AppUI::new(is_enabled, shortcuts))),
    )?)
}



fn save_shortcuts_to_file(map: &HashMap<String, String>) -> std::io::Result<()> {
    use std::io::Write;

    let mut file = File::create("/home/ido/learn_rust/shortcuts.txt")?;
    for (k, v) in map {
        writeln!(file, "{}:{}", k, v)?;
    }
    Ok(())
}
