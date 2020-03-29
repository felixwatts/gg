use crate::network::sim::SimNetwork;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;
use ggez::GameResult;

pub struct LocalClientServerSetup{
    client_engine: Engine<SimNetwork>,
    server_engine: Engine<SimNetwork>,
    network: SimNetwork,
}

impl LocalClientServerSetup{
    pub fn new(context: &mut ggez::Context, latency: u32) -> LocalClientServerSetup{

        let client_engine = crate::engine::new_client(context).unwrap();
        let server_engine = crate::engine::new_server(context).unwrap();

        LocalClientServerSetup{
            client_engine: client_engine,
            server_engine: server_engine,
            network: SimNetwork::new(latency)
        }
    }
}

impl EventHandler for LocalClientServerSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {

        self.network.step();

        self.client_engine.update(context, &mut self.network)?;
        self.server_engine.update(context, &mut self.network)?; 

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {

        self.client_engine.draw(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.client_engine.key_down_event(context, &mut self.network, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.client_engine.key_up_event(context, &mut self.network, keycode, keymod);
    }
}