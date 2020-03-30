use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::state::State;
use crate::component::Dead;
use crate::system::system::System;
use crate::err::GgResult;
use ggez::Context;

pub struct Engine{
    state: State,
    systems: Vec<Box<dyn System>>,
}

impl Engine {

    pub fn new(systems: Vec<Box<dyn System>>, context: &mut Context) -> GgResult<Engine> {
        let mut engine = Engine{
            state: State{
                ecs: recs::Ecs::new()
            },
            systems
        };

        for system in engine.systems.iter_mut() {
            system.init(&mut engine.state, context)?;
        }

        Ok(engine)
    }

    pub fn update(
        &mut self, 
        context: &mut Context) -> GgResult {

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context)?;
        }

        self.teardown_dead_entities()?;

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
}