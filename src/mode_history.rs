use crate::constants;
use crate::mode::Mode;
use std::vec::Vec;

pub struct ModeHistory {
    history: Vec<Mode>,
}

impl ModeHistory {
    pub fn new() -> Self {
        ModeHistory {
            history: Vec::<Mode>::with_capacity(constants::DEFAULT_HISTORY_CAPACITY),
        }
    }

    pub fn register(&mut self, mode: Mode) {
        self.history.push(mode);
    }

    // Constumes history and try to return top mode
    pub fn last_consume(&mut self) -> Option<Mode> {
        self.history.truncate(1);
        self.history.pop()
    }

    pub fn prev_consume(&mut self) -> Option<Mode> {
        // first pop current
        self.history.pop()?;
        self.history.pop()
    }
}
