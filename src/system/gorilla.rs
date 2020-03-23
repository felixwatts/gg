use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Network;
use crate::component::Anchor;
use crate::state::State;
use ggez::Context;
use crate::system::system::System;
use nalgebra::Vector2;
use crate::component::Focus;
use crate::entity::*;
use crate::component::Dead;
use crate::component::Gorilla;
use crate::component::body::Body;
use ggez::GameResult;
use recs::EntityId;

pub struct GorillaSystem {}

pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult<EntityId> {
    let gorilla = ecs.create_entity();
    with_sprite(ecs, gorilla, [1.0, 0.0, 0.0, 1.0], [0.3, 0.3].into())?;
    ecs.set(gorilla, Focus).unwrap();
    ecs.set(gorilla, Gorilla{button_state:[false, false]}).unwrap();
    ecs.set(gorilla, Body::new(loc, Vector2::zeros(), Vector2::new(0.0, -10.0))).unwrap();
    ecs.set(gorilla, Network).unwrap();

    Ok(gorilla)
}

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult<EntityId> {
    let anchor = ecs.create_entity();
    ecs.set(anchor, Anchor).unwrap();
    ecs.set(anchor, Body::new(loc, Vector2::zeros(), Vector2::zeros())).unwrap();
    with_sprite(ecs, anchor, [0.0, 1.0, 1.0, 1.0], [0.1, 0.1].into())?;
    ecs.set(anchor, Network).unwrap();

    Ok(anchor)
}

fn update_button_state(keycode: KeyCode, state: &mut State, context: &Context) {
    match keycode {
        KeyCode::Space | KeyCode::Return => {
            let mut gorillas = vec![];
            state.ecs.collect_with(&component_filter!(Gorilla), &mut gorillas);
            // TODO find the right player
            if let Some(&gorilla) = gorillas.first() {
                let button_state = [
                    ggez::input::keyboard::is_key_pressed(context, KeyCode::Space),
                    ggez::input::keyboard::is_key_pressed(context, KeyCode::Return)
                ];
                state.ecs.set(gorilla, Gorilla{button_state}).unwrap();
            } 
        },
        _ => {}
    }
}

impl System for GorillaSystem {
    fn init(&mut self, state: &mut State, _: &Context) -> GameResult {
    
        spawn_anchor(&mut state.ecs, [-3.0, -3.0].into())?;
        spawn_anchor(&mut state.ecs, [-3.0, 3.0].into())?;
        spawn_anchor(&mut state.ecs, [0.0, 0.0].into())?;
        spawn_anchor(&mut state.ecs, [3.0, -3.0].into())?;
        spawn_anchor(&mut state.ecs, [3.0, 3.0].into())?;

        spawn_gorilla(&mut state.ecs, [-0.95, 2.0].into())?;

        Ok(())
    }

    fn update(
        &mut self, 
        state: &mut State, 
        _: &Context) -> GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Gorilla, Body);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {

            if state.ecs.borrow::<Body>(entity).unwrap().get_loc().y < -30.0 {
                state.ecs.set(entity, Dead).unwrap();
            }

            let gorilla = state.ecs.get::<Gorilla>(entity).unwrap();

            if gorilla.button_state[0] {
                self.try_add_rope(state, entity);
            } else {
                self.try_remove_rope(state, entity);
            }

            if gorilla.button_state[1] {
                if let Ok(body) = state.ecs.borrow_mut::<Body>(entity) {
                    body.set_acc(Vector2::new(0.0, -20.0));
                }
            } else {
                if let Ok(body) = state.ecs.borrow_mut::<Body>(entity) {
                    body.set_acc(Vector2::new(0.0, -10.0));
                }
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

    fn key_down(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        _: bool) {
            update_button_state(keycode, state, context);

        }

    fn key_up(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods) {
            update_button_state(keycode, state, context);
        }
}

impl GorillaSystem {

    fn try_add_rope(
        &mut self, 
        state: &mut State, 
        gorilla: EntityId
    ) {
        let gorilla_body = state.ecs.borrow::<Body>(gorilla).unwrap();
        if gorilla_body.get_is_attached() {
            return;
        }
        let loc = gorilla_body.get_loc();

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Anchor, Body);
        state.ecs.collect_with(&filter, &mut ids);
        let closest_anchor = ids
            .iter()
            .map(|&id| (id, (loc - state.ecs.borrow::<Body>(id).unwrap().get_loc()).norm()))
            .min_by(|a, b| {
                if a.1 > b.1 { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
            })
            .map(|a| a.0);

        if let Some(anchor) = closest_anchor {
            let anchor_loc = state.ecs.borrow::<Body>(anchor).unwrap().get_loc().clone();
            let attached_body = state.ecs.borrow::<Body>(gorilla).unwrap().to_attached(anchor_loc);
            state.ecs.set(gorilla, attached_body).unwrap();
        }
    }

    fn try_remove_rope(
        &mut self, 
        state: &mut State, 
        gorilla: EntityId
    ) {
        let gorilla_body = state.ecs.borrow::<Body>(gorilla).unwrap();
        if !gorilla_body.get_is_attached() {
            return;
        }
        let detached_body = gorilla_body.to_detached();
        state.ecs.set(gorilla, detached_body).unwrap();
    }
}