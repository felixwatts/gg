mod planar_body;
mod radial_body;

pub mod body;
pub mod gorilla;

use crate::input::KeyMapping;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::ServerMsg;
use crate::network::TxChannel;
use nalgebra::Vector2;
use recs::EntityId;
use serde::{Serialize, Deserialize};

pub struct Dead;

pub struct Client<TNetwork>(pub TNetwork) where TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg>;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Sprite{
    pub color: [f32; 4],
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>
}

#[derive(Clone)]
pub struct Focus;

pub struct Anchor;

pub struct Network;

pub struct Keyboard(pub KeyMapping);