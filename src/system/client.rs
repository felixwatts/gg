use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::component::Focus;
use crate::network::ServerMsg;
use crate::err::GgResult;
use recs::EntityId;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;
use crate::network::ClientMsg;
use std::collections::HashMap;

pub struct ClientSystem {
    network_entity_id_mapping: HashMap<u64, EntityId>
}

fn tx_key_state<TNetwork>(
    keycode: KeyCode, 
    state: &mut State, 
    context: &mut Context, 
    network: &mut TNetwork) 
    where TNetwork: TxChannel::<ClientMsg> {
    match keycode {
        KeyCode::Space | KeyCode::Return =>
        {
            let key_state = [
                ggez::input::keyboard::is_key_pressed(context, KeyCode::Space),
                ggez::input::keyboard::is_key_pressed(context, KeyCode::Return),
            ];
            network.enqueue(ClientMsg::ButtonStateChange(key_state));
        },
        _ => {}
    }
}

impl ClientSystem {

    pub fn new() -> ClientSystem {
        ClientSystem{
            network_entity_id_mapping: HashMap::<u64, EntityId>::new()
        }
    }

    fn to_client_entity_id(&mut self, state: &mut State, server_id: u64) -> EntityId {
        if let Some(client_id) = self.network_entity_id_mapping.get(&server_id) {
            *client_id
        } else {
            let client_id = state.ecs.create_entity();
            self.network_entity_id_mapping.insert(server_id, client_id);

            client_id
        }
    }
}

impl<TNetwork> System<TNetwork> for ClientSystem where TNetwork: TxChannel::<ClientMsg> + RxChannel::<ServerMsg> {
    fn key_down(&mut self,
        state: &mut State,
        context: &mut Context,
        network: &mut TNetwork,
        keycode: KeyCode,
        _: KeyMods,
        _: bool) {
        tx_key_state(keycode, state, context, network);
    }

    fn key_up(&mut self,
        state: &mut State,
        context: &mut Context,
        network: &mut TNetwork,
        keycode: KeyCode,
        _: KeyMods) {
        tx_key_state(keycode, state, context, network);
    }

    fn update(
        &mut self, 
        state: &mut State, 
        _: &Context, 
        network: &mut TNetwork) -> GgResult {

        // read all network entities
        let mut buffer = vec![];
        network.dequeue(&mut buffer);
        for msg in buffer {
            match msg {
                ServerMsg::SetBody(server_id, body) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.ecs.set(client_id, body).unwrap();
                },
                ServerMsg::SetSprite(server_id, sprite) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.ecs.set(client_id, sprite).unwrap();
                },
                ServerMsg::SetFocus(server_id) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.ecs.set(client_id, Focus).unwrap();
                },
                ServerMsg::Kill(server_id) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    self.network_entity_id_mapping.remove(&server_id);
                    state.ecs.destroy_entity(client_id).unwrap();
                }
            }
        }

        Ok(())
    }
}