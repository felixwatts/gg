use std::marker::PhantomData;
use crate::network::real::NetworkChannel;
use crate::network::ClientMsg;
use crate::network::ServerMsg;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;

pub struct ClientSetup{
    engine: Engine<NetworkChannel<ClientMsg, ServerMsg>>,
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
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.engine.update(context, &mut self.network)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult{
        self.engine.draw(context)?;
        Ok(())
    }
}