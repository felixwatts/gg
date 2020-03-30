use recs::Ecs;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::component::Dead;
use crate::system::system::System;
use crate::err::GgResult;
use ggez::Context;

pub struct Engine{
    state: Ecs,
    systems: Vec<Box<dyn System>>,
}

impl Engine {

    pub fn new(systems: Vec<Box<dyn System>>, context: &mut Context) -> GgResult<Engine> {
        let mut engine = Engine{
            state: Ecs::new(),
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

        self.teardown_dead_entities(context)?;

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

    fn teardown_dead_entities(&mut self, context: &Context) -> GgResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        self.state.collect_with(&filter, &mut dead_entities);
        for entity in dead_entities.iter() {
            self.teardown_entity(context, *entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, context: &Context, entity: recs::EntityId) -> GgResult {
        // its possible for an entity in an Owns list to have been previously removed
        if !self.state.exists(entity) {
            return Ok(())
        }

        if let Ok(owns) = self.state.get::<Owns>(entity) {
            for &owned_entity in owns.0.iter() {
                self.teardown_entity(context, owned_entity)?;
            }
        }

        for system in self.systems.iter_mut() {
            system.teardown_entity(entity, &mut self.state, context)?;
        }

        self.state.destroy_entity(entity).unwrap();

        Ok(())
    }
}