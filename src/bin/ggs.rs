
extern crate gg;

pub fn main() -> ggez::GameResult { 
    let mut environment = gg::setup::new_server()?;
    environment.run()
}
