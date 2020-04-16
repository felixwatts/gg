mod timer;

use std::time::Duration;
use crate::context::TimerService;
use crate::context::server::timer::TimeContext;

#[derive(Default)]
pub struct ServerContext{
    timer: TimeContext
}

impl ServerContext{
    // pub fn new() -> ServerContext {
    //     ServerContext{
    //         timer: TimeContext::new()
    //     }
    // }

    pub fn step(&mut self){
        self.timer.tick();
    }
}

impl TimerService for ServerContext {
    fn average_delta(&self) -> Duration{
        self.timer.average_delta()
    }

    fn time_since_start(&self) -> Duration {
        self.timer.time_since_start()
    }
}