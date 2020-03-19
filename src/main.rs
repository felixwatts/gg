mod system;
mod component;
mod engine;
mod environment;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate ggez;
extern crate mint;
#[macro_use]
extern crate recs;

pub fn main() -> ggez::GameResult { 
    let mut environment = crate::environment::Environment::new()?;
    environment.run()
}
