use crate::component::Dead;
use recs::Ecs;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::network::Server;
use crate::component::Focus;
use recs::EntityId;
use crate::component::Sprite;
use crate::component::body::Body;
use crate::component::Network;
use crate::component::gorilla::Gorilla;
use crate::network::{ClientMsg, ServerMsg};
use crate::err::GgResult;
use crate::system::system::System;
use crate::component::Client;

pub struct ServerSystem<TServer, TNetwork> where TServer: Server<TNetwork>, TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg> {
    server: TServer,
    new_client_buffer: Vec::<TNetwork>,
    entity_buffer_1: Vec::<EntityId>,
    entity_buffer_2: Vec::<EntityId>,
    msg_buffer: Vec::<ClientMsg>
}

impl<TServer, TNetwork> ServerSystem<TServer, TNetwork> where TServer: Server<TNetwork>, TNetwork: 'static + TxChannel<ServerMsg> + RxChannel<ClientMsg> {
    pub fn new(server: TServer) -> GgResult<ServerSystem<TServer, TNetwork>> {
        Ok(ServerSystem{
            server,
            new_client_buffer: vec![],
            entity_buffer_1: vec![],
            entity_buffer_2: vec![],
            msg_buffer: vec![]
        })
    }

    fn broadcast(&self, ecs: &mut recs::Ecs, to: &[EntityId], msg: ServerMsg) -> GgResult {
        for &client_entity in to.iter() {
            if let Ok(client_component) = ecs.borrow_mut::<Client<TNetwork>>(client_entity){
                if let Err(_) = client_component.0.enqueue(msg.clone()) {
                    self.disconnect_client(ecs, client_entity);
                }
            }
        }

        Ok(())
    }

    fn disconnect_client(&self, state: &mut Ecs, entity: EntityId) {
        state.set(entity, Dead{}).unwrap();
        state.unset::<Client::<TNetwork>>(entity).unwrap();
        println!("client #{} has disconnected", entity.get_id_number());
    }
}

impl<TServer, TNetwork, TContext> System<TContext> for ServerSystem<TServer, TNetwork>  
    where 
        TServer: Server<TNetwork>, 
        TNetwork: 'static + TxChannel<ServerMsg> + RxChannel<ClientMsg> {

    fn update(&mut self, state: &mut Ecs, _: &TContext) -> GgResult {

        // process new clients
        self.new_client_buffer.clear();
        self.server.get_new_clients(&mut self.new_client_buffer);
        for mut new_client in self.new_client_buffer.drain(..) {
            let client_entity = crate::system::gorilla::spawn_gorilla(state, [-1.5, 5.0].into(), false)?;

            let msg = ServerMsg::SetFocus(client_entity.get_id_number());
            new_client.enqueue(msg)?;

            state.set(client_entity, Client(new_client))?;
            println!("client #{} has connected", client_entity.get_id_number());

            // send current state to new client
            self.entity_buffer_2.clear();
            state.collect_with(&component_filter!(Network), &mut self.entity_buffer_2);     
            for &network_entity in self.entity_buffer_2.iter() {

                if let Ok(body) = state.get::<Body>(network_entity) {
                    let msg = ServerMsg::SetBody(network_entity.get_id_number(), body);
                    state.borrow_mut::<Client<TNetwork>>(client_entity).unwrap().0.enqueue(msg)?;
                }
                
                if let Ok(sprite) = state.get::<Sprite>(network_entity) {
                    let msg = ServerMsg::SetSprite(network_entity.get_id_number(), sprite);
                    state.borrow_mut::<Client<TNetwork>>(client_entity).unwrap().0.enqueue(msg)?;
                }

                if let Ok(_) = state.get::<Focus>(network_entity) {
                    let msg = ServerMsg::SetFocus(network_entity.get_id_number());
                    state.borrow_mut::<Client<TNetwork>>(client_entity).unwrap().0.enqueue(msg)?;
                }
            }
        }

        // read and apply user input messages from clients
        self.entity_buffer_1.clear();
        state.collect_with(&component_filter!(Client<TNetwork>), &mut self.entity_buffer_1);
        for &client_entity in self.entity_buffer_1.iter() {
            let client_component = state.borrow_mut::<Client<TNetwork>>(client_entity).unwrap();

            if let Err(_) = client_component.0.dequeue(&mut self.msg_buffer) {
                self.disconnect_client(state, client_entity);
                continue;
            }

            for msg in self.msg_buffer.drain(..) {
                match msg {
                    ClientMsg::Input(input_event) => {
                        let gorilla_component = state.borrow_mut::<Gorilla>(client_entity).unwrap();
                        gorilla_component.input_events.push(input_event);
                    },
                    #[cfg(test)]
                    ClientMsg::Test(_) => {}
                }
            }
        }

        // broadcast all network entities
        self.entity_buffer_2.clear();
        state.collect_with(&component_filter!(Network), &mut self.entity_buffer_2);        
        for &network_entity in self.entity_buffer_2.iter() {

            let is_keyframe = state.borrow_mut::<Body>(network_entity).unwrap().get_is_keyframe_and_reset();
            if !is_keyframe { continue; }   
            
            if let Ok(body) = state.get::<Body>(network_entity) {
                let msg = ServerMsg::SetBody(network_entity.get_id_number(), body);
                self.broadcast(state, &self.entity_buffer_1, msg)?;
            }
            
            if let Ok(sprite) = state.get::<Sprite>(network_entity) {
                let msg = ServerMsg::SetSprite(network_entity.get_id_number(), sprite);
                self.broadcast(state, &self.entity_buffer_1, msg)?;
            }

            if let Ok(_) = state.get::<Focus>(network_entity) {
                let msg = ServerMsg::SetFocus(network_entity.get_id_number());
                self.broadcast(state, &self.entity_buffer_1, msg)?;
            }
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, state: &mut Ecs, _: &TContext) -> GgResult {
        if state.has::<Network>(entity).unwrap() {
            let msg = ServerMsg::Kill(entity.get_id_number());
            self.entity_buffer_1.clear();
            state.collect_with(&component_filter!(Client<TNetwork>), &mut self.entity_buffer_1);
            self.broadcast(state, &self.entity_buffer_1, msg)?;
        }

        Ok(())
    }
}