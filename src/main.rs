mod system;
mod component;
mod engine;
mod environment;
mod entity;
mod state;
mod network;
mod local_client_server;

extern crate nalgebra;
extern crate ggez;
#[macro_use]
extern crate recs;

pub fn main() -> ggez::GameResult { 
    let mut environment = crate::environment::Environment::new()?;
    environment.run()
}
