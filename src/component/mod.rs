mod planar_body;
mod radial_body;

pub mod body;

use nalgebra::Vector2;
use recs::EntityId;

pub struct Dead;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

#[derive(Clone)]
pub struct Sprite{
    pub color: [f32; 4],
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>
}

#[derive(Clone)]
pub struct Focus;

#[derive(Clone)]
pub struct Gorilla{
    pub button_state: [bool; 2]
}

pub struct Anchor;

pub struct TxQueue<TMsg>(pub Vec::<TMsg>);

#[derive(Clone)]
pub struct RxQueue<TMsg>(pub Vec::<TMsg>);

pub struct Network;