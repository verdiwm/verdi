use colpetto::event::KeyState;
use input_linux_sys::{
    KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6, KEY_F7, KEY_F8, KEY_F9, KEY_LEFTALT,
    KEY_LEFTCTRL, KEY_RIGHTALT, KEY_RIGHTCTRL,
};
use std::collections::{HashMap, HashSet};

/// Maps function keys to VT numbers
pub struct KeyMap {
    mappings: HashMap<u32, u32>,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyMap {
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        // Function keys mapped to respective VTs
        mappings.insert(KEY_F1 as u32, 1);
        mappings.insert(KEY_F2 as u32, 2);
        mappings.insert(KEY_F3 as u32, 3);
        mappings.insert(KEY_F4 as u32, 4);
        mappings.insert(KEY_F5 as u32, 5);
        mappings.insert(KEY_F6 as u32, 6);
        mappings.insert(KEY_F7 as u32, 7);
        mappings.insert(KEY_F8 as u32, 8);
        mappings.insert(KEY_F9 as u32, 9);

        Self { mappings }
    }

    pub fn get_vt(&self, key: u32) -> Option<u32> {
        self.mappings.get(&key).copied()
    }
}

pub struct ModifierState {
    pressed_keys: HashSet<u32>,
}

impl Default for ModifierState {
    fn default() -> Self {
        Self::new()
    }
}

impl ModifierState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, key: u32, state: KeyState) {
        match state {
            KeyState::Pressed => {
                self.pressed_keys.insert(key);
            }
            KeyState::Released => {
                self.pressed_keys.remove(&key);
            }
        }
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.pressed_keys.contains(&(KEY_LEFTCTRL as u32))
            || self.pressed_keys.contains(&(KEY_RIGHTCTRL as u32))
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.pressed_keys.contains(&(KEY_LEFTALT as u32))
            || self.pressed_keys.contains(&(KEY_RIGHTALT as u32))
    }

    pub fn is_ctrl_alt_pressed(&self) -> bool {
        self.is_ctrl_pressed() && self.is_alt_pressed()
    }
}
