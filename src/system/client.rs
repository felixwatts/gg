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
use crate::system::System;
use crate::network::ClientMsg;
use std::collections::HashMap;
use crate::input::{KeyMapping};
#[cfg(test)]
use crate::network::Server;
#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use std::rc::Rc;
#[cfg(test)]
use std::cell::Cell;

pub struct ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg>{
    server: TNetwork,
    network_entity_id_mapping: HashMap<u64, EntityId>,
    key_mapping: KeyMapping
}

impl<TNetwork> ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg>{
    pub fn new(server: TNetwork, key_mapping: KeyMapping) -> ClientSystem<TNetwork> {
        ClientSystem{
            server,
            network_entity_id_mapping: HashMap::<u64, EntityId>::new(),
            key_mapping
        }
    }

    fn get_client_entity_id(&mut self, state: &mut Ecs, server_id: u64) -> EntityId {
        if let Some(client_id) = self.network_entity_id_mapping.get(&server_id) {
            *client_id
        } else {
            let client_id = state.create_entity();
            self.network_entity_id_mapping.insert(server_id, client_id);

            client_id
        }
    }
}

impl<TNetwork, TContext> System<TContext> for ClientSystem<TNetwork> where TNetwork: TxChannel<ClientMsg> + RxChannel<ServerMsg>{
    fn key_down(
        &mut self,
        _: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool) {
            if repeat { return; }

            if let Some(&button) = self.key_mapping.get(&keycode) {
                self.server.enqueue(ClientMsg::Input(InputEvent{button, is_down: true})).unwrap();
            }
    }

    fn key_up(
        &mut self,
        _: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods) {   
            if let Some(&button) = self.key_mapping.get(&keycode) {
                self.server.enqueue(ClientMsg::Input(InputEvent{button, is_down: false})).unwrap();
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
                    let client_id = self.get_client_entity_id(state, server_id);
                    state.set(client_id, body).unwrap();
                },
                ServerMsg::SetSprite(server_id, sprite) => {
                    let client_id = self.get_client_entity_id(state, server_id);
                    state.set(client_id, sprite).unwrap();
                },
                ServerMsg::SetFocus(server_id) => {
                    let client_id = self.get_client_entity_id(state, server_id);
                    state.set(client_id, Focus).unwrap();
                },
                ServerMsg::Kill(server_id) => {
                    let client_id = self.get_client_entity_id(state, server_id);
                    self.network_entity_id_mapping.remove(&server_id);
                    state.destroy_entity(client_id).unwrap();
                },
                ServerMsg::Ping(tx_time) => {
                    self.server.enqueue(ClientMsg::Pong(tx_time))?;
                }
                #[cfg(test)]
                ServerMsg::Test(_) => {}
            }
        }

        Ok(())
    }
}

#[test]
fn test_ping_pong() {
    // build a simulated network
    let time = Rc::new(Cell::new(Duration::from_millis(0u64)));
    let mut server = crate::network::sim::SimServer::new(Duration::from_millis(0), Rc::clone(&time));
    let network = server.connect();
    
    // create a new ClientSystem to test and connect it to the network
    let mut subject = ClientSystem::new(network, crate::input::default_key_mapping());
    
    // send the ClientSystem a Ping message
    let mut new_clients = vec![];
    server.get_new_clients(&mut new_clients);
    new_clients[0].enqueue(ServerMsg::Ping(std::time::Duration::from_millis(42u64))).unwrap();

    // Step the ClientSystem so that it can process the Ping message 
    let mut state = Ecs::new();
    subject.update(&mut state, &0).unwrap();

    // Check that it correctly responded with a Pong message
    let mut client_msgs = vec![];
    new_clients[0].dequeue(&mut client_msgs).unwrap();
    assert_eq!(1, client_msgs.len());
    assert_eq!(ClientMsg::Pong(std::time::Duration::from_millis(42u64)), client_msgs[0]);
}