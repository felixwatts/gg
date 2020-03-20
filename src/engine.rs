use crate::component::Owns;
use crate::component::Dead;
use crate::entity::spawn_anchor;
use crate::system::system::System;
use ggez::event::EventHandler;
use ggez::GameResult;
use ggez::Context;

pub struct Engine {
    ecs: recs::Ecs,
    systems: Vec<Box<dyn System>>
}

impl EventHandler for Engine {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        for system in self.systems.iter_mut() {
            system.update(&mut self.ecs, context)?;
        }

        self.teardown_dead_entities()?;

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {

        for system in self.systems.iter_mut() {
            system.draw(&mut self.ecs, context)?;
        }

        Ok(())
    }
}

impl Engine {
    pub fn new(context: &mut ggez::Context) -> GameResult<Engine> {
        let mut engine = Engine{
            ecs: recs::Ecs::new(),
            systems: vec![
                Box::new(crate::system::render::RenderSystem::new(context)?),
                Box::new(crate::system::physics::PhysicsSystem::new()),
                Box::new(crate::system::gorilla::GorillaSystem::new())
            ]
        };

        for system in engine.systems.iter_mut() {
            system.init(&mut engine.ecs, context)?;
        }

        spawn_anchor(&mut engine.ecs, [-1.0, -1.0].into())?;
        spawn_anchor(&mut engine.ecs, [-1.0, 1.0].into())?;
        spawn_anchor(&mut engine.ecs, [0.0, 0.0].into())?;
        spawn_anchor(&mut engine.ecs, [1.0, -1.0].into())?;
        spawn_anchor(&mut engine.ecs, [1.0, 1.0].into())?;

        Ok(engine)
    }

    fn teardown_dead_entities(&mut self) -> GameResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        self.ecs.collect_with(&filter, &mut dead_entities);
        for &entity in dead_entities.iter() {
            self.teardown_entity(entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, entity: recs::EntityId) -> ggez::GameResult {
        // its possible for an entity in an Owns list to have been previously removed
        if !self.ecs.exists(entity) {
            return Ok(())
        }

        if let Ok(owns) = self.ecs.get::<Owns>(entity) {
            for owned_entity in owns.0 {
                self.teardown_entity(owned_entity)?;
            }
        }

        for system in self.systems.iter_mut() {
            system.teardown_entity(entity, &mut self.ecs)?;
        }

        self.ecs.destroy_entity(entity).unwrap();

        Ok(())
    }
}