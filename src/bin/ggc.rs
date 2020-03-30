
extern crate gg;

use gg::err::GgResult;

pub fn main() -> GgResult { 
    let mut environment = gg::setup::new_client()?;
    environment.run()
}
