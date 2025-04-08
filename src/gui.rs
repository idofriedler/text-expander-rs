use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::AtomicBool, atomic::Ordering};
use eframe::egui::{self, Color32, RichText};
use crate::config::{self, AppPaths};

pub struct AppUI {
    pub is_enabled: Arc<AtomicBool>,
    pub shortcuts: Arc<Mutex<HashMap<String, String>>>,
    pub paths: AppPaths,
    new_key: String,
    new_value: String,
}

impl AppUI {
    pub fn new(
        is_enabled: Arc<AtomicBool>,
        shortcuts: Arc<Mutex<HashMap<String, String>>>,
        paths: AppPaths,
    ) -> Self {
        Self {
            is_enabled,
            shortcuts,
            paths,
            new_key: String::new(),
            new_value: String::new(),
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let enabled = self.is_enabled.load(Ordering::Relaxed);
            let button_label = if enabled {
                RichText::new("ON").color(Color32::GREEN)
            } else {
                RichText::new("OFF").color(Color32::RED)
            };

            if ui.button(button_label).clicked() {
                self.is_enabled.store(!enabled, Ordering::Relaxed);
                log::info!("🔄 Toggled expander to {}", !enabled);
            }

            ui.separator();
            ui.heading("📋 Shortcuts:");

            let mut to_remove = None;

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
                ui.label("⚠️ Failed to load shortcuts");
                log::warn!("⚠️ Failed to acquire lock for shortcuts");
            }

            if let Some(key) = to_remove {
                if let Ok(mut map) = self.shortcuts.lock() {
                    if map.remove(&key).is_some() {
                        log::info!("🗑️ Removed shortcut '{}'", key);
                        if let Err(e) = config::save_shortcuts_to_file(self.paths.shortcuts_file.as_str().as_ref(), &map)
                        {
                            log::error!("❌ Failed to save shortcuts: {}", e);
                        }
                    }
                }
            }

            ui.separator();
            ui.heading("➕ Add New Shortcut");
            ui.horizontal(|ui| {
                ui.label("Shortcut:");
                ui.text_edit_singleline(&mut self.new_key);
                ui.label("Expansion:");
                ui.text_edit_singleline(&mut self.new_value);
            });

            if ui.button("💾 Add Shortcut").clicked() {
                let key = self.new_key.trim();
                let value = self.new_value.trim();
                if !key.is_empty() && !value.is_empty() {
                    if let Ok(mut map) = self.shortcuts.lock() {
                        map.insert(key.to_string(), value.to_string());
                        log::info!("✨ Added shortcut '{}': '{}'", key, value);
                        if let Err(e) = config::save_shortcuts_to_file(self.paths.shortcuts_file.as_str().as_ref(), &map)
                        {
                            log::error!("❌ Failed to save shortcuts: {}", e);
                        }
                        self.new_key.clear();
                        self.new_value.clear();
                    }
                } else {
                    log::warn!("⚠️ Shortcut or expansion was empty, ignoring...");
                }
            }
        });
    }
}

pub fn run_gui(
    is_enabled: Arc<AtomicBool>,
    shortcuts: Arc<Mutex<HashMap<String, String>>>,
    paths: AppPaths,
) -> Result<(), eframe::Error> {
    let app = AppUI::new(is_enabled, shortcuts, paths);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Text Expander", native_options, Box::new(|_cc| Box::new(app)))
}
