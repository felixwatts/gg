use crate::component::Focus;
use recs::EntityId;
use crate::component::Sprite;
use crate::component::body::Body;
use crate::component::Network;
use crate::component::Gorilla;
use crate::network::{ClientMsg, ServerMsg};
use crate::component::RxQueue;
use ggez::GameResult;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;

pub struct ServerSystem {}

impl System for ServerSystem {
    fn update(&mut self, state: &mut State, _: &Context) -> GameResult {

        // read and apply user input messages from clients

        // TODO try to avoid this copy of the whole vector
        let msgs = state.ecs.get::<RxQueue<ClientMsg>>(state.rx_queue).unwrap();
        for msg in msgs.0.iter() {
            match msg {
                ClientMsg::ButtonStateChange(m) => {
                    let mut gorillas = vec![];
                    state.ecs.collect_with(&component_filter!(Gorilla), &mut gorillas);
                    // TODO find the right player
                    if let Some(&gorilla) = gorillas.first() {
                        state.ecs.set(gorilla, Gorilla{button_state: *m}).unwrap();
                    }
                }
            }
        }

        // write all network entities
        let mut network_entities = vec![];
        state.ecs.collect_with(&component_filter!(Network), &mut network_entities);
        for &network_entity in network_entities.iter() {
            if let Ok(body) = state.ecs.get::<Body>(network_entity) {
                let msg = ServerMsg::SetBody(network_entity, body);
                crate::network::tx(state, msg);
            }
            
            if let Ok(sprite) = state.ecs.get::<Sprite>(network_entity) {
                let msg = ServerMsg::SetSprite(network_entity, sprite);
                crate::network::tx(state, msg);
            }

            if let Ok(_) = state.ecs.get::<Focus>(network_entity) {
                let msg = ServerMsg::SetFocus(network_entity);
                crate::network::tx(state, msg);
            }
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, state: &mut State) -> GameResult {
        if state.ecs.has::<Network>(entity).unwrap() {
            let msg = ServerMsg::Kill(entity);
            crate::network::tx(state, msg);
        }

        Ok(())
    }
}