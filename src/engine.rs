use crate::network::TxChannel;
use crate::network::RxChannel;
use crate::network::NoNetwork;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::component::{TxQueue, RxQueue};
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::state::State;
use crate::component::Dead;
use crate::system::system::System;
use crate::err::GgResult;
use ggez::Context;

pub struct Engine<TNetwork>{
    state: State,
    systems: Vec<Box<dyn System<TNetwork>>>,
}

pub fn new_local(context: &mut ggez::Context) -> GgResult<Engine<NoNetwork>> {

    let mut engine = Engine::<NoNetwork>{
        state: State{
            ecs: recs::Ecs::new(),
            tx_queue: None,
            rx_queue: None,
        },
        systems: vec![
            Box::new(crate::system::gorilla::GorillaSystem{}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ]
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

pub fn new_client<TNetwork>(context: &mut ggez::Context) -> GgResult<Engine<TNetwork>>
    where TNetwork: RxChannel<ServerMsg> + TxChannel<ClientMsg> {

    let mut ecs = recs::Ecs::new();

    let tx_queue = ecs.create_entity();
    ecs.set(tx_queue, TxQueue::<ClientMsg>(vec![])).unwrap();

    let rx_queue = ecs.create_entity();
    ecs.set(rx_queue, RxQueue::<ServerMsg>(vec![])).unwrap();

    let mut engine = Engine::<TNetwork>{
        state: State{
            ecs,
            tx_queue: Some(tx_queue),
            rx_queue: Some(rx_queue),
        },
        systems: vec![
            Box::new(crate::system::client::ClientSystem::new()),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ]
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

pub fn new_server<TNetwork>(context: &mut ggez::Context) -> GgResult<Engine<TNetwork>> {
    let mut ecs = recs::Ecs::new();

    let tx_queue = ecs.create_entity();
    ecs.set(tx_queue, TxQueue::<ServerMsg>(vec![])).unwrap();

    let rx_queue = ecs.create_entity();
    ecs.set(rx_queue, RxQueue::<ClientMsg>(vec![])).unwrap();

    let mut engine = Engine::<TNetwork>{
        state: State{
            ecs,
            tx_queue: Some(tx_queue),
            rx_queue: Some(rx_queue)
        },
        systems: vec![
            Box::new(crate::system::server::ServerSystem{}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{}),
        ]
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

impl<TNetwork> Engine<TNetwork> {

    pub fn update(
        &mut self, 
        context: &mut Context, 
        network: &mut TNetwork) -> GgResult {

        // self.read_network_messages(network)?;

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context, network)?;
        }

        self.teardown_dead_entities()?;

        // self.write_network_messages(network)?;

        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> GgResult {

        for system in self.systems.iter_mut() {
            system.draw(&mut self.state, context)?;
        }

        Ok(())
    }

    pub fn key_down_event(
        &mut self,
        context: &mut Context,
        network: &mut TNetwork,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        for system in self.systems.iter_mut() {
            system.key_down(&mut self.state, context, network, keycode, keymod, repeat);
        }
    }

    pub fn key_up_event(
        &mut self, 
        context: &mut Context, 
        network: &mut TNetwork,
        keycode: KeyCode, 
        keymod: KeyMods) {
        for system in self.systems.iter_mut() {
            system.key_up(&mut self.state, context, network, keycode, keymod);
        }
    }

    fn teardown_dead_entities(&mut self) -> GgResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        self.state.ecs.collect_with(&filter, &mut dead_entities);
        for entity in dead_entities.iter() {
            self.teardown_entity(*entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: recs::EntityId) -> GgResult {
        // its possible for an entity in an Owns list to have been previously removed
        if !self.state.ecs.exists(entity) {
            return Ok(())
        }

        if let Ok(owns) = self.state.ecs.get::<Owns>(entity) {
            for &owned_entity in owns.0.iter() {
                self.teardown_entity(owned_entity)?;
            }
        }

        for system in self.systems.iter_mut() {
            system.teardown_entity(entity, &mut self.state)?;
        }

        self.state.ecs.destroy_entity(entity).unwrap();

        Ok(())
    }

    // fn read_network_messages(&mut self, rx: &mut dyn RxChannel<TRx>) -> GgResult {
    //     if let Some(rx_queue) = self.state.rx_queue {
    //         let mut buffer = vec![];
    //         rx.dequeue(&mut buffer)?;
    
    //         self.state.ecs.set(rx_queue, RxQueue(buffer)).unwrap();
    //     }

    //     Ok(())
    // }

    // fn write_network_messages(&mut self, tx: &mut dyn TxChannel<TTx>)-> GgResult {
    //     if let Some(tx_queue) = self.state.tx_queue {
    //         let to_tx = self.state.ecs.set(tx_queue, TxQueue(vec![])).unwrap().unwrap();
    //         for msg in to_tx.0 {
    //             tx.enqueue(msg)?;
    //         }
    //     }

    //     Ok(())
    // }
}