use crate::network::NoMsg;
use std::marker::PhantomData;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::component::{TxQueue, RxQueue};
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::state::State;
use crate::component::Dead;
use crate::system::system::System;
use ggez::GameResult;
use ggez::Context;

pub struct Engine<TTx, TRx>{
    state: State,
    systems: Vec<Box<dyn System>>,
    
    phantom1: PhantomData<TTx>,
    phantom2: PhantomData<TRx>
}

pub fn new_local(context: &mut ggez::Context) -> GameResult<Engine<NoMsg, NoMsg>> {

    let mut engine = Engine::<NoMsg, NoMsg>{
        state: State{
            ecs: recs::Ecs::new(),
            tx_queue: None,
            rx_queue: None,
        },
        systems: vec![
            Box::new(crate::system::gorilla::GorillaSystem{}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ],
        phantom1: PhantomData{},
        phantom2: PhantomData{}
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

pub fn new_client(context: &mut ggez::Context) -> GameResult<Engine<ClientMsg, ServerMsg>> {

    let mut ecs = recs::Ecs::new();

    let tx_queue = ecs.create_entity();
    ecs.set(tx_queue, TxQueue::<ClientMsg>(vec![])).unwrap();

    let rx_queue = ecs.create_entity();
    ecs.set(rx_queue, RxQueue::<ServerMsg>(vec![])).unwrap();

    let mut engine = Engine::<ClientMsg, ServerMsg>{
        state: State{
            ecs,
            tx_queue: Some(tx_queue),
            rx_queue: Some(rx_queue),
        },
        systems: vec![
            Box::new(crate::system::client::ClientSystem::new()),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ],
        phantom1: PhantomData{},
        phantom2: PhantomData{}
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

pub fn new_server(context: &mut ggez::Context) -> GameResult<Engine<ServerMsg, ClientMsg>> {
    let mut ecs = recs::Ecs::new();

    let tx_queue = ecs.create_entity();
    ecs.set(tx_queue, TxQueue::<ServerMsg>(vec![])).unwrap();

    let rx_queue = ecs.create_entity();
    ecs.set(rx_queue, RxQueue::<ClientMsg>(vec![])).unwrap();

    let mut engine = Engine::<ServerMsg, ClientMsg>{
        state: State{
            ecs,
            tx_queue: Some(tx_queue),
            rx_queue: Some(rx_queue)
        },
        systems: vec![
            Box::new(crate::system::server::ServerSystem{}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{}),
        ],
        phantom1: PhantomData{},
        phantom2: PhantomData{}
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

impl<TTx, TRx> Engine<TTx, TRx> where TRx: 'static, TTx: 'static {

    pub fn update (
        &mut self, 
        context: &mut Context, 
        tx: &mut dyn TxChannel<TTx>, 
        rx: &mut dyn RxChannel<TRx>) -> ggez::GameResult {

        self.read_network_messages(rx)?;

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context)?;
        }

        self.teardown_dead_entities()?;

        self.write_network_messages(tx)?;

        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> ggez::GameResult {

        for system in self.systems.iter_mut() {
            system.draw(&mut self.state, context)?;
        }

        Ok(())
    }

    pub fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        for system in self.systems.iter_mut() {
            system.key_down(&mut self.state, context, keycode, keymod, repeat);
        }
    }

    pub fn key_up_event(
        &mut self, 
        context: &mut Context, 
        keycode: KeyCode, 
        keymod: KeyMods) {
        for system in self.systems.iter_mut() {
            system.key_up(&mut self.state, context, keycode, keymod);
        }
    }

    fn teardown_dead_entities(&mut self) -> GameResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        self.state.ecs.collect_with(&filter, &mut dead_entities);
        for entity in dead_entities.iter() {
            self.teardown_entity(*entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: recs::EntityId) -> ggez::GameResult {
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

    fn read_network_messages(&mut self, rx: &mut dyn RxChannel<TRx>) -> GameResult {
        if let Some(rx_queue) = self.state.rx_queue {
            let mut buffer = vec![];
            rx.dequeue(&mut buffer)?;
    
            self.state.ecs.set(rx_queue, RxQueue(buffer)).unwrap();
        }

        Ok(())
    }

    fn write_network_messages(&mut self, tx: &mut dyn TxChannel<TTx>)-> GameResult {
        if let Some(tx_queue) = self.state.tx_queue {
            let to_tx = self.state.ecs.set(tx_queue, TxQueue(vec![])).unwrap().unwrap();
            for msg in to_tx.0 {
                tx.enqueue(msg)?;
            }
        }

        Ok(())
    }
}