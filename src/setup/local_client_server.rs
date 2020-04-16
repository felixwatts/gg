use crate::input::KeyMapping;
use crate::system::System;
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
use crate::network::sim::SimServer;

pub struct LocalClientServerSetup{
    client_1_engine: Option<Engine<ggez::Context>>,
    client_2_engine: Option<Engine<ggez::Context>>,
    server_engine: Option<Engine<ggez::Context>>,
    network_time: Rc::<Cell::<Duration>>,
}

impl LocalClientServerSetup {
    pub fn new(context: &mut ggez::Context, network_latency: Duration, is_latency_compensation_enabled: bool) -> GgResult<LocalClientServerSetup>{

        let mut result = LocalClientServerSetup{
            client_1_engine: None,
            client_2_engine: None,
            server_engine: None,
            network_time: Rc::new(Cell::new(Duration::from_millis(0)))
        };

        let mut server = crate::network::sim::SimServer::new(network_latency, Rc::clone(&result.network_time));

        result.client_1_engine = Some(LocalClientServerSetup::build_client(crate::input::p1_key_mapping(), &mut server, context)?);
        result.client_2_engine = Some(LocalClientServerSetup::build_client(crate::input::p2_key_mapping(), &mut server, context)?);

        let server_systems: Vec<Box<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::server::ServerSystem::new(server, is_latency_compensation_enabled)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_latency_compensation_enabled}),
            Box::new(crate::system::game::tag::TagGameSystem::new())
        ];
        result.server_engine = Some(Engine::new(server_systems, None, context)?);

        Ok(result)
    }

    fn build_client(key_mapping: KeyMapping, server: &mut SimServer, context: &mut ggez::Context) -> GgResult::<Engine::<ggez::Context>> {
        let client = server.connect();
        let client_systems: Vec<Box<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::client::ClientSystem::new(client, key_mapping)),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];
        let client_engine = Engine::new(client_systems, None, context)?;

        Ok(client_engine)
    }
}

impl EventHandler for LocalClientServerSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.network_time.set(ggez::timer::time_since_start(context));
        self.client_1_engine.as_mut().unwrap().update(context)?;
        self.client_2_engine.as_mut().unwrap().update(context)?;
        self.server_engine.as_mut().unwrap().update(context)?; 

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {

        self.client_1_engine.as_mut().unwrap().draw(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.client_1_engine.as_mut().unwrap().key_down_event(context, keycode, keymod, repeat);
        self.client_2_engine.as_mut().unwrap().key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.client_1_engine.as_mut().unwrap().key_up_event(context, keycode, keymod);
        self.client_2_engine.as_mut().unwrap().key_up_event(context, keycode, keymod);
    }
}