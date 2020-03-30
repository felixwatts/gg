use recs::Ecs;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::component::Focus;
use crate::network::ServerMsg;
use crate::err::GgResult;
use recs::EntityId;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use crate::system::system::System;
use crate::network::ClientMsg;
use std::collections::HashMap;

pub struct ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg> {
    server: TNetwork,
    network_entity_id_mapping: HashMap<u64, EntityId>
}

impl<TNetwork> ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg> {

    pub fn new(server: TNetwork) -> ClientSystem<TNetwork> {
        ClientSystem{
            server,
            network_entity_id_mapping: HashMap::<u64, EntityId>::new()
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

    fn tx_key_state(
        &mut self,
        keycode: KeyCode, 
        context: &mut Context) 
        where TNetwork: TxChannel::<ClientMsg> {
        match keycode {
            KeyCode::Space | KeyCode::Return =>
            {
                let key_state = [
                    ggez::input::keyboard::is_key_pressed(context, KeyCode::Space),
                    ggez::input::keyboard::is_key_pressed(context, KeyCode::Return),
                ];
                self.server.enqueue(ClientMsg::ButtonStateChange(key_state)).unwrap();
            },
            _ => {}
        }
    }
}

impl<TNetwork> System for ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg> {
    fn key_down(
        &mut self,
        _: &mut Ecs,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool) {
            if repeat { return; }
            self.tx_key_state(keycode, context);
    }

    fn key_up(
        &mut self,
        _: &mut Ecs,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods) {            
            self.tx_key_state(keycode, context);
    }

    fn update(
        &mut self, 
        state: &mut Ecs, 
        _: &Context) -> GgResult {

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