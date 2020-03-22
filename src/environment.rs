use crate::local_client_server::LocalClientServer;
use ggez::GameResult;
use std::env;
use std::path;
use ggez::event;

pub struct Environment {
    context: ggez::Context,
    event_loop: ggez::event::EventsLoop,
    local_client_server: LocalClientServer,
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