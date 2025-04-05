use rdev::{listen, EventType, Key};
use std::collections::{HashMap, HashSet};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
use crate::expander::Expander;

pub fn start_listening(shortcuts: Arc<Mutex<HashMap<String, String>>>, is_enabled: Arc<AtomicBool>) {
    let mut expander = Expander::new(HashMap::new());
    let mut modifiers: HashSet<Key> = HashSet::new();

    if let Err(e) = listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if !is_enabled.load(Ordering::Relaxed) {
                    return;
                }
                modifiers.insert(key);
                // 🔁 Get current shortcuts from shared map
                if let Ok(map) = shortcuts.lock() {
                    expander.set_shortcuts(map.clone());
                    let shift = modifiers.contains(&Key::ShiftLeft) || modifiers.contains(&Key::ShiftRight);
                    expander.key_pressed(key, shift);
                }
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
