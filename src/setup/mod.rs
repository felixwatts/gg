mod local;
mod local_client_server;
mod server;
mod client;

use crate::setup::client::ClientSetup;
use crate::setup::server::ServerSetup;
use crate::setup::local::LocalSetup;
use crate::setup::local_client_server::LocalClientServerSetup;
use ggez::GameResult;
use std::env;
use std::path;
use ggez::event;
use ggez::event::EventsLoop;
use ggez::Context;
use ggez::event::EventHandler;

pub struct Setup<TSetup> where TSetup: EventHandler {
    context: ggez::Context,
    event_loop: ggez::event::EventsLoop,
    game: TSetup,
}

pub fn new_server() -> GameResult<Setup<ServerSetup>>{
    let (mut context, event_loop) = build_context()?;

    let setup = ServerSetup::new(&mut context);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

pub fn new_client() -> GameResult<Setup<ClientSetup>>{
    let (mut context, event_loop) = build_context()?;

    let setup = ClientSetup::new(&mut context);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

pub fn new_local() -> GameResult<Setup<LocalSetup>> {
    let (mut context, event_loop) = build_context()?;

    let setup = LocalSetup::new(&mut context);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

pub fn new_local_client_server(latency: u32) -> GameResult<Setup<LocalClientServerSetup>> {
    let (mut context, event_loop) = build_context()?;

    let setup = LocalClientServerSetup::new(&mut context, latency);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

fn build_context() -> GameResult<(Context, EventsLoop)> {
    let mut cb = ggez::ContextBuilder::new("gg", "ggez");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    cb.build()
}

impl<TSetup> Setup<TSetup> where TSetup: EventHandler {
    pub fn run(&mut self) -> GameResult {
        event::run(&mut self.context, &mut self.event_loop, &mut self.game)
    }
}