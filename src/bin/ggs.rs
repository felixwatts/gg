
extern crate gg;

pub fn main() -> ggez::GameResult { 
    let mut environment = gg::setup::new_local_client_server(0u32)?;
    environment.run()
}
