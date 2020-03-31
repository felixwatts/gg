mod timer;

use ggez::event::KeyCode;
use crate::context::InputService;
use std::time::Duration;
use crate::context::TimerService;
use crate::context::server::timer::TimeContext;

pub struct ServerContext{
    timer: TimeContext
}

impl ServerContext{
    pub fn new() -> ServerContext {
        ServerContext{
            timer: TimeContext::new()
        }
    }

    pub fn step(&mut self){
        self.timer.tick();
    }
}

impl TimerService for ServerContext {
    fn average_delta(&self) -> Duration{
        self.timer.average_delta()
    }
}

impl InputService for ServerContext {
    fn is_key_pressed(&self, _: KeyCode) -> bool{
        false
    }
}