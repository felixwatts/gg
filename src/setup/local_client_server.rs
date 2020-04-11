use crate::system::system::System;
use crate::err::GgResult;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;
use std::rc::Rc;
use std::cell::Cell;
use std::time::Duration;

pub struct LocalClientServerSetup{
    client_engine: Option<Engine<ggez::Context>>,
    server_engine: Option<Engine<ggez::Context>>,
    network_time: Rc::<Cell::<Duration>>,
}

impl LocalClientServerSetup {
    pub fn new(context: &mut ggez::Context, network_latency: Duration, is_latency_compensation_enabled: bool) -> GgResult<LocalClientServerSetup>{

        let mut result = LocalClientServerSetup{
            client_engine: None,
            server_engine: None,
            network_time: Rc::new(Cell::new(Duration::from_millis(0)))
        };

        let mut server = crate::network::sim::SimServer::new(network_latency, Rc::clone(&result.network_time));// result.network.get_server(latency);
        let client = server.connect();

        let server_systems: Vec<Box<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::server::ServerSystem::new(server, is_latency_compensation_enabled)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_latency_compensation_enabled})
        ];
        let server_engine = Engine::new(server_systems, None, context)?;

        let client_systems: Vec<Box<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::client::ClientSystem::new(client)),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];
        let client_engine = Engine::new(client_systems, None, context)?;

        result.client_engine = Some(client_engine);
        result.server_engine = Some(server_engine);

        Ok(result)
    }
}

impl EventHandler for LocalClientServerSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {

        self.network_time.set(ggez::timer::time_since_start(context));

        // self.network.step();

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