pub mod local;
pub mod local_client_server;

use crate::setup::local_client_server::LocalClientServer;
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

pub fn new_local() -> GameResult<Setup<LocalClientServer>> {
    let (mut context, event_loop) = build_context()?;

    let local_client_server = LocalClientServer::new(&mut context, 0u32);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: local_client_server
    })
}

pub fn new_local_client_server(latency: u32) -> GameResult<Setup<LocalClientServer>> {
    let (mut context, event_loop) = build_context()?;

    let local_client_server = LocalClientServer::new(&mut context, latency);

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: local_client_server
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