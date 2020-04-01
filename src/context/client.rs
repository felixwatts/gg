use std::time::Duration;
use crate::context::TimerService;
use ggez::graphics::DrawParam;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use crate::err::GgResult;
use crate::context::GfxService;

impl TimerService for ggez::Context {
    fn average_delta(&self) -> Duration{
        ggez::timer::average_delta(self)
    }
}

impl GfxService for ggez::Context {
    fn new_img(&mut self, filename: &'static str) -> GgResult<ggez::graphics::Image> {
        let img = ggez::graphics::Image::new(self, filename)?;
        Ok(img)
    }

    fn set_screen_coordinates(&mut self, rect: Rect) -> GgResult {
        ggez::graphics::set_screen_coordinates(self, rect)?;
        Ok(())
    }

    fn clear(&mut self, color: Color) {
        ggez::graphics::clear(self, color)
    }

    fn draw(&mut self, sprite_batch: &SpriteBatch, draw_param: DrawParam) -> GgResult {
        ggez::graphics::draw(self, sprite_batch, draw_param)?;
        Ok(())
    }

    fn present(&mut self) -> GgResult {
        ggez::graphics::present(self)?;
        Ok(())
    }
}