use crate::component::InitForce;
use crate::component::Body;
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
    with_sensor(ecs, gorilla, 2.5)?;
    with_collider(ecs, gorilla, 0.15)?;
    with_overlapping(ecs, gorilla)?;
    with_sprite(ecs, gorilla, [1.0, 0.0, 0.0, 1.0], [0.3, 0.3].into())?;
    ecs.set(gorilla, Focus).unwrap();
    ecs.set(gorilla, Owns(vec![])).unwrap();
    ecs.set(gorilla, Gorilla{rope: None, force: None}).unwrap();

    Ok(gorilla)
}

pub fn spawn_rope(state: &mut State, gorilla: EntityId, anchor: EntityId) -> GameResult<EntityId> {
    let rope = state.ecs.create_entity();

    let body_gorilla = state.world.bodies.rigid_body(state.ecs.borrow::<Body>(gorilla).unwrap().0).unwrap();
    let body_anchor = state.world.bodies.rigid_body(state.ecs.borrow::<Body>(anchor).unwrap().0).unwrap();

    let pos_gorilla = body_gorilla.position();
    let pos_anchor = body_anchor.position();
    let offset = pos_anchor.translation.vector - pos_gorilla.translation.vector;

    let heading = nalgebra::Rotation2::rotation_between(&nalgebra::Vector2::y(), &offset);
    let angle = heading.angle();
    let body_gorilla_mut = state.world.bodies.rigid_body_mut(state.ecs.borrow::<Body>(gorilla).unwrap().0).unwrap();
    body_gorilla_mut.set_position(
        nalgebra::Isometry2::new(
            body_gorilla_mut.position().translation.vector, 
            angle
        )
    );

    state.ecs.set(rope, InitRevoluteJoint{
        end1: gorilla,
        end2: anchor,
        anchor1: nalgebra::Point2::new(0.0, offset.norm()).into(),
        anchor2: nalgebra::Point2::new(0.0, 0.0)
    }).unwrap();
    crate::entity::with_sprite(&mut state.ecs, rope, [0.0, 1.0, 1.0, 1.0].into(), [0.1, 0.0].into())?;

    if let Ok(owns) = state.ecs.borrow_mut::<Owns>(gorilla) {
        owns.0.push(rope);
    }
    if let Ok(owns) = state.ecs.borrow_mut::<Owns>(anchor) {
        owns.0.push(rope);
    }

    Ok(rope)
}

pub fn spawn_force(state: &mut State, gorilla: EntityId) -> GameResult<EntityId> {
    let force = state.ecs.create_entity();
    state.ecs.set(force, InitForce{
        entity: gorilla,
        force: Vector2::new(0.0, -15.0)
    }).unwrap();

    if let Ok(owns) = state.ecs.borrow_mut::<Owns>(gorilla) {
        owns.0.push(force);
    }

    Ok(force)
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
                self.try_add_rope(state, entity)?;
            } else {
                self.try_remove_rope(&mut state.ecs, entity)?;
            }

            if ggez::input::keyboard::is_key_pressed(context, KeyCode::Return) {
                self.try_add_force(state, entity)?;
            } else {
                self.try_remove_force(&mut state.ecs, entity)?;
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
        state: &mut State, 
        entity: EntityId
    ) -> GameResult {
        let gorilla : Gorilla = state.ecs.get(entity).unwrap();
        if let None = gorilla.rope {
            let overlapping: &Overlapping = state.ecs.borrow(entity).unwrap();
            if let Some(&closest_anchor) = overlapping.0.first() {
                let rope = spawn_rope(state, entity, closest_anchor)?;
                state.ecs.set(entity, Gorilla{ rope: Some(rope), force: gorilla.force }).unwrap();
            };
        }
        
        Ok(())
    }

    fn try_remove_rope(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId
    ) -> GameResult {
        let gorilla : Gorilla = ecs.get(entity).unwrap();
        if let Some(rope) = gorilla.rope {
            ecs.set(rope, Dead).unwrap();
            ecs.set(entity, Gorilla{ rope: None, force: gorilla.force }).unwrap();
            // let owns : &mut Owns = ecs.borrow_mut(entity).unwrap();
            // owns.0.clear(); // TODO remove the actual rope
        }
        Ok(())
    }

    fn try_add_force(
        &mut self, 
        state: &mut State, 
        entity: EntityId
    ) -> GameResult {
        let gorilla : Gorilla = state.ecs.get(entity).unwrap();
        if let None = gorilla.force {
            if let Some(_) = gorilla.rope {
                let force = spawn_force(state, entity)?;
                state.ecs.set(entity, Gorilla{ rope: gorilla.rope, force: Some(force) }).unwrap();
            };
        }
        
        Ok(())
    }

    fn try_remove_force(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId
    ) -> GameResult {
        let gorilla : Gorilla = ecs.get(entity).unwrap();
        if let Some(force) = gorilla.force {
            ecs.set(force, Dead).unwrap();
            ecs.set(entity, Gorilla{ rope: gorilla.rope, force: None }).unwrap();
            // let owns : &mut Owns = ecs.borrow_mut(entity).unwrap();
            // owns.0.clear(); // TODO remove the actual rope
        }
        Ok(())
    }
}