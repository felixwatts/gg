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

pub fn new_key_mapping(btn1: KeyCode, btn2: KeyCode) -> KeyMapping {
    [
        (btn1, Button::One),
        (btn2, Button::Two)
    ].iter().cloned().collect()
}

pub fn default_key_mapping() -> KeyMapping{
    [
        (KeyCode::Space, Button::One),
        (KeyCode::Return, Button::Two)
    ].iter().cloned().collect()
}

pub fn p1_key_mapping() -> KeyMapping{
    [
        (KeyCode::Z, Button::One),
        (KeyCode::X, Button::Two)
    ].iter().cloned().collect()
}

pub fn p2_key_mapping() -> KeyMapping{
    [
        (KeyCode::Left, Button::One),
        (KeyCode::Right, Button::Two)
    ].iter().cloned().collect()
}
