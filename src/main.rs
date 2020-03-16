mod common;
mod entity;
mod state;
mod anchor;
mod gorilla;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate ggez;
extern crate mint;

use std::env;
use ggez::event;
use ggez::graphics;
use nphysics2d::object::{DefaultBodyHandle};
use std::path;
use crate::state::State;

trait Embodied {
    fn body() -> DefaultBodyHandle;
}

pub fn main() -> ggez::GameResult { 
    let mut cb = ggez::ContextBuilder::new("gg", "ggez");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    let (context, event_loop) = &mut cb.build()?;
    let gfx = graphics::Image::new(context, "/1px.png")?;
    let state = &mut State::new(gfx)?;
    event::run(context, event_loop, state)
}
