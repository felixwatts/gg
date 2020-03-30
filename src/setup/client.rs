use crate::err::GgResult;
use crate::system::system::System;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;
use std::net::TcpStream;

pub struct ClientSetup {
    engine: Engine
}

impl ClientSetup {
    pub fn new(context: &mut ggez::Context) -> GgResult<ClientSetup> {
        let tcp_stream = TcpStream::connect("127.0.0.1:9001")?;
        let network = crate::network::real::RealNetwork::new(tcp_stream)?;
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(crate::system::client::ClientSystem::new(network)),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];
        let engine = Engine::new(systems, context)?;

        Ok(ClientSetup{
            engine
        })
    }
}

impl EventHandler for ClientSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.engine.update(context)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult{
        self.engine.draw(context)?;
        Ok(())
    }
}