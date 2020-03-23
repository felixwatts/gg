use ggez::GameResult;
use std::marker::PhantomData;
use crate::network::real::NetworkChannel;
use crate::network::ClientMsg;
use crate::network::ServerMsg;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct ServerSetup{
    engine: Engine<ServerMsg, ClientMsg>,
    network: NetworkChannel<ServerMsg, ClientMsg>
}

impl ServerSetup{
    pub fn new(context: &mut ggez::Context) -> ServerSetup{
        ServerSetup{
            engine: crate::engine::new_server(context).unwrap(),
            network: NetworkChannel::<ServerMsg, ClientMsg>{
                phantom1: PhantomData::<ServerMsg>{},
                phantom2: PhantomData::<ClientMsg>{},
            }
        }
    }
}

impl EventHandler for ServerSetup {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.engine.update(context, &mut self.network)?;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult{
        Ok(())
    }
}