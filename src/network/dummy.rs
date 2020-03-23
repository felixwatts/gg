use crate::network::RxChannel;
use crate::network::TxChannel;
use ggez::GameResult;

#[derive(Clone)]
pub enum NoMsg{
}

pub struct DummyChannel{}

impl<TMsg> TxChannel<TMsg> for DummyChannel {
    fn enqueue(&mut self, _: TMsg) -> GameResult{
        panic!("cannot enqueue on DummyChannel");
    }
}

impl<TMsg> RxChannel<TMsg> for DummyChannel {
    fn dequeue(&mut self, _: &mut Vec::<TMsg>) -> GameResult{
        panic!("cannot dequeue from DummyChannel");
    }
}