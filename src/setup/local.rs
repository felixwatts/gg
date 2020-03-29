use ggez::GameResult;
use crate::network::NoNetwork;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct LocalSetup{
    engine: Engine<NoNetwork>
}

impl LocalSetup{
    pub fn new(context: &mut ggez::Context) -> LocalSetup{
        LocalSetup{
            engine: crate::engine::new_local(context).unwrap()
        }
    }
}

impl EventHandler for LocalSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.engine.update(context, &mut NoNetwork{})?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
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
        self.engine.key_down_event(context, &mut NoNetwork{}, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.engine.key_up_event(context, &mut NoNetwork{}, keycode, keymod);
    }
}