use nalgebra::Vector2;
use crate::input::InputEvent;

pub struct Gorilla{
    pub input_events: Vec::<InputEvent>,
    pub spawn_location: Vector2::<f32>
}

impl Gorilla{
    pub fn new(spawn_location: Vector2::<f32>) -> Gorilla{
        Gorilla{
            input_events: vec![],
            spawn_location
        }
    }
}