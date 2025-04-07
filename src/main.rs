use simplelog::*;
use std::collections::HashMap;
use std::{
    fs::File,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

mod config;
mod listener;
mod expander;
mod gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("/home/ido/logs/text_expander.log").unwrap(),
        ),
    ])
    .unwrap();

    log::info!("Text Expander started.");
    let is_enabled = Arc::new(AtomicBool::new(true)); // Shared toggle
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
    
    
    let thread_enabled = Arc::clone(&is_enabled);
    let thread_shortcuts = Arc::clone(&shortcuts);

    // Run listener in background
    thread::spawn(move || {
        listener::start_listening(thread_shortcuts, thread_enabled);
    });

    // Launch the GUI (blocking call)
    gui::run_gui(is_enabled, shortcuts)?;
    Ok(())
    
    /*
    // Launch GUI
    let options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "Text Expander",
        options,
        Box::new(|_cc| Box::new(AppUI::new(is_enabled, shortcuts))),
    )?)
     */
}



