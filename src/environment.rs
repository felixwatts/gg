use ggez::GameResult;
use crate::engine::Engine;
use std::env;
use std::path;
use ggez::event;

pub struct Environment {
    engine: Engine,
    context: ggez::Context,
    event_loop: ggez::event::EventsLoop
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

        Ok(Environment{
            engine: crate::engine::Engine::new(&mut context)?,
            context: context,
            event_loop: event_loop
        })
    }

    pub fn run(&mut self) -> GameResult {
        event::run(&mut self.context, &mut self.event_loop, &mut self.engine)
    }
}