use crate::input::default_key_mapping;
use crate::input::InputEvent;
use recs::Ecs;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::component::Focus;
use crate::network::ServerMsg;
use crate::err::GgResult;
use recs::EntityId;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::system::system::System;
use crate::network::ClientMsg;
use std::collections::HashMap;
use crate::input::{KeyMapping};

pub struct ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg>{
    server: TNetwork,
    network_entity_id_mapping: HashMap<u64, EntityId>,
    key_mapping: KeyMapping
}

impl<TNetwork> ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg>{
    pub fn new(server: TNetwork) -> ClientSystem<TNetwork> {
        ClientSystem{
            server,
            network_entity_id_mapping: HashMap::<u64, EntityId>::new(),
            key_mapping: default_key_mapping()
        }
    }

    fn to_client_entity_id(&mut self, state: &mut Ecs, server_id: u64) -> EntityId {
        if let Some(client_id) = self.network_entity_id_mapping.get(&server_id) {
            *client_id
        } else {
            let client_id = state.create_entity();
            self.network_entity_id_mapping.insert(server_id, client_id);

            client_id
        }
    }
}

impl<TNetwork, TContext> System<TContext> for ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg> {
    fn key_down(
        &mut self,
        _: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool) {
            if repeat { return; }

            match self.key_mapping.get(&keycode) {
                Some(&button) => {
                    self.server.enqueue(ClientMsg::Input(InputEvent{button, is_down: true})).unwrap();
                },
                None => {}
            }
    }

    fn key_up(
        &mut self,
        _: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods) {            
            match self.key_mapping.get(&keycode) {
                Some(&button) => {
                    self.server.enqueue(ClientMsg::Input(InputEvent{button, is_down: false})).unwrap();
                },
                None => {}
            }
    }

    fn update(
        &mut self, 
        state: &mut Ecs, 
        _: &TContext) -> GgResult {

        // read all network entities
        let mut buffer = vec![];
        self.server.dequeue(&mut buffer)?;
        for msg in buffer.drain(..) {
            match msg {
                ServerMsg::SetBody(server_id, body) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.set(client_id, body).unwrap();
                },
                ServerMsg::SetSprite(server_id, sprite) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.set(client_id, sprite).unwrap();
                },
                ServerMsg::SetFocus(server_id) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    state.set(client_id, Focus).unwrap();
                },
                ServerMsg::Kill(server_id) => {
                    let client_id = self.to_client_entity_id(state, server_id);
                    self.network_entity_id_mapping.remove(&server_id);
                    state.destroy_entity(client_id).unwrap();
                },
                #[cfg(test)]
                ServerMsg::Test(_) => {}
            }
        }

        Ok(())
    }
}