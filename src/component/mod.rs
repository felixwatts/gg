mod planar_body;
mod radial_body;

pub mod body;
pub mod gorilla;
pub mod sprite;

use crate::input::KeyMapping;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::ServerMsg;
use crate::network::TxChannel;
use recs::EntityId;

pub struct Dead;

pub struct Client<TNetwork>(pub TNetwork) where TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg>;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

#[derive(Clone)]
pub struct Focus;

pub struct Anchor;

pub struct Network;

pub struct Keyboard(pub KeyMapping);