pub mod client;
pub mod server;

use crate::err::GgResult;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use std::time::Duration;


pub trait TimerService {
    fn average_delta(&self) -> Duration;
    fn time_since_start(&self) -> Duration;
}

pub trait GfxService {
    fn new_img(&mut self, filename: &'static str) -> GgResult<ggez::graphics::Image>;
    fn set_screen_coordinates(&mut self, rect: Rect) -> GgResult;
    fn clear(&mut self, color: Color);
    fn draw(&mut self, sprite_batch: &SpriteBatch, draw_param: DrawParam) -> GgResult;
    fn present(&mut self) -> GgResult;
}
