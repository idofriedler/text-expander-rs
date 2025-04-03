use rdev::{listen, EventType, Key};
use std::collections::{HashMap, HashSet};

use crate::expander::Expander;

pub fn start_listening(shortcuts: HashMap<String, String>) {
    let mut expander = Expander::new(shortcuts);
    let mut modifiers: HashSet<Key> = HashSet::new();

    if let Err(e) = listen(move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
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
