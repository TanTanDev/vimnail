use crate::action_type::ActionType;
use crate::input_action::{InputAction, InputActions};
use crate::mode::Mode;

use ggez::event::KeyCode;
use std::collections::HashMap;

pub struct ActionProcesser {
    mode_inputactions: HashMap<Mode, InputActions>,
}

impl ActionProcesser {
    pub fn get_input_actions(&mut self, mode: Mode) -> Option<&InputActions> {
        self.mode_inputactions.get(&mode)
    }

    pub fn process_input(&self, mode: Mode, key_code: KeyCode) -> Option<ActionType> {
        let inputactions = self.mode_inputactions.get(&mode)?;
        let maybe_actiontype = inputactions
            .iter()
            .find(|ia| ia.key_code == key_code)
            .map(|ia| ia.action_type);

        if let Some(action_type) = maybe_actiontype {
            return Some(action_type);
        }

        let any_inputactions = self.mode_inputactions.get(&Mode::Any)?;
        any_inputactions
            .iter()
            .find(|ia| ia.key_code == key_code)
            .map(|ia| ia.action_type)
    }
}

pub struct ActionProcesserBuilder {
    mode_inputactions: HashMap<Mode, InputActions>,
}

impl ActionProcesserBuilder {
    pub fn new() -> Self {
        ActionProcesserBuilder {
            mode_inputactions: HashMap::new(),
        }
    }

    pub fn with_mode(&mut self, mode: Mode) -> &mut Self {
        self.mode_inputactions.insert(mode, InputActions::new());
        self
    }

    pub fn with_inputaction(&mut self, mode: Mode, inputaction: InputAction) -> &mut Self {
        if !self.mode_inputactions.contains_key(&mode) {
            self.with_mode(mode);
        }
        let maybe_inputactions = self.mode_inputactions.get_mut(&mode);
        if let Some(inputactions) = maybe_inputactions {
            inputactions.push(inputaction);
        } else {
            // this should not happend due to above check, but maybe handle this error more elegantly?
            println!("Failed to add input action, inputacitons not initialized");
        }
        self
    }

    pub fn build(&self) -> ActionProcesser {
        ActionProcesser {
            mode_inputactions: self.mode_inputactions.clone(),
        }
    }
}
