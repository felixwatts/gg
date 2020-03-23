use ggez::GameResult;
use std::marker::PhantomData;
use crate::network::NetworkChannel;
use crate::network::ClientMsg;
use crate::network::ServerMsg;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct ClientSetup{
    engine: Engine<ClientMsg, ServerMsg>,
    network: NetworkChannel<ClientMsg, ServerMsg>
}

impl ClientSetup{
    pub fn new(context: &mut ggez::Context) -> ClientSetup{
        ClientSetup{
            engine: crate::engine::new_client(context).unwrap(),
            network: NetworkChannel::<ClientMsg, ServerMsg>{
                phantom1: PhantomData::<ClientMsg>{},
                phantom2: PhantomData::<ServerMsg>{},
            }
        }
    }
}

impl EventHandler for ClientSetup {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.engine.update(context, &mut self.network)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult{
        self.engine.draw(context)?;
        Ok(())
    }
}