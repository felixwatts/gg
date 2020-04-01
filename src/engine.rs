use recs::Ecs;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use crate::component::Owns;
use crate::component::Dead;
use crate::system::system::System;
use crate::err::GgResult;

pub struct Engine<TContext>{
    state: Ecs,
    systems: Vec<Box<dyn System<TContext>>>,
}

impl<TContext> Engine<TContext> {

    pub fn new(systems: Vec<Box<dyn System<TContext>>>, init_systems: Option<Vec<Box<dyn System<TContext>>>>, context: &mut TContext) -> GgResult<Engine<TContext>> {
        let mut engine = Engine{
            state: Ecs::new(),
            systems
        };

        if let Some(mut systems) = init_systems {
            for mut system in systems.drain(..) {
                system.init(&mut engine.state, context)?;
            }
        }

        for system in engine.systems.iter_mut() {
            system.init(&mut engine.state, context)?;
        }

        Ok(engine)
    }

    pub fn update(
        &mut self, 
        context: &mut TContext) -> GgResult {

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context)?;
        }

        self.teardown_dead_entities(context)?;

        Ok(())
    }

    pub fn draw(&mut self, context: &mut TContext) -> GgResult {

        for system in self.systems.iter_mut() {
            system.draw(&mut self.state, context)?;
        }

        Ok(())
    }

    pub fn key_down_event(
        &mut self,
        context: &mut TContext,
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
        context: &mut TContext, 
        keycode: KeyCode, 
        keymod: KeyMods) {
        for system in self.systems.iter_mut() {
            system.key_up(&mut self.state, context, keycode, keymod);
        }
    }

    fn teardown_dead_entities(&mut self, context: &TContext) -> GgResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        self.state.collect_with(&filter, &mut dead_entities);
        for entity in dead_entities.iter() {
            self.teardown_entity(context, *entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, context: &TContext, entity: recs::EntityId) -> GgResult {
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