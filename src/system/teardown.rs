use crate::component::lifecycle::Dead;
use recs::Ecs;
use crate::component::lifecycle::Owns;

pub struct Teardown;

impl Teardown{
    pub fn new() -> Teardown {
        Teardown{}
    }

    pub fn step(
        &mut self, 
        ecs: &mut Ecs, 
        physics_system: &mut crate::system::physics::Physics,
        gorilla_system: &mut crate::system::gorilla::GorillaSystem) -> ggez::GameResult {
        let mut dead_entities = vec![];
        let filter = component_filter!(Dead);
        ecs.collect_with(&filter, &mut dead_entities);
        for &entity in dead_entities.iter() {
            self.teardown_entity(ecs, physics_system, gorilla_system, entity)?;
        }

        Ok(())
    }

    fn teardown_entity(
        &mut self, 
        ecs: &mut Ecs, 
        physics_system: &mut crate::system::physics::Physics, 
        gorilla_system: &mut crate::system::gorilla::GorillaSystem, 
        entity: recs::EntityId) -> ggez::GameResult {
        if let Ok(owns) = ecs.get::<Owns>(entity) {
            for owned_entity in owns.0 {
                self.teardown_entity(ecs, physics_system, gorilla_system, owned_entity)?;
            }
        }

        physics_system.teardown_entity(ecs, entity)?;
        gorilla_system.teardown_entity(ecs, entity)?;

        ecs.destroy_entity(entity).unwrap(); // TODO better error handling

        Ok(())
    }
}