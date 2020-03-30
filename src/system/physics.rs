use recs::Ecs;
use crate::component::Sprite;
use crate::component::body::Body;
use recs::EntityId;
use crate::err::GgResult;
use ggez::Context;
use crate::system::system::System;

pub struct PhysicsSystem {
}

impl System for PhysicsSystem {
    fn update(
        &mut self, 
        state: &mut Ecs, 
        context: &Context) -> GgResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body);
        state.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : &mut Body = state.borrow_mut(entity).unwrap();

            body.step(ggez::timer::average_delta(context).as_secs_f32());

            let loc = body.get_loc().clone();

            // update sprite
            if let Ok(sprite) = state.borrow_mut::<Sprite>(entity) {
                sprite.location = loc;
                sprite.orientation = 0.0;
            }
        };

        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut Ecs) -> GgResult {
        Ok(())
    }
}