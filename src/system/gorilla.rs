// use crate::component::Gravity;
// use crate::component::RadialLocVel;
// use crate::component::Anchor;
// use crate::state::State;
// use ggez::Context;
// use crate::system::system::System;
// use nalgebra::Vector2;
// use crate::component::Focus;
// use crate::entity::*;
// use crate::component::Dead;
// use crate::component::Sprite;
// use crate::component::Gorilla;
// // use crate::component::planar_body::PlanarLocVel;
// use ggez::GameResult;
// use recs::EntityId;
// use ggez::input::keyboard::KeyCode;


// pub struct GorillaSystem {
// }

// pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>, anchor: EntityId) -> GameResult<EntityId> {
//     let gorilla = ecs.create_entity();
//     with_sprite(ecs, gorilla, [1.0, 0.0, 0.0, 1.0], [0.3, 0.3].into())?;
//     ecs.set(gorilla, Focus).unwrap();
//     ecs.set(gorilla, Gorilla).unwrap();
//     // ecs.set(gorilla, PlanarLocVel::new(loc).to_radial(ecs, anchor)).unwrap();
//     ecs.set(gorilla, Gravity(9.81)).unwrap();

//     Ok(gorilla)
// }

// impl System for GorillaSystem {
//     fn init(&mut self, state: &mut State, _: &Context) -> GameResult {
    
//         spawn_anchor(&mut state.ecs, [-3.0, -3.0].into())?;
//         spawn_anchor(&mut state.ecs, [-3.0, 3.0].into())?;
//         spawn_anchor(&mut state.ecs, [0.0, 0.0].into())?;
//         spawn_anchor(&mut state.ecs, [3.0, -3.0].into())?;
//         let anchor = spawn_anchor(&mut state.ecs, [3.0, 3.0].into())?;

//         spawn_gorilla(&mut state.ecs, [-0.95, 2.0].into(), anchor)?;

//         Ok(())
//     }

//     fn update(&mut self, state: &mut State, context: &Context) -> GameResult {
//         let mut ids: Vec<EntityId> = Vec::new();
//         let filter = component_filter!(Gorilla, Sprite);
//         state.ecs.collect_with(&filter, &mut ids);
//         for &entity in ids.iter() {

//             if state.ecs.borrow::<Sprite>(entity).unwrap().location.y < -100.0 {
//                 state.ecs.set(entity, Dead).unwrap();
//             }

//             if ggez::input::keyboard::is_key_pressed(context, KeyCode::Space) {
//                 self.try_add_rope(state, entity);
//             } else {
//                 self.try_remove_rope(state, entity);
//             }

//             if ggez::input::keyboard::is_key_pressed(context, KeyCode::Return) {
//                 self.try_add_force(state, entity);
//             } else {
//                 self.try_remove_force(state, entity);
//             }

//         }
//         Ok(())
//     }
    
//     fn teardown_entity(&mut self, entity: EntityId, state: &mut State) -> GameResult {
//         // if let Ok(&_) = state.ecs.borrow::<Gorilla>(entity) {
//         //     spawn_gorilla(&mut state.ecs, [-0.5, 2.0].into())?;
//         // }
//         Ok(())
//     }
// }

// impl GorillaSystem {
//     pub fn new() -> GorillaSystem {
//         GorillaSystem {
//         }
//     }

//     fn try_add_rope(
//         &mut self, 
//         state: &mut State, 
//         gorilla: EntityId
//     ) {
//         if let Ok(gorilla_loc_vel) = state.ecs.get::<PlanarLocVel>(gorilla) {

//             let mut ids: Vec<EntityId> = Vec::new();
//             let filter = component_filter!(Anchor, PlanarLocVel);
//             state.ecs.collect_with(&filter, &mut ids);
//             let closest_anchor = ids
//                 .iter()
//                 .map(|&id| (id, gorilla_loc_vel.distance_to(state.ecs.get::<PlanarLocVel>(id).unwrap())))
//                 .min_by(|a, b| {
//                     if a.1 > b.1 { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
//                 })
//                 .map(|a| a.0);

//             if let Some(anchor) = closest_anchor {
//                 state.ecs.unset::<PlanarLocVel>(gorilla).unwrap();
//                 // state.ecs.set(gorilla, gorilla_loc_vel.to_radial(&state.ecs, anchor)).unwrap();
//             }
//         }
//     }

//     fn try_remove_rope(
//         &mut self, 
//         state: &mut State, 
//         gorilla: EntityId
//     ) {
//         if let Ok(gorilla_loc_vel) = state.ecs.get::<RadialLocVel>(gorilla) {
//             state.ecs.unset::<RadialLocVel>(gorilla).unwrap();
//             // state.ecs.set(gorilla, gorilla_loc_vel.to_planar(&state.ecs)).unwrap();
//         }
//     }

//     fn try_add_force(
//         &mut self, 
//         state: &mut State, 
//         gorilla: EntityId
//     ) {
//         if let Ok(gorilla_gravity) = state.ecs.borrow_mut::<Gravity>(gorilla) {
//             gorilla_gravity.0 = 20.0;
//         }
//     }

//     fn try_remove_force(
//         &mut self, 
//         state: &mut State, 
//         gorilla: EntityId
//     ) {
//         if let Ok(gorilla_gravity) = state.ecs.borrow_mut::<Gravity>(gorilla) {
//             gorilla_gravity.0 = 9.81;
//         }
//     }
// }