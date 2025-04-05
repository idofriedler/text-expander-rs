use rdev::{listen, EventType, Key};
use std::collections::{HashMap, HashSet};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use crate::expander::Expander;

pub fn start_listening(shortcuts: HashMap<String, String>, is_enabled: Arc<AtomicBool>) {
    let mut expander = Expander::new(shortcuts);
    let mut modifiers: HashSet<Key> = HashSet::new();

    if let Err(e) = listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if !is_enabled.load(Ordering::Relaxed) {
                    return;
                }
                modifiers.insert(key);
                expander.key_pressed(key);
            }
            EventType::KeyRelease(key) => {
                modifiers.remove(&key);
            }
            _ => {}
        }
    }) {
        eprintln!("Error starting listener: {:?}", e);
    }
}
