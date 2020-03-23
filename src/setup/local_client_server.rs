use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::network::SimChannel;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::engine::Engine;

pub struct LocalClientServer{
    client_engine: Engine<ClientMsg, ServerMsg>,
    server_engine: Engine<ServerMsg, ClientMsg>,
    client_server_channel: SimChannel<ClientMsg>,
    server_client_channel: SimChannel<ServerMsg>,
}

impl LocalClientServer{
    pub fn new(context: &mut ggez::Context, latency: u32) -> LocalClientServer{

        let client_server_channel = SimChannel::<ClientMsg>::new(latency);
        let server_client_channel = SimChannel::<ServerMsg>::new(latency);

        let client_engine = crate::engine::new_client(context).unwrap();
        let server_engine = crate::engine::new_server(context).unwrap();

        LocalClientServer{
            client_engine: client_engine,
            server_engine: server_engine,
            client_server_channel: client_server_channel,
            server_client_channel: server_client_channel
        }
    }

    pub fn step(&mut self) {
        self.client_server_channel.step();
        self.server_client_channel.step();
    }
}

impl EventHandler for LocalClientServer {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        self.step();

        self.client_engine.update(context, &mut self.client_server_channel, &mut self.server_client_channel)?;
        self.server_engine.update(context, &mut self.server_client_channel, &mut self.client_server_channel)?; 

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