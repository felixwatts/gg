use crate::network::NoMsg;
use crate::network::DummyChannel;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct LocalSetup{
    engine: Engine<NoMsg, NoMsg>
}

impl LocalSetup{
    pub fn new(context: &mut ggez::Context) -> LocalSetup{
        LocalSetup{
            engine: crate::engine::new_local(context).unwrap()
        }
    }
}

impl EventHandler for LocalSetup {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        self.engine.update(context, &mut DummyChannel{})?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        self.engine.draw(context)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.engine.key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.engine.key_up_event(context, keycode, keymod);
    }
}