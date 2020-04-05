mod planar_body;
mod radial_body;

pub mod body;
pub mod gorilla;
pub mod sprite;
pub mod client;

use crate::input::KeyMapping;
use recs::EntityId;

pub struct Dead;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

#[derive(Clone)]
pub struct Focus;

pub struct Anchor;

pub struct Network;

pub struct Keyboard(pub KeyMapping);