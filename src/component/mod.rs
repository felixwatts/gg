mod planar_body;
mod radial_body;

pub mod body;

use nalgebra::Vector2;
use recs::EntityId;

pub struct Dead;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

pub struct Sprite{
    pub color: [f32; 4],
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>
}

pub struct Focus;

pub struct Gorilla;

pub struct Anchor;