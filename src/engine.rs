use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::state::State;
use crate::component::Dead;
use crate::system::system::System;
use ggez::GameResult;
use ggez::Context;

pub struct Engine{
    state: State,
    systems: Vec<Box<dyn System>>
}

pub fn new_client(context: &mut ggez::Context) -> GameResult<Engine> {
    let mut engine = Engine{
        state: State{
            ecs: recs::Ecs::new()
        },
        systems: vec![
            Box::new(crate::system::render::RenderSystem::new(context)?),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::client::ClientSystem{})
        ]
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

pub fn new_server(context: &mut ggez::Context) -> GameResult<Engine> {
    let mut engine = Engine{
        state: State{
            ecs: recs::Ecs::new()
        },
        systems: vec![
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{})
        ]
    };

    for system in engine.systems.iter_mut() {
        system.init(&mut engine.state, context)?;
    }

    Ok(engine)
}

impl Engine {

    pub fn update<TTx, TRx> (
        &mut self, 
        context: &mut Context, 
        tx: &dyn TxChannel<TTx>, 
        rx: &mut dyn RxChannel<TRx>) -> ggez::GameResult where TRx: 'static {

        self.read_network_messages(rx);

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context)?;
        }

        self.teardown_dead_entities()?;

        self.write_network_messages(tx);

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

    fn read_network_messages<TRx>(&self, rx: &mut dyn RxChannel<TRx>) where TRx : 'static {
        let mut buffer = vec![];
        rx.dequeue(&mut buffer);
    
        for msg in buffer{
            // let m = msg.clone();
            let msg_entity = self.state.ecs.create_entity();
            self.state.ecs.set(msg_entity, msg).unwrap();
        }
    }

    fn write_network_messages<TTx>(&self, tx: &dyn TxChannel<TTx>){
        // TODO
    }
}