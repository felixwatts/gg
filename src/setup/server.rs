use crate::system::system::System;
use crate::err::GgResult;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct ServerSetup{
    engine: Engine,
}

impl ServerSetup{
    pub fn new(context: &mut ggez::Context) -> GgResult<ServerSetup> {
        let server = crate::network::real::RealServer::new()?;
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(crate::system::server::ServerSystem::new(server)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{}),
        ];
        let engine = Engine::new(systems, context)?;
        Ok(ServerSetup{
            engine
        })
    }
}

impl EventHandler for ServerSetup {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.engine.update(context)?;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> ggez::GameResult{
        Ok(())
    }
}