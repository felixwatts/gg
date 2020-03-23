use crate::network::SimNetwork;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::engine::Engine;

pub struct LocalClientServerSetup{
    client_engine: Engine<ClientMsg, ServerMsg>,
    server_engine: Engine<ServerMsg, ClientMsg>,
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
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        self.network.step();

        self.client_engine.update(context, &mut self.network)?;
        self.server_engine.update(context, &mut self.network)?; 

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {

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
        self.client_engine.key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.client_engine.key_up_event(context, keycode, keymod);
    }
}