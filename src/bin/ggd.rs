
extern crate gg;
#[cfg(target_os = "linux")]
extern crate daemonize;

#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use daemonize::Daemonize;
use gg::err::GgResult;


pub fn main() -> GgResult { 
    #[cfg(target_os = "linux")]
    {
        let stdout = File::create("/tmp/ggd.out").unwrap();
        let stderr = File::create("/tmp/ggd.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/ggd.pid")
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!("ggd started"));

        match daemonize.start() {
            Ok(_) => {
                let mut setup = gg::setup::new_server()?;
                loop{
                    setup.step().unwrap();
                }
            },
            Err(e) => Err(e.into()),
        }
    }

    Ok(())
}
