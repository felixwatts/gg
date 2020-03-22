use crate::component::Owns;
use crate::state::State;
use crate::component::Dead;
use crate::system::system::System;
use ggez::event::EventHandler;
use ggez::GameResult;
use ggez::Context;

pub struct Engine {
    state: State,
    systems: Vec<Box<dyn System>>
}

impl EventHandler for Engine {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        for system in self.systems.iter_mut() {
            system.update(&mut self.state, context)?;
        }

        self.teardown_dead_entities()?;

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {

        for system in self.systems.iter_mut() {
            system.draw(&mut self.state, context)?;
        }

        Ok(())
    }
}

impl Engine {
    pub fn new(context: &mut ggez::Context) -> GameResult<Engine> {
        let mut engine = Engine{
            state: State{
                ecs: recs::Ecs::new()
            },
            systems: vec![
                Box::new(crate::system::render::RenderSystem::new(context)?),
                Box::new(crate::system::physics::PhysicsSystem{}),
                Box::new(crate::system::gorilla::GorillaSystem{})
            ]
        };

        for system in engine.systems.iter_mut() {
            system.init(&mut engine.state, context)?;
        }

        Ok(engine)
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
}