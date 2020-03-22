use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::network::SimChannel;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use ggez::GameResult;
use crate::engine::Engine;
use std::env;
use std::path;
use ggez::event;

pub struct Environment {
    context: ggez::Context,
    event_loop: ggez::event::EventsLoop,
    local_client_server: LocalClientServer,
}

struct LocalClientServer{
    client_engine: Engine,
    server_engine: Engine,
    client_server_channel: SimChannel<ClientMsg>,
    server_client_channel: SimChannel<ServerMsg>,
}

impl LocalClientServer{
    pub fn new(context: &mut ggez::Context) -> LocalClientServer{

        let client_server_channel = SimChannel::<ClientMsg>::new(0u32);
        let server_client_channel = SimChannel::<ServerMsg>::new(0u32);

        let client_engine = crate::engine::new_client(context).unwrap();
        let server_engine = crate::engine::new_server(context).unwrap();

        LocalClientServer{
            client_engine: client_engine,
            server_engine: server_engine,
            client_server_channel: client_server_channel,
            server_client_channel: server_client_channel
        }
    }
}

impl EventHandler for LocalClientServer {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        self.client_engine.update(context, &self.client_server_channel, &mut self.server_client_channel);
        self.server_engine.update(context, &self.server_client_channel, &mut self.client_server_channel); 

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {

        self.server_engine.draw(context);
        self.client_engine.draw(context);

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

impl Environment {
    pub fn new() -> GameResult<Environment> {
        let mut cb = ggez::ContextBuilder::new("gg", "ggez");
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            cb = cb.add_resource_path(path);
        }
        let (mut context, event_loop) = cb.build()?;

        let local_client_server = LocalClientServer::new(&mut context);

        Ok(Environment{
            context: context,
            event_loop: event_loop,
            local_client_server
        })
    }

    pub fn run(&mut self) -> GameResult {
        event::run(&mut self.context, &mut self.event_loop, &mut self.local_client_server)
    }
}