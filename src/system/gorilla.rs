use crate::component::physics::Physical;
use crate::component::lifecycle::Owns;
use crate::component::physics::InitRevoluteJoint;
use crate::component::physics::Overlapping;
use ggez::GameResult;
use crate::component::lifecycle::Dead;
use recs::EntityId;
use recs::Ecs;
use ggez::input::keyboard::KeyCode;
use crate::component::gorilla::Gorilla;

pub struct GorillaSystem {
}

impl GorillaSystem {
    pub fn new() -> GorillaSystem {
        GorillaSystem {
        }
    }

    pub fn step(
        &mut self, 
        ecs: &mut Ecs, 
        context: &ggez::Context) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Gorilla, Owns, Physical);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {

            if ecs.get::<Physical>(entity).unwrap().location.y < -10.0 {
                ecs.set(entity, Dead).unwrap();
            }

            if ggez::input::keyboard::is_key_pressed(context, KeyCode::Space) {
                self.try_add_rope(ecs, entity)?;
            } else {
                self.try_remove_rope(ecs, entity)?;
            }

        }
        Ok(())
    }

    pub fn teardown_entity(&mut self, ecs: &mut Ecs, entity: EntityId) ->  GameResult {
        if let Ok(_) = ecs.get::<Gorilla>(entity) {
            crate::entity::gorilla::spawn_gorilla(ecs, [-2.5, 10.0].into())?;
        }
        Ok(())
    }

    fn try_add_rope(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId) -> GameResult {
        let gorilla : Gorilla = ecs.get(entity).unwrap();
        if let None = gorilla.rope {
            let overlapping: &Overlapping = ecs.borrow(entity).unwrap();
            if let Some(&closest_anchor) = overlapping.0.first() {

                let p1 = ecs.get::<Physical>(entity).unwrap().location;
                let p2 = ecs.get::<Physical>(closest_anchor).unwrap().location;
                let offset = p2 - p1;

                let rope = ecs.create_entity();
                ecs.set(rope, InitRevoluteJoint{
                    end1: closest_anchor,
                    end2: entity,
                    anchor1: nalgebra::Point2::new(0.0, 0.0),
                    anchor2: offset.into(),
                }).unwrap();
                crate::entity::with_physical(ecs, rope, [0.1, 0.0].into())?;
                crate::entity::with_sprite(ecs, rope, [0.0, 1.0, 1.0, 1.0].into())?;
                ecs.set(entity, Gorilla{ rope: Some(rope) }).unwrap();
                let owns : &mut Owns = ecs.borrow_mut(entity).unwrap();
                owns.0.push(rope);
            };
        }
        
        Ok(())
    }

    fn try_remove_rope(
        &mut self, 
        ecs: &mut Ecs, 
        entity: EntityId) -> GameResult {
            let gorilla : Gorilla = ecs.get(entity).unwrap();
            if let Some(rope) = gorilla.rope {
                ecs.set(rope, Dead).unwrap();
                ecs.set(entity, Gorilla{ rope: None }).unwrap();
                let owns : &mut Owns = ecs.borrow_mut(entity).unwrap();
                owns.0.clear(); // TODO remove the actual rope
            }
            Ok(())
        }
}