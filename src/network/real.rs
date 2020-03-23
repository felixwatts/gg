use crate::network::RxChannel;
use crate::network::TxChannel;
use std::marker::PhantomData;
use ggez::GameResult;

pub struct NetworkChannel<TTx, TRx>{
    pub phantom1: PhantomData<TTx>,
    pub phantom2: PhantomData<TRx>,
}

impl<TTx, TRx> TxChannel<TTx> for NetworkChannel<TTx, TRx> {
    fn enqueue(&mut self, _: TTx) -> GameResult{
        unimplemented!();
    }
}

impl<TTx, TRx> RxChannel<TRx> for NetworkChannel<TTx, TRx> {
    fn dequeue(&mut self, _: &mut Vec::<TRx>) -> GameResult{
        unimplemented!();
    }
}