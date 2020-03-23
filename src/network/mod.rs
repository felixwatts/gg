pub mod dummy;
pub mod sim;
pub mod real;

use crate::component::Sprite;
use crate::component::body::Body;
use recs::EntityId;
use crate::component::TxQueue;
use crate::state::State;
use ggez::GameResult;

pub fn tx<TMsg>(state: &mut State, msg: TMsg) where TMsg: 'static {
    state.ecs.borrow_mut::<TxQueue<TMsg>>(state.tx_queue.unwrap()).unwrap().0.push(msg);
}

pub trait TxChannel<TMsg>{
    fn enqueue(&mut self, msg: TMsg) -> GameResult;
}

pub trait RxChannel<TMsg>{
    fn dequeue(&mut self, buffer: &mut Vec::<TMsg>) -> GameResult;
}

#[derive(Clone)]
pub enum ServerMsg{
    Kill(EntityId),
    SetBody(EntityId, Body),
    SetSprite(EntityId, Sprite),
    SetFocus(EntityId)
}

#[derive(Clone)]
pub enum ClientMsg{
    ButtonStateChange([bool; 2])
}

