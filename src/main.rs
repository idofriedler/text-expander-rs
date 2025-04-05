use simplelog::*;
use std::collections::HashMap;
use std::{
    fs::File,
    sync::{atomic::{AtomicBool, Ordering}, Arc},
    thread,
};

mod config;
mod listener;
mod expander;

use eframe::egui;

struct AppUI {
    is_enabled: Arc<AtomicBool>,
    shortcuts: HashMap<String, String>,
}

impl AppUI {
    fn new(is_enabled: Arc<AtomicBool>, shortcuts: HashMap<String, String>) -> Self {
        Self { is_enabled, shortcuts }
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

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (key, value) in &self.shortcuts {
                    ui.horizontal(|ui| {
                        ui.monospace(format!("{} → {}", key, value));
                    });
                }
            });
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/home/ido/logs/text_expander.log").unwrap(),
        ),
    ])
    .unwrap();

    log::info!("Text Expander started.");
    let shortcuts = config::load_shortcuts("/home/ido/learn_rust/shortcuts.txt")
        .expect("Failed to load shortcuts.");

    let is_enabled = Arc::new(AtomicBool::new(true)); // Shared toggle
    let thread_enabled = Arc::clone(&is_enabled);
    let thread_shortcuts = shortcuts.clone();

    // Run listener in background
    thread::spawn(move || {
        listener::start_listening(thread_shortcuts, thread_enabled);
    });

    

    // Launch GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Text Expander",
        options,
        Box::new(|_cc| Box::new(AppUI::new(is_enabled, shortcuts))),
    )
}
