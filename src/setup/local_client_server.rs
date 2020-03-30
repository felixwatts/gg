use crate::system::system::System;
use crate::err::GgResult;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;
use crate::network::sim::SimServerContainer;

pub struct LocalClientServerSetup{
    client_engine: Option<Engine>,
    server_engine: Option<Engine>,
    network: SimServerContainer,
}

impl LocalClientServerSetup {
    pub fn new(context: &mut ggez::Context, latency: u32) -> GgResult<LocalClientServerSetup>{

        let mut result = LocalClientServerSetup{
            client_engine: None,
            server_engine: None,
            network: SimServerContainer::new()
        };

        let mut server = result.network.get_server(latency);
        let client = server.connect();

        let server_systems: Vec<Box<dyn System>> = vec![
            Box::new(crate::system::server::ServerSystem::new(server)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_local: false}),
        ];
        let server_engine = Engine::new(server_systems, context)?;

        let client_systems: Vec<Box<dyn System>> = vec![
            Box::new(crate::system::client::ClientSystem::new(client)),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];
        let client_engine = Engine::new(client_systems, context)?;

        result.client_engine = Some(client_engine);
        result.server_engine = Some(server_engine);

        Ok(result)
    }
}

impl EventHandler for LocalClientServerSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {

        self.network.step();

        self.client_engine.as_mut().unwrap().update(context)?;
        self.server_engine.as_mut().unwrap().update(context)?; 

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {

        self.client_engine.as_mut().unwrap().draw(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.client_engine.as_mut().unwrap().key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.client_engine.as_mut().unwrap().key_up_event(context, keycode, keymod);
    }
}