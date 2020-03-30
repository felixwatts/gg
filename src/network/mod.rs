pub mod sim;
pub mod real;

use crate::component::Sprite;
use crate::component::body::Body;
use crate::err::GgResult;
use serde::Serialize;
use serde::Deserialize;

pub trait Server<TNetwork>
    where TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg> {
    fn get_new_clients(&mut self, buffer: &mut Vec<TNetwork>);
}

pub trait TxChannel<TMsg>{
    fn enqueue(&mut self, msg: TMsg) -> GgResult;
}

pub trait RxChannel<TMsg>{
    fn dequeue(&mut self, buffer: &mut Vec::<TMsg>) -> GgResult;
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub enum ServerMsg{
    Kill(u64),
    SetBody(u64, Body),
    SetSprite(u64, Sprite),
    SetFocus(u64)
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
pub enum ClientMsg{
    ButtonStateChange([bool; 2])
}

