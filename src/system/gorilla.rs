use crate::state::State;
use ggez::Context;
use crate::system::system::System;
use nalgebra::Vector2;
use crate::component::Focus;
use crate::entity::*;
use crate::component::InitRevoluteJoint;
use crate::component::Overlapping;
use crate::component::Dead;
use crate::component::Sprite;
use crate::component::Owns;
use crate::component::Gorilla;
use ggez::GameResult;
use recs::EntityId;
use recs::Ecs;
use ggez::input::keyboard::KeyCode;
use nphysics2d::object::BodyStatus;

pub struct GorillaSystem {
}

pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult<EntityId> {
    let gorilla = ecs.create_entity();

    with_body(ecs, gorilla, loc, BodyStatus::Dynamic)?;
    with_sensor(ecs, gorilla, 1.0)?;
    with_collider(ecs, gorilla, 0.15)?;
    with_overlapping(ecs, gorilla)?;
    with_sprite(ecs, gorilla, [1.0, 0.0, 0.0, 1.0], [0.3, 0.3].into())?;
    ecs.set(gorilla, Focus).unwrap();
    ecs.set(gorilla, Owns(vec![])).unwrap();
    ecs.set(gorilla, Gorilla{rope: None}).unwrap();

    Ok(gorilla)
}

pub fn spawn_rope(ecs: &mut recs::Ecs, from_entity: EntityId, to_entity: EntityId) -> GameResult<EntityId> {
    let rope = ecs.create_entity();

    let p1 = ecs.borrow::<Sprite>(from_entity).unwrap().location;
    let p2 = ecs.borrow::<Sprite>(to_entity).unwrap().location;
    let offset = p2 - p1;

    ecs.set(rope, InitRevoluteJoint{
        end1: from_entity,
        end2: to_entity,
        anchor1: nalgebra::Point2::new(0.0, offset.norm()).into(),
        anchor2: nalgebra::Point2::new(0.0, 0.0)
    }).unwrap();
    crate::entity::with_sprite(ecs, rope, [0.0, 1.0, 1.0, 1.0].into(), [0.1, 0.0].into())?;

    if let Ok(owns) = ecs.borrow_mut::<Owns>(from_entity) {
        owns.0.push(rope);
    }
    if let Ok(owns) = ecs.borrow_mut::<Owns>(to_entity) {
        owns.0.push(rope);
    }

    Ok(rope)
}

impl System for GorillaSystem {
    fn init(&mut self, state: &mut State, _: &Context) -> GameResult {
        spawn_gorilla(&mut state.ecs, [-0.95, 2.0].into())?;
        Ok(())
    }

    fn update(&mut self, state: &mut State, context: &Context) -> GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Gorilla, Owns, Sprite);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {

            if state.ecs.borrow::<Sprite>(entity).unwrap().location.y < -10.0 {
                state.ecs.set(entity, Dead).unwrap();
            }

            if ggez::input::keyboard::is_key_pressed(context, KeyCode::Space) {
                self.try_add_rope(&mut state.ecs, entity)?;
            } else {
                self.try_remove_rope(&mut state.ecs, entity)?;
            }

        }
        Ok(())
    }
    
    fn teardown_entity(&mut self, entity: EntityId, state: &mut State) -> GameResult {
        if let Ok(&_) = state.ecs.borrow::<Gorilla>(entity) {
            spawn_gorilla(&mut state.ecs, [-0.5, 2.0].into())?;
        }
        Ok(())
    }
}

impl GorillaSystem {
    pub fn new() -> GorillaSystem {
        GorillaSystem {
        }
    }

    fn try_add_rope(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId) -> GameResult {
        let gorilla : Gorilla = ecs.get(entity).unwrap();
        if let None = gorilla.rope {
            let overlapping: &Overlapping = ecs.borrow(entity).unwrap();
            if let Some(&closest_anchor) = overlapping.0.first() {
                let rope = spawn_rope(ecs, entity, closest_anchor)?;
                ecs.set(entity, Gorilla{ rope: Some(rope) }).unwrap();
            };
        }
        
        Ok(())
    }

    fn try_remove_rope(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId) -> GameResult {
            let gorilla : Gorilla = ecs.get(entity).unwrap();
            if let Some(rope) = gorilla.rope {
                ecs.set(rope, Dead).unwrap();
                ecs.set(entity, Gorilla{ rope: None }).unwrap();
                let owns : &mut Owns = ecs.borrow_mut(entity).unwrap();
                owns.0.clear(); // TODO remove the actual rope
            }
            Ok(())
        }
}