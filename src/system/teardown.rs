use recs::Ecs;
use crate::component::lifecycle::Owns;

pub struct Teardown;

impl Teardown{
    pub fn new() -> Teardown {
        Teardown{}
    }

    pub fn step(&mut self, ecs: &mut Ecs, physics_system: &mut crate::system::physics::Physics) -> ggez::GameResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(crate::component::lifecycle::Teardown);
        ecs.collect_with(&filter, &mut dead_entities);
        for &entity in dead_entities.iter() {
            self.teardown_entity(ecs, physics_system, entity)?;
        }

        Ok(())
    }

    fn teardown_entity(&mut self, ecs: &mut Ecs, physics_system: &mut crate::system::physics::Physics, entity: recs::EntityId) -> ggez::GameResult {
        if let Ok(owns) = ecs.get::<Owns>(entity) {
            for owned_entity in owns.0 {
                self.teardown_entity(ecs, physics_system, owned_entity)?;
            }
        }

        physics_system.teardown_entity(ecs, entity)?;

        ecs.destroy_entity(entity).unwrap(); // TODO better error handling

        Ok(())
    }
}