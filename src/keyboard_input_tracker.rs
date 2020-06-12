use crate::key_state::KeyState;

use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;
pub struct KeyboardInputTracker {
    pub keys: HashMap<KeyCode, KeyState>,
}

impl KeyboardInputTracker {
    pub fn new() -> Self {
        KeyboardInputTracker {
            keys: HashMap::with_capacity(256),
        }
    }

    pub fn update_key(&mut self, key_code: KeyCode, key_state: KeyState) {
        self.keys.insert(key_code, key_state);
    }

    pub fn update(&mut self) {
        for (_key_code, key_state) in self
            .keys
            .iter_mut()
            .filter(|(_key_code, key_state)| **key_state == KeyState::Pressed)
        {
            *key_state = KeyState::Down;
        }
    }
}
