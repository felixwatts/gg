use crate::component::Keyboard;
use crate::input::default_key_mapping;
use recs::Ecs;
use crate::component::Network;
use crate::component::Anchor;
use crate::system::system::System;
use nalgebra::Vector2;
use crate::component::Focus;
use crate::entity::*;
use crate::component::body::Body;
use crate::component::gorilla::Gorilla;
use crate::err::GgResult;
use recs::EntityId;
use crate::input::Button;

pub struct GorillaSystem {
    pub is_local: bool
}

pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>, with_focus: bool) -> GgResult<EntityId> {
    let gorilla = ecs.create_entity();
    with_sprite(ecs, gorilla, [1.0, 0.0, 0.0, 1.0], [0.3, 0.3].into())?;    
    ecs.set(gorilla, Gorilla::new()).unwrap();
    ecs.set(gorilla, Body::new(loc, Vector2::zeros(), Vector2::new(0.0, -10.0))).unwrap();
    ecs.set(gorilla, Network).unwrap();
    if with_focus {
        ecs.set(gorilla, Focus).unwrap();
        ecs.set(gorilla, Keyboard(default_key_mapping())).unwrap();
    }

    Ok(gorilla)
}

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GgResult<EntityId> {
    let anchor = ecs.create_entity();
    ecs.set(anchor, Anchor).unwrap();
    ecs.set(anchor, Body::new(loc, Vector2::zeros(), Vector2::zeros())).unwrap();
    with_sprite(ecs, anchor, [0.0, 1.0, 1.0, 1.0], [0.1, 0.1].into())?;
    ecs.set(anchor, Network).unwrap();

    Ok(anchor)
}

impl<TContext> System<TContext> for GorillaSystem {
    fn init(&mut self, state: &mut Ecs, _: &TContext) -> GgResult {
    
        spawn_anchor(state, [-3.0, -3.0].into())?;
        spawn_anchor(state, [-3.0, 3.0].into())?;
        spawn_anchor(state, [0.0, 0.0].into())?;
        spawn_anchor(state, [3.0, -3.0].into())?;
        spawn_anchor(state, [3.0, 3.0].into())?;

        if self.is_local {
            spawn_gorilla(state, [-1.5, 5.0].into(), true)?;
        }

        Ok(())
    }

    fn update(
        &mut self, 
        state: &mut Ecs, 
        _: &TContext) -> GgResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Gorilla, Body);
        state.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {

            if state.borrow::<Body>(entity).unwrap().get_loc().y < -20.0 {
                state.set(entity, Body::new([-1.5, 5.0].into(), Vector2::zeros(), Vector2::new(0.0, -10.0))).unwrap();
            }

            let gorilla = state.borrow_mut::<Gorilla>(entity).unwrap();
            let mut events = vec![];
            events.extend(gorilla.input_events.drain(..));
            for input_event in events {
                match input_event.button {
                    Button::One =>
                        match input_event.is_down {
                            true => self.try_add_rope(state, entity),
                            false => self.try_remove_rope(state, entity)
                        }
                    ,
                    Button::Two => {
                        let body = state.borrow_mut::<Body>(entity)?;
                        match input_event.is_down {
                            true => body.set_acc(Vector2::new(0.0, -20.0)),
                            false => body.set_acc(Vector2::new(0.0, -10.0))
                        }
                    }
                    
                };
            }
        }
        Ok(())
    }
}

impl GorillaSystem {

    fn try_add_rope(
        &mut self, 
        state: &mut Ecs, 
        gorilla: EntityId
    ) {
        let gorilla_body = state.borrow::<Body>(gorilla).unwrap();
        if gorilla_body.get_is_attached() {
            return;
        }
        let loc = gorilla_body.get_loc();

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Anchor, Body);
        state.collect_with(&filter, &mut ids);
        let closest_anchor = ids
            .iter()
            .map(|&id| (id, (loc - state.borrow::<Body>(id).unwrap().get_loc()).norm()))
            .min_by(|a, b| {
                if a.1 > b.1 { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
            })
            .map(|a| a.0);

        if let Some(anchor) = closest_anchor {
            let anchor_loc = state.borrow::<Body>(anchor).unwrap().get_loc().clone();
            let attached_body = state.borrow::<Body>(gorilla).unwrap().to_attached(anchor_loc);
            state.set(gorilla, attached_body).unwrap();
        }
    }

    fn try_remove_rope(
        &mut self, 
        state: &mut Ecs, 
        gorilla: EntityId
    ) {
        let gorilla_body = state.borrow::<Body>(gorilla).unwrap();
        if !gorilla_body.get_is_attached() {
            return;
        }
        let detached_body = gorilla_body.to_detached();
        state.set(gorilla, detached_body).unwrap();
    }
}