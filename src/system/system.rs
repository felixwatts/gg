use recs::EntityId;
use ggez::Context;
use ggez::GameResult;
use recs::Ecs;

pub trait System {

    fn init(&mut self, _: &mut Ecs, _: &Context) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _: &mut Ecs, _: &Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _: &Ecs, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut Ecs) -> GameResult {
        Ok(())
    }
}