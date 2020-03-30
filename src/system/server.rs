use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::network::Server;
use crate::component::Focus;
use recs::EntityId;
use crate::component::Sprite;
use crate::component::body::Body;
use crate::component::Network;
use crate::component::Gorilla;
use crate::network::{ClientMsg, ServerMsg};
use crate::err::GgResult;
use ggez::Context;
use crate::state::State;
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
            let client_component = ecs.borrow_mut::<Client<TNetwork>>(client_entity).unwrap();
            client_component.0.enqueue(msg.clone())?;
        }

        Ok(())
    }
}

impl<TServer, TNetwork> System for ServerSystem<TServer, TNetwork>  where TServer: Server<TNetwork>, TNetwork: 'static + TxChannel<ServerMsg> + RxChannel<ClientMsg> {

    fn update(&mut self, state: &mut State, _: &Context) -> GgResult {

        // process new clients
        self.new_client_buffer.clear();
        self.server.get_new_clients(&mut self.new_client_buffer);
        for new_client in self.new_client_buffer.drain(..) {
            let gorilla = crate::system::gorilla::spawn_gorilla(&mut state.ecs, [0.0, 0.0].into())?;
            state.ecs.set(gorilla, Client(new_client))?;
        }

        // read and apply user input messages from clients
        self.entity_buffer_1.clear();
        state.ecs.collect_with(&component_filter!(Client<TNetwork>), &mut self.entity_buffer_1);
        for &client_entity in self.entity_buffer_1.iter() {
            let client_component = state.ecs.borrow_mut::<Client<TNetwork>>(client_entity).unwrap();
            self.msg_buffer.clear();
            client_component.0.dequeue(&mut self.msg_buffer)?;

            for msg in self.msg_buffer.iter() {
                match msg {
                    ClientMsg::ButtonStateChange(m) => {
                        let gorilla_component = state.ecs.borrow_mut::<Gorilla>(client_entity).unwrap();
                        gorilla_component.button_state = *m;
                    }
                }
            }
        }

        // TODO only on key frames, i.e. periodically and after user input
        // broadcast all network entities
        self.entity_buffer_2.clear();
        state.ecs.collect_with(&component_filter!(Network), &mut self.entity_buffer_2);
        for &network_entity in self.entity_buffer_2.iter() {
            if let Ok(body) = state.ecs.get::<Body>(network_entity) {
                let msg = ServerMsg::SetBody(network_entity.get_id_number(), body);
                self.broadcast(&mut state.ecs, &self.entity_buffer_1, msg)?;
            }
            
            if let Ok(sprite) = state.ecs.get::<Sprite>(network_entity) {
                let msg = ServerMsg::SetSprite(network_entity.get_id_number(), sprite);
                self.broadcast(&mut state.ecs, &self.entity_buffer_1, msg)?;
            }

            if let Ok(_) = state.ecs.get::<Focus>(network_entity) {
                let msg = ServerMsg::SetFocus(network_entity.get_id_number());
                self.broadcast(&mut state.ecs, &self.entity_buffer_1, msg)?;
            }
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, state: &mut State) -> GgResult {
        if state.ecs.has::<Network>(entity).unwrap() {
            let msg = ServerMsg::Kill(entity.get_id_number());
            self.entity_buffer_1.clear();
            state.ecs.collect_with(&component_filter!(Client<TNetwork>), &mut self.entity_buffer_1);
            self.broadcast(&mut state.ecs, &self.entity_buffer_1, msg)?;
        }

        Ok(())
    }
}