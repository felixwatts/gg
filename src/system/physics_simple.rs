use crate::component::Gravity;
use crate::component::Sprite;
use crate::component::RadialLocVel;
use crate::component::PlanarLocVel;
use recs::EntityId;
use ggez::GameResult;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;

pub struct PhysicsSimpleSystem {

}

impl System for PhysicsSimpleSystem {
    fn update(&mut self, state: &mut State, context: &Context) -> GameResult {
        self.refresh_planar_loc_vels(state, context)?;
        self.refresh_radial_loc_vels(state, context)?;


        Ok(())
    }

    fn teardown_entity(&mut self, _: EntityId, _: &mut State) -> GameResult {
        Ok(())
    }
}

impl PhysicsSimpleSystem {
    fn refresh_planar_loc_vels(&mut self, state: &mut State, _: &Context) -> GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(PlanarLocVel);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let mut loc_vel : PlanarLocVel = state.ecs.get(entity).unwrap();

            // apply gravity to velocity
            if let Ok(gravity) = state.ecs.get::<Gravity>(entity) {
                loc_vel.vel.y -= gravity.0 * 0.001;
            }

            // apply velocity to location
            loc_vel.loc += loc_vel.vel;

            let loc = loc_vel.loc;

            // update sprite
            if let Ok(sprite) = state.ecs.borrow_mut::<Sprite>(entity) {
                sprite.location = loc;
                sprite.orientation = 0.0;

                println!("{},{}", sprite.location.x, sprite.location.y);
            }

            state.ecs.set(entity, loc_vel).unwrap();
        };

        Ok(())
    }

    fn refresh_radial_loc_vels(&mut self, state: &mut State, _: &Context) -> GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(RadialLocVel);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let mut loc_vel : RadialLocVel = state.ecs.get(entity).unwrap();

            let dx = loc_vel.radius * loc_vel.loc.sin();
            let dy = loc_vel.radius * loc_vel.loc.cos();

            // apply gravity to velocity
            // if let Ok(gravity) = state.ecs.get::<Gravity>(entity) {
            //     let v = loc_vel.vel / nalgebra::Vector2::<f32>::new(dx, dy).norm();
    
            //     let vx = dx * v;
            //     let vy = dy * v;
    
            //     let v_vertical = vy / (vy + vx);
    
            //     loc_vel.vel = loc_vel.vel - ((gravity.0 * 0.001) * v_vertical);
            // }

            // apply velocity to location
            loc_vel.loc += loc_vel.vel;            

            // update sprite
            let origin_entity = loc_vel.origin;
            let orientation = loc_vel.loc;
            let origin = state.ecs.get::<PlanarLocVel>(origin_entity).unwrap();
            let loc_world = origin.loc + nalgebra::Vector2::new(dx, dy);
            if let Ok(sprite) = state.ecs.borrow_mut::<Sprite>(entity) {
                sprite.location = loc_world;
                sprite.orientation = orientation;
            }

            state.ecs.set(entity, loc_vel).unwrap();
        };

        Ok(())
    }
}