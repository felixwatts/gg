use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ggez::input::keyboard::KeyCode;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Button {
    One,
    Two
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct InputEvent{
    pub button: Button,
    pub is_down: bool
}

pub type KeyMapping = HashMap<KeyCode, Button>;

pub fn default_key_mapping() -> KeyMapping{
    [
        (KeyCode::Space, Button::One),
        (KeyCode::Return, Button::Two)
    ].iter().cloned().collect()
}
