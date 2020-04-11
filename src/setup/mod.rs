mod local;
mod local_client_server;
mod server;
mod client;

use crate::setup::client::ClientSetup;
use crate::setup::server::ServerSetup;
use crate::setup::local::LocalSetup;
use crate::setup::local_client_server::LocalClientServerSetup;
use crate::err::GgResult;
use std::time::Duration;
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

pub fn new_server() -> GgResult<ServerSetup>{
    let context = crate::context::server::ServerContext::new();
    let setup = ServerSetup::new(context)?;
    Ok(setup)
}

pub fn new_client(server_addr: &String) -> GgResult<Setup<ClientSetup>>{
    let (mut context, event_loop) = build_context()?;

    let setup = ClientSetup::new(&mut context, server_addr)?;

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

pub fn new_local() -> GgResult<Setup<LocalSetup>> {
    let (mut context, event_loop) = build_context()?;

    let setup = LocalSetup::new(&mut context)?;

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

pub fn new_local_client_server(latency: Duration, is_latency_compensation_enabled: bool) -> GgResult<Setup<LocalClientServerSetup>> {
    let (mut context, event_loop) = build_context()?;

    let setup = LocalClientServerSetup::new(&mut context, latency, is_latency_compensation_enabled)?;

    Ok(Setup{
        context: context,
        event_loop: event_loop,
        game: setup
    })
}

fn build_context() -> GgResult<(Context, EventsLoop)> {
    let mut cb = ggez::ContextBuilder::new("gg", "ggez");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    Ok(cb.build()?)
}

impl<TSetup> Setup<TSetup> where TSetup: EventHandler {
    pub fn run(&mut self) -> GgResult {
        Ok(event::run(&mut self.context, &mut self.event_loop, &mut self.game)?)
    }
}