use crate::context::TimerService;
use recs::Ecs;
use crate::component::body::Body;
use recs::EntityId;
use crate::err::GgResult;
use crate::system::system::System;
use crate::component::sprite::Sprite;

pub struct PhysicsSystem {
}

impl<TContext> System<TContext> for PhysicsSystem where TContext: TimerService {
    fn update(
        &mut self, 
        state: &mut Ecs, 
        context: &TContext) -> GgResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body);
        state.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : &mut Body = state.borrow_mut(entity).unwrap();

            let t_delta = context.average_delta().as_secs_f32();
            body.step(t_delta);

            let loc = body.get_loc().clone();

            // update sprite
            if let Ok(sprite) = state.borrow_mut::<Sprite>(entity) {
                sprite.location = loc;
                sprite.orientation = 0.0;
            }
        };

        Ok(())
    }
}