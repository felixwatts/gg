use crate::component::client::Latency;
use crate::input::KeyMapping;
use crate::colors::Color;
use crate::component::Keyboard;
use recs::Ecs;
use crate::component::Network;
use crate::component::Anchor;
use crate::system::system::System;
use nalgebra::Vector2;
use crate::component::Focus;
use crate::component::body::Body;
use crate::component::gorilla::Gorilla;
use crate::err::GgResult;
use recs::EntityId;
use crate::input::Button;
use crate::component::sprite::Sprite;
use crate::colors::WHITE;

#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use ggez::input::keyboard::KeyCode;
#[cfg(test)]
use ggez::event::KeyMods;
#[cfg(test)]
use crate::testing::assert_roughly_eq;

pub struct GorillaSystem {
    pub is_latency_compensation_enabled: bool
}

pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>, color: Color, key_mapping: Option<KeyMapping>, with_focus: bool) -> GgResult<EntityId> {
    let gorilla = ecs.create_entity();
    ecs.set(gorilla, Sprite::new(color, [0.3, 0.3].into()))?;  
    ecs.set(gorilla, Gorilla::new(loc.clone()))?;
    ecs.set(gorilla, Body::new_dynamic(loc, Vector2::zeros(), Vector2::new(0.0, -10.0)))?;
    ecs.set(gorilla, Network)?;
    ecs.set(gorilla, Latency(0.0))?;

    if with_focus {
        ecs.set(gorilla, Focus)?;
    }

    if let Some(km) = key_mapping {
        ecs.set(gorilla, Keyboard(km)).unwrap();
    }

    Ok(gorilla)
}

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GgResult<EntityId> {
    let anchor = ecs.create_entity();
    ecs.set(anchor, Anchor)?;
    ecs.set(anchor, Body::new_static(loc))?;
    ecs.set(anchor, Sprite::new(WHITE, [0.1, 0.1].into()))?;
    ecs.set(anchor, Network)?;

    Ok(anchor)
}

impl<TContext> System<TContext> for GorillaSystem {
    fn init(&mut self, state: &mut Ecs, _: &TContext) -> GgResult {
    
        spawn_anchor(state, [-3.0, -3.0].into())?;
        spawn_anchor(state, [-3.0, 3.0].into())?;
        spawn_anchor(state, [0.0, 0.0].into())?;
        spawn_anchor(state, [3.0, -3.0].into())?;
        spawn_anchor(state, [3.0, 3.0].into())?;

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
            self.respawn_if_outside_bounds(entity, state)?;
            self.process_user_input(entity, state)?;
        }
        Ok(())
    }
}

impl GorillaSystem {

    fn respawn_if_outside_bounds(&mut self, entity: EntityId, state: &mut Ecs) -> GgResult {
        if state.borrow::<Body>(entity).unwrap().get_loc().y < -20.0 {
            let spawn_location = state.borrow::<Gorilla>(entity).unwrap().spawn_location.clone();
            state.set(entity, Body::new_dynamic(spawn_location.into(), Vector2::zeros(), Vector2::new(0.0, -10.0))).unwrap();
        }

        Ok(())
    }

    fn process_user_input(&mut self, entity: EntityId, state: &mut Ecs) -> GgResult {
        let gorilla = state.borrow_mut::<Gorilla>(entity).unwrap();
        let mut events = vec![];
        events.extend(gorilla.input_events.drain(..));

        if events.len() == 0 {
            return Ok(())
        }

        self.apply_latency_compensation(entity, state, false)?;

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

        self.apply_latency_compensation(entity, state, true)?;

        Ok(())
    }

    fn apply_latency_compensation(&mut self, entity: EntityId, state: &mut Ecs, is_forward: bool) -> GgResult {
        if !self.is_latency_compensation_enabled {
            return Ok(());
        }

        let mut latency = state.borrow::<Latency>(entity).unwrap().0 / 2.0;
        if !is_forward {
            latency = -latency;
        }

        #[cfg(debug)]
        println!("apply latency comp of {}s", latency);

        let body = state.borrow_mut::<Body>(entity).unwrap();
        body.step(latency);

        Ok(())
    }

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

#[test]
fn test_latency_compensation() {
    expect_latency_compensation(KeyCode::Return);
    expect_latency_compensation(KeyCode::Space);
}

#[cfg(test)]
fn expect_latency_compensation(user_input: KeyCode) {
    // create two setups, one with zero latency and no latency comp and
    // one with non-zero latency and latency comp
    let mut setups = vec![
        crate::testing::MockSetup::new(Duration::from_millis(0), Duration::from_millis(16), false),
        crate::testing::MockSetup::new(Duration::from_millis(50), Duration::from_millis(16), true)
    ];

    // allow time for server to measure latency
    for setup in setups.iter_mut() {
        for _ in 0..10 {
            setup.step();
        }
    }

    // in each setup a client issues the same user input at the same time
    for setup in setups.iter_mut() {
        setup.client1_engine.key_down_event(
            &mut setup.context,
            user_input,
            KeyMods::empty(),
            false,
        );
    }

    // after both servers have received the user input
    for setup in setups.iter_mut() {
        for _ in 0..10 {
            setup.step();
        }
    }

    // both should be in roughly the same state 
    // (despite the fact that they recevied it at different times)
    let bodies = setups.iter().map(|s| {
        let mut body_entities = vec![];
        s.server_engine.get_state().collect_with(&component_filter!(crate::component::gorilla::Gorilla), &mut body_entities);
        body_entities.iter().map(|&e| s.server_engine.get_state().borrow::<Body>(e).unwrap().get_loc().y).collect::<Vec::<_>>()
    }).collect::<Vec::<_>>();

    for b in 0..bodies[0].len() {
        assert_roughly_eq("body.y", bodies[0][b], bodies[1][b]);
    }
}