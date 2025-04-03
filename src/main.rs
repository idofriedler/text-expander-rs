use simplelog::*;
use std::fs::File;

mod config;
mod listener;
mod expander;

fn main() {
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("/home/ido/logs/text_expander.log").unwrap()),
    ]).unwrap();

    log::info!("Text Expander started.");
    let shortcuts = config::load_shortcuts("/home/ido/learn_rust/shortcuts.txt")
        .expect("Failed to load shortcuts.");

    log::info!("Shortcuts loaded successfully:");
    for (key, value) in &shortcuts {
        log::info!("'{}' expands to '{}'", key, value);
    }

    listener::start_listening(shortcuts);
}
