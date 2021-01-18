use std::char;
use std::string::String;

#[derive(Clone)]
pub struct TextInput {
    pub content: String,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            content: String::from(""),
        }
    }

    pub fn clear(&mut self) {
        self.content = String::from("");
    }

    pub fn add(&mut self, character: char) {
        self.content.push(character);
    }

    pub fn del(&mut self) {
        self.content.pop();
    }
}
