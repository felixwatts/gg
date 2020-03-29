
extern crate gg;

pub fn main() -> GgResult { 
    let mut environment = gg::setup::new_client()?;
    environment.run()
}
