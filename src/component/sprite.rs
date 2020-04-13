use serde::{Serialize, Deserialize};
use nalgebra::Vector2;
use crate::colors::Color;

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Sprite{
    pub color: Color,
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>,
    pub src_loc: Vector2<f32>,
    pub src_size: Vector2<f32>
}

impl Sprite{
    pub fn new(color: Color, size: Vector2<f32>, src_loc: Vector2<f32>, src_size: Vector2<f32>) -> Sprite {
        Sprite{
            color,
            location: [0.0, 0.0].into(),
            orientation: 0.0,
            size,
            src_loc,
            src_size
        }
    }
}