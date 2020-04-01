
extern crate gg;

use gg::err::GgResult;
use std::env;

pub fn main() -> GgResult { 
    let args: Vec<String> = env::args().collect();
    let server_addr = match args.len() {
        2 => args[1].clone(),
        _ => "127.0.0.1:9001".to_string()
    };

    let mut env = gg::setup::new_client(&server_addr)?;
    env.run()
}