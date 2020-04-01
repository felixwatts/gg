use crate::input::InputEvent;

pub struct Gorilla{
    pub input_events: Vec::<InputEvent>
}

impl Gorilla{
    pub fn new() -> Gorilla{
        Gorilla{
            input_events: vec![]
        }
    }
}