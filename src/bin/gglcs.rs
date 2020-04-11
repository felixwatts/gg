
extern crate gg;

use gg::err::GgResult;
use std::time::Duration;

pub fn main() -> GgResult { 
    let mut environment = gg::setup::new_local_client_server(Duration::from_millis(50), true)?;
    environment.run()
}
