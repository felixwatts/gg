
extern crate gg;

use gg::err::GgResult;

pub fn main() -> GgResult { 
    let mut environment = gg::setup::new_local_client_server(3u32, true)?;
    environment.run()
}
