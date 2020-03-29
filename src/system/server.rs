use std::net::TcpListener;
use crate::component::Focus;
use recs::EntityId;
use crate::component::Sprite;
use crate::component::body::Body;
use crate::component::Network;
use crate::component::Gorilla;
use crate::network::{ClientMsg, ServerMsg};
use crate::component::RxQueue;
use crate::err::GgResult;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;
use crate::network::real::RealNetwork;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::component::Client;

pub struct ServerSystem {
    listener: TcpListener,
    new_client_queue: Receiver<RealNetwork::<ServerMsg, ClientMsg>>
}

impl ServerSystem {
    fn new() -> GgResult<ServerSystem> {
        let (new_client_send, new_client_recv) = channel();
        let listener = TcpListener::bind("127.0.0.1:9001")?;
        let listen_thread = std::thread::spawn(move || {
            for stream in listener.incoming() {
                let client = RealNetwork::<ServerMsg, ClientMsg>::new(stream.unwrap());
                new_client_send.send(client);
            }
        });
        Ok(ServerSystem {
            listener, 
            new_client_queue: new_client_recv
        })
    }
}

impl System for ServerSystem {

    fn update(&mut self, state: &mut State, _: &Context) -> GgResult {

        // process new clients
        for new_client in self.new_client_queue.try_iter() {
            let gorilla = crate::system::gorilla::spawn_gorilla(&mut state.ecs, [0.0, 0.0].into())?;
            state.ecs.set(gorilla, Client(new_client))?;
        }

        // read and apply user input messages from clients
        let clients = state.ecs.get::<Client>(state.rx_queue.unwrap()).unwrap();

        // TODO try to avoid this copy of the whole vector
        let msgs = state.ecs.get::<RxQueue<ClientMsg>>(state.rx_queue.unwrap()).unwrap();
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
                let msg = ServerMsg::SetBody(network_entity.get_id_number(), body);
                network.enqueue
                crate::network::tx(state, msg);
            }
            
            if let Ok(sprite) = state.ecs.get::<Sprite>(network_entity) {
                let msg = ServerMsg::SetSprite(network_entity.get_id_number(), sprite);
                crate::network::tx(state, msg);
            }

            if let Ok(_) = state.ecs.get::<Focus>(network_entity) {
                let msg = ServerMsg::SetFocus(network_entity.get_id_number());
                crate::network::tx(state, msg);
            }
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, state: &mut State, network: &mut TNetwork) -> GgResult {
        if state.ecs.has::<Network>(entity).unwrap() {
            let msg = ServerMsg::Kill(entity.get_id_number());
            network.enqueue(msg)?;
        }

        Ok(())
    }
}