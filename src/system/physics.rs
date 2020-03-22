use crate::component::Sprite;
use crate::component::body::Body;
use recs::EntityId;
use ggez::GameResult;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;

pub struct PhysicsSystem {
}

impl System for PhysicsSystem {
    fn update(&mut self, state: &mut State, context: &Context) -> GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : &mut Body = state.ecs.borrow_mut(entity).unwrap();

            body.step(ggez::timer::average_delta(context).as_secs_f32());

            let loc = body.get_loc().clone();

            // update sprite
            if let Ok(sprite) = state.ecs.borrow_mut::<Sprite>(entity) {
                sprite.location = loc;
                sprite.orientation = 0.0;
            }
        };

        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut State) -> GameResult {
        Ok(())
    }
}