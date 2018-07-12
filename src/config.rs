extern crate json;
extern crate sdl2;

use sdl2::keyboard::*;
use std::fs;

pub struct KeyBindings {
    pub up: Keycode,
    pub down: Keycode,
    pub left: Keycode,
    pub right: Keycode,
    pub undo: Keycode,
    pub quit: Keycode,
}

fn map_keys(entry: &str) -> Option<Keycode> {
    if entry == "arrow-up" {
        Some(Keycode::Up)
    } else if entry == "arrow-down" {
        Some(Keycode::Down)
    } else if entry == "arrow-left" {
        Some(Keycode::Left)
    } else if entry == "arrow-right" {
        Some(Keycode::Right)
    } else if entry == "backspace" {
        Some(Keycode::Backspace)
    } else {
        let ch = {
            match entry.chars().next() {
                Some(c) => {
                    match c.to_lowercase().next() {
                        Some(d) => d,
                        None => return None
                    }
                },
                None => return None
            }
        };
        match ch {
            'a' => Some(Keycode::A),
            'b' => Some(Keycode::B),
            'c' => Some(Keycode::C),
            'd' => Some(Keycode::D),
            'e' => Some(Keycode::E),
            'f' => Some(Keycode::F),
            'g' => Some(Keycode::G),
            'h' => Some(Keycode::H),
            'i' => Some(Keycode::I),
            'j' => Some(Keycode::J),
            'k' => Some(Keycode::K),
            'l' => Some(Keycode::L),
            'm' => Some(Keycode::M),
            'n' => Some(Keycode::N),
            'o' => Some(Keycode::O),
            'p' => Some(Keycode::P),
            'q' => Some(Keycode::Q),
            'r' => Some(Keycode::R),
            't' => Some(Keycode::T),
            's' => Some(Keycode::S),
            'u' => Some(Keycode::U),
            'v' => Some(Keycode::V),
            'w' => Some(Keycode::W),
            'x' => Some(Keycode::X),
            'y' => Some(Keycode::Y),
            'z' => Some(Keycode::Z),
            _ => None
        }
    }
}

impl KeyBindings {
    pub fn new(config: &json::JsonValue, path: &str) -> KeyBindings {

        /* Default bindings */
        let mut kb = KeyBindings {
            up: Keycode::Up,
            down: Keycode::Down,
            left: Keycode::Left,
            right: Keycode::Right,
            undo: Keycode::Backspace,
            quit: Keycode::Q
        };

        let keys = &config["key-bindings"];
        if !keys.is_object() {
            eprintln!("Invalid 'key-bindings' configuration in {}", path);
            return kb;
        }

        let up = &keys["up"];
        let down = &keys["down"];
        let left = &keys["left"];
        let right = &keys["right"];
        let undo = &keys["undo"];
        let quit = &keys["quit"];

        if up.is_string() {
            match map_keys(up.as_str().unwrap()) {
                Some (k) => kb.up = k,
                None => eprintln!("W: unknown key binding for 'up'")
            }
        }
        if down.is_string() {
            match map_keys(down.as_str().unwrap()) {
                Some (k) => kb.down = k,
                None => eprintln!("W: unknown key binding for 'down'")
            }
        }
        if left.is_string() {
            match map_keys(left.as_str().unwrap()) {
                Some (k) => kb.left = k,
                None => eprintln!("W: unknown key binding for 'left'")
            }
        }
        if right.is_string() {
            match map_keys(right.as_str().unwrap()) {
                Some (k) => kb.right = k,
                None => eprintln!("W: unknown key binding for 'right'")
            }
        }
        if undo.is_string() {
            match map_keys(undo.as_str().unwrap()) {
                Some (k) => kb.undo = k,
                None => eprintln!("W: unknown key binding for 'undo'")
            }
        }
        if quit.is_string() {
            match map_keys(quit.as_str().unwrap()) {
                Some (k) => kb.quit = k,
                None => eprintln!("W: unknown key binding for 'quit'")
            }
        }
        kb
    }
}

pub fn new(path: &str) -> Result<(KeyBindings, usize), String> {
        let data = {
            match fs::read_to_string(path)  {
                Ok(d) => d,
                Err(e) =>
                    return Err(format!("Can't read configuration file '{}': {}\n",
                                       path, e))
            }
        };
        let config = {
            match json::parse(&data) {
                Ok(d) => d,
                Err(e) =>
                    return Err(format!("Can't parse configuration file '{}': {}\n",
                                       path, e))
            }
        };
        let undo = &config["undo-level"];
        if !undo.is_number() {
            return Err(format!("Invalid 'undo-level' entry\n"));
        }
        let kb = KeyBindings::new(&config, path);
        Ok((kb, undo.as_usize().unwrap()))
}
