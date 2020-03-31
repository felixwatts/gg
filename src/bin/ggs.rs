
extern crate gg;

use gg::err::GgResult;

pub fn main() -> GgResult { 
    let mut setup = gg::setup::new_server()?;
    loop{
        setup.step().unwrap();
    }
}
