use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::state::State;
use recs::EntityId;
use ggez::Context;
use ggez::GameResult;

pub trait System {

    fn init(&mut self, _: &mut State, _: &Context) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _: &mut State, _: &Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _: &State, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut State) -> GameResult {
        Ok(())
    }

    fn key_down(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool) {}

    fn key_up(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods) {}
}