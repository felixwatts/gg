
extern crate gg;
#[cfg(feature = "server")]
extern crate daemonize;

#[cfg(feature = "server")]
use std::fs::File;
#[cfg(feature = "server")]
use daemonize::Daemonize;
use gg::err::GgResult;


pub fn main() -> GgResult { 
    #[cfg(feature = "server")]
    {
        let stdout = File::create("/tmp/ggd.out").unwrap();
        let stderr = File::create("/tmp/ggd.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/ggd.pid")
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!("ggd started"));

        return match daemonize.start() {
            Ok(_) => {
                let mut setup = gg::setup::new_server()?;
                loop{
                    setup.step().unwrap();
                }
            },
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(not(feature = "server"))]
    Ok(())
}
