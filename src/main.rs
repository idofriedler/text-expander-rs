use simplelog::*;
use std::{
    fs::File,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};


mod config;
mod expander;
mod gui;
mod listener;

fn main() -> Result<(), eframe::Error> {
    let (data_dir, state_dir) = config::setup_paths();

    let log_path = state_dir.join("text_expander.log");
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create(&log_path).unwrap(),
    )])
    .unwrap();

    log::info!("🚀 Text Expander started.");
    log::info!("📂 Data directory: {}", data_dir.display());
    log::info!("📂 State directory: {}", state_dir.display());
    log::info!("📂 Log file: {}", log_path.display());

    let shortcuts_path = data_dir.join("shortcuts.txt");
    let shortcuts = Arc::new(Mutex::new(
        config::load_shortcuts(&shortcuts_path).unwrap_or_default(),
    ));

    log::info!("📋 Loaded shortcuts from: {}", shortcuts_path.display());

    let is_enabled = Arc::new(AtomicBool::new(true));
    let thread_enabled = Arc::clone(&is_enabled);
    let thread_shortcuts = Arc::clone(&shortcuts);

    thread::spawn(move || {
        listener::start_listening(thread_shortcuts, thread_enabled);
    });

    let app_paths = config::AppPaths {
        data_dir: data_dir.to_string_lossy().into_owned(),
        state_dir: state_dir.to_string_lossy().into_owned(),
        log_file: log_path.to_string_lossy().into_owned(),
        shortcuts_file: shortcuts_path.to_string_lossy().into_owned(),
    };

    gui::run_gui(is_enabled, shortcuts, app_paths)
}
