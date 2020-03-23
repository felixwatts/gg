
extern crate gg;

pub fn main() -> ggez::GameResult { 
    let mut environment = gg::setup::new_client()?;
    environment.run()
}
