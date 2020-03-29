use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::state::State;
use recs::EntityId;
use ggez::Context;
use crate::err::GgResult;

pub trait System {

    fn init(&mut self, _: &mut State, _: &Context) -> GgResult {
        Ok(())
    }

    fn update(&mut self, _: &mut State, _: &Context) -> GgResult {
        Ok(())
    }

    fn draw(&mut self, _: &State, _: &mut Context) -> GgResult {
        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut State) -> GgResult {
        Ok(())
    }

    fn key_down(&mut self,
        _: &mut State,
        _: &mut Context,
        _: KeyCode,
        _: KeyMods,
        _: bool) {}

    fn key_up(&mut self,
        _: &mut State,
        _: &mut Context,
        _: KeyCode,
        _: KeyMods) {}
}