use recs::Ecs;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use recs::EntityId;
use crate::err::GgResult;

pub trait System<TContext> {

    fn init(&mut self, _: &mut Ecs, _: &TContext) -> GgResult {
        Ok(())
    }

    fn update(&mut self, _: &mut Ecs, _: &TContext) -> GgResult {
        Ok(())
    }

    fn draw(&mut self, _: &Ecs, _: &mut TContext) -> GgResult {
        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut Ecs, _: &TContext) -> GgResult {
        Ok(())
    }

    fn key_down(&mut self,
        _: &mut Ecs,
        _: &mut TContext,
        _: KeyCode,
        _: KeyMods,
        _: bool) {}

    fn key_up(&mut self,
        _: &mut Ecs,
        _: &mut TContext,
        _: KeyCode,
        _: KeyMods) {}
}