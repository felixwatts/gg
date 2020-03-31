use crate::network::real::RealNetwork;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::err::GgResult;
use crate::system::system::System;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;
use std::net::TcpStream;
use crate::system::client::ClientSystem;

pub struct ClientSetup {
    engine: Engine<ggez::Context>
}

impl ClientSetup {
    pub fn new(context: &mut ggez::Context) -> GgResult<ClientSetup> {
        let tcp_stream = TcpStream::connect("127.0.0.1:9001")?;
        let network = RealNetwork::new(tcp_stream)?;
        let systems: Vec<Box<dyn System<ggez::Context>>> = vec![
            Box::new(ClientSystem::new(network)),
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

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.engine.key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.engine.key_up_event(context, keycode, keymod);
    }
}