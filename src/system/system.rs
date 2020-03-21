use crate::state::State;
use recs::EntityId;
use ggez::Context;
use ggez::GameResult;
use recs::Ecs;

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
}