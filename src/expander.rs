use std::collections::{HashMap, VecDeque};
use std::{thread, time::Duration};

use rdev::{simulate, EventType, Key};

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

    pub fn key_pressed(&mut self, key: Key) {
        // Step 1: Handle Tab press
        if key == Key::Tab {
            let typed: String = self.buffer.iter().collect();
            for (shortcut, expansion) in &self.shortcuts {
                if typed.ends_with(shortcut) {
                    log::info!("🚀 Shortcut matched with TAB: '{}'", shortcut);
                    log::info!("📝 Expansion: '{}'", expansion);

                    // Delete shortcut + tab
                    for _ in 0..(shortcut.len() + 1) {
                        simulate(&EventType::KeyPress(Key::Backspace)).ok();
                        thread::sleep(Duration::from_millis(5));
                        simulate(&EventType::KeyRelease(Key::Backspace)).ok();
                        
                    }

                    // Type the expansion
                    for ch in expansion.chars() {
                        if let Some(k) = char_to_key(ch) {
                            simulate(&EventType::KeyPress(k)).ok();
                            thread::sleep(Duration::from_millis(5));
                            simulate(&EventType::KeyRelease(k)).ok();
                            
                        }
                    }

                    self.buffer.clear();
                    return;
                }
            }

            // No match: just type a regular tab
            //simulate(&EventType::KeyPress(Key::Tab)).ok();
            //thread::sleep(Duration::from_millis(10)); // <== add this delay
            //simulate(&EventType::KeyRelease(Key::Tab)).ok();
            //thread::sleep(Duration::from_millis(5));
            // No shortcut match — just let the user's real Tab happen (do nothing)
            self.buffer.clear(); // Clear buffer anyway to avoid false positives

            return;
        }

        // Step 2: Build the typed buffer (only for printable keys)
        if let Some(c) = key_to_char(key) {
            if self.buffer.len() == self.max_shortcut_length {
                self.buffer.pop_front();
            }
            self.buffer.push_back(c);

            //let typed: String = self.buffer.iter().collect();
            //println!("Typed buffer: '{}'", typed); // Optional debug
        }
    }

    pub fn set_shortcuts(&mut self, shortcuts: HashMap<String, String>) {
        self.shortcuts = shortcuts;
    }
    
}

fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::KeyA => Some('a'), Key::KeyB => Some('b'), Key::KeyC => Some('c'),
        Key::KeyD => Some('d'), Key::KeyE => Some('e'), Key::KeyF => Some('f'),
        Key::KeyG => Some('g'), Key::KeyH => Some('h'), Key::KeyI => Some('i'),
        Key::KeyJ => Some('j'), Key::KeyK => Some('k'), Key::KeyL => Some('l'),
        Key::KeyM => Some('m'), Key::KeyN => Some('n'), Key::KeyO => Some('o'),
        Key::KeyP => Some('p'), Key::KeyQ => Some('q'), Key::KeyR => Some('r'),
        Key::KeyS => Some('s'), Key::KeyT => Some('t'), Key::KeyU => Some('u'),
        Key::KeyV => Some('v'), Key::KeyW => Some('w'), Key::KeyX => Some('x'),
        Key::KeyY => Some('y'), Key::KeyZ => Some('z'),
        Key::Num0 => Some('0'), Key::Num1 => Some('1'), Key::Num2 => Some('2'),
        Key::Num3 => Some('3'), Key::Num4 => Some('4'), Key::Num5 => Some('5'),
        Key::Num6 => Some('6'), Key::Num7 => Some('7'), Key::Num8 => Some('8'),
        Key::Num9 => Some('9'),
        Key::Space => Some(' '),
        Key::Minus => Some('-'),
        Key::Slash => Some('/'),
        Key::Dot => Some('.'),
        Key::Comma => Some(','),
        Key::BackQuote => Some('`'),
        _ => None,
    }
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
