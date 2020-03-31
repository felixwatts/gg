use crate::context::server::ServerContext;
use crate::system::system::System;
use crate::err::GgResult;
use crate::engine::Engine;

pub struct ServerSetup{
    engine: Engine<ServerContext>,
    context: ServerContext
}

impl ServerSetup{
    pub fn new(mut context: ServerContext) -> GgResult<ServerSetup> {
        let server = crate::network::real::RealServer::new()?;
        let systems: Vec<Box<dyn System<ServerContext>>> = vec![
            Box::new(crate::system::server::ServerSystem::new(server)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_local: false}),
        ];
        let engine = Engine::new(systems, &mut context)?;
        Ok(ServerSetup{
            engine,
            context
        })
    }

    pub fn step(&mut self) -> ggez::GameResult {
        self.context.step();
        self.engine.update(&mut self.context)?;
        Ok(())
    }
}