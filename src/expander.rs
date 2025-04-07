
use std::collections::{HashMap, VecDeque};
use std::{thread, time::Duration};

use rdev::{simulate, EventType, Key};
use rdev::Key::*;

pub struct Expander {
    shortcuts: HashMap<String, String>,
    buffer: VecDeque<char>,
    max_shortcut_length: usize,
}

impl Expander {
    pub fn new(shortcuts: HashMap<String, String>) -> Self {
        let max_shortcut_length = shortcuts.keys().map(|k| k.len()).max().unwrap_or(10);
        Self {
            shortcuts,
            buffer: VecDeque::with_capacity(max_shortcut_length),
            max_shortcut_length,
        }
    }

    pub fn key_pressed(&mut self, key: Key, shift: bool) {
    use std::time::Instant;

    // Log every key to char conversion
    if let Some(c) = key_to_char(key, shift) {
        log::debug!("Key pressed: {:?}, shift: {}, resolved char: '{}'", key, shift, c);
    } else {
        log::debug!("Key pressed: {:?}, shift: {}, but no valid char", key, shift);
    }

    // Handle Tab (expansion trigger)
    if key == Key::Tab {
        let typed: String = self.buffer.iter().collect();
        log::info!("🔍 Buffer before Tab: '{}'", typed);
        /*
        if typed.ends_with("!testback") {
            log::info!("🚀 Triggered test shortcut '!testback'");
            self.run_backspace_typing_test();
            self.buffer.clear();
            return;
        }
         */
        for (shortcut, expansion) in &self.shortcuts {
            if typed.ends_with(shortcut) {
                log::info!("✅ Shortcut matched: '{}'", shortcut);
                log::info!("➡️ Will expand to: '{}'", expansion);

                let chars_to_delete = shortcut.len() + 1; // +1 for Tab itself
                log::info!("⌫ Will send {} backspaces", chars_to_delete);

                let start = Instant::now();
                for i in 0..chars_to_delete {
                    simulate(&EventType::KeyPress(Key::Backspace)).ok();
                    thread::sleep(Duration::from_millis(20));
                    simulate(&EventType::KeyRelease(Key::Backspace)).ok();
                    log::debug!("⌫ Sent backspace {}/{}", i + 1, chars_to_delete);
                }
                log::debug!("⌫ Backspacing took {:?}", start.elapsed());

                for ch in expansion.chars() {
                    if let Some(k) = char_to_key(ch) {
                        simulate(&EventType::KeyPress(k)).ok();
                        thread::sleep(Duration::from_millis(20));
                        simulate(&EventType::KeyRelease(k)).ok();
                        log::debug!("⌨️ Typed '{}'", ch);
                    } else {
                        log::warn!("⚠️ Could not convert '{}' to key", ch);
                    }
                }

                log::info!("✅ Expansion complete, clearing buffer");
                self.buffer.clear();
                return;
            }
        }

        log::info!("❌ No shortcut matched for '{}', clearing buffer", typed);
        self.buffer.clear();
        return;
    }

    // Regular character input
    if let Some(c) = key_to_char(key, shift) {
        if self.buffer.len() == self.max_shortcut_length {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);
        log::debug!("🧠 Buffer updated: \"{}\"", self.buffer.iter().collect::<String>());
    }
}

    pub fn set_shortcuts(&mut self, shortcuts: HashMap<String, String>) {
        self.shortcuts = shortcuts;
    }
    

    /* 
    fn run_backspace_typing_test(&self) {
        log::info!("🔬 Running in-app backspace+typing test...");

        for i in 0..5 {
            simulate(&EventType::KeyPress(Key::Backspace)).ok();
            thread::sleep(Duration::from_millis(10));
            simulate(&EventType::KeyRelease(Key::Backspace)).ok();
            thread::sleep(Duration::from_millis(10));
            log::debug!("Sent backspace {}", i);
        }

        let text = "hello";
        for ch in text.chars() {
            if let Some(key) = char_to_key(ch) {
                simulate(&EventType::KeyPress(key)).ok();
                thread::sleep(Duration::from_millis(10));
                simulate(&EventType::KeyRelease(key)).ok();
                thread::sleep(Duration::from_millis(10));
                log::debug!("Typed char '{}'", ch);
            } else {
                log::warn!("No key mapping for char '{}'", ch);
            }
        }

        log::info!("✅ Test complete.");
    }
    */

}

fn char_to_key(c: char) -> Option<Key> {
    match c {
        'a' | 'A' => Some(Key::KeyA),
        'b' | 'B' => Some(Key::KeyB),
        'c' | 'C' => Some(Key::KeyC),
        'd' | 'D' => Some(Key::KeyD),
        'e' | 'E' => Some(Key::KeyE),
        'f' | 'F' => Some(Key::KeyF),
        'g' | 'G' => Some(Key::KeyG),
        'h' | 'H' => Some(Key::KeyH),
        'i' | 'I' => Some(Key::KeyI),
        'j' | 'J' => Some(Key::KeyJ),
        'k' | 'K' => Some(Key::KeyK),
        'l' | 'L' => Some(Key::KeyL),
        'm' | 'M' => Some(Key::KeyM),
        'n' | 'N' => Some(Key::KeyN),
        'o' | 'O' => Some(Key::KeyO),
        'p' | 'P' => Some(Key::KeyP),
        'q' | 'Q' => Some(Key::KeyQ),
        'r' | 'R' => Some(Key::KeyR),
        's' | 'S' => Some(Key::KeyS),
        't' | 'T' => Some(Key::KeyT),
        'u' | 'U' => Some(Key::KeyU),
        'v' | 'V' => Some(Key::KeyV),
        'w' | 'W' => Some(Key::KeyW),
        'x' | 'X' => Some(Key::KeyX),
        'y' | 'Y' => Some(Key::KeyY),
        'z' | 'Z' => Some(Key::KeyZ),
        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),
        ' ' => Some(Key::Space),
        '-' => Some(Key::Minus),
        '.' => Some(Key::Dot),
        '/' => Some(Key::Slash),
        _ => None,
    }
}


fn key_to_char(key: Key, shift: bool) -> Option<char> {
    Some(match key {
        KeyA => if shift { 'A' } else { 'a' },
        KeyB => if shift { 'B' } else { 'b' },
        KeyC => if shift { 'C' } else { 'c' },
        KeyD => if shift { 'D' } else { 'd' },
        KeyE => if shift { 'E' } else { 'e' },
        KeyF => if shift { 'F' } else { 'f' },
        KeyG => if shift { 'G' } else { 'g' },
        KeyH => if shift { 'H' } else { 'h' },
        KeyI => if shift { 'I' } else { 'i' },
        KeyJ => if shift { 'J' } else { 'j' },
        KeyK => if shift { 'K' } else { 'k' },
        KeyL => if shift { 'L' } else { 'l' },
        KeyM => if shift { 'M' } else { 'm' },
        KeyN => if shift { 'N' } else { 'n' },
        KeyO => if shift { 'O' } else { 'o' },
        KeyP => if shift { 'P' } else { 'p' },
        KeyQ => if shift { 'Q' } else { 'q' },
        KeyR => if shift { 'R' } else { 'r' },
        KeyS => if shift { 'S' } else { 's' },
        KeyT => if shift { 'T' } else { 't' },
        KeyU => if shift { 'U' } else { 'u' },
        KeyV => if shift { 'V' } else { 'v' },
        KeyW => if shift { 'W' } else { 'w' },
        KeyX => if shift { 'X' } else { 'x' },
        KeyY => if shift { 'Y' } else { 'y' },
        KeyZ => if shift { 'Z' } else { 'z' },
        Space => ' ',
        Return => '\n',
        Tab => '\t',
        Num1 => if shift { '!' } else { '1' },
        Num2 => if shift { '@' } else { '2' },
        Num3 => if shift { '#' } else { '3' },
        Num4 => if shift { '$' } else { '4' },
        Num5 => if shift { '%' } else { '5' },
        Num6 => if shift { '^' } else { '6' },
        Num7 => if shift { '&' } else { '7' },
        Num8 => if shift { '*' } else { '8' },
        Num9 => if shift { '(' } else { '9' },
        Num0 => if shift { ')' } else { '0' },
        Minus => if shift { '_' } else { '-' },
        Equal => if shift { '+' } else { '=' },
        LeftBracket => if shift { '{' } else { '[' },
        RightBracket => if shift { '}' } else { ']' },
        BackSlash => if shift { '|' } else { '\\' },
        SemiColon => if shift { ':' } else { ';' },
        Quote => if shift { '"' } else { '\'' },
        Comma => if shift { '<' } else { ',' },
        Dot => if shift { '>' } else { '.' },
        Slash => if shift { '?' } else { '/' },
        _ => return None,
    })
}