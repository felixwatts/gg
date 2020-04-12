use std::time::Duration;
use crate::context::TimerService;
use ggez::graphics::DrawParam;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use crate::err::GgResult;
use crate::context::GfxService;
use std::convert::TryInto;

impl TimerService for ggez::Context {
    fn average_delta(&self) -> Duration{
        ggez::timer::average_delta(self)
    }

    fn time_since_start(&self) -> Duration {
        ggez::timer::time_since_start(self)
    }
}

impl GfxService for ggez::Context {
    fn new_img(&mut self, _filename: &'static str) -> GgResult<ggez::graphics::Image> {

        let png_bytes = include_bytes!("../../resources/gfx.png");
        let decoder = png::Decoder::new(&png_bytes[..]);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let mut img = ggez::graphics::Image::from_rgba8(
            self,
            info.width.try_into().unwrap(),
            info.height.try_into().unwrap(),
            &buf
        )?;
        img.set_filter(ggez::graphics::FilterMode::Nearest);

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