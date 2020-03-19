use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use recs::{Ecs, EntityId};
use nalgebra::Vector2;
use crate::component::physics::*;

pub struct Physics {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new()
        }
    }

    pub fn step(&mut self, ecs: &mut Ecs) -> ggez::GameResult {

        self.init_bodies(ecs)?;
        self.refresh_bodies(ecs)?;

        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        Ok(())
    }

    pub fn teardown_entity(&mut self, ecs: &mut recs::Ecs, entity: recs::EntityId) -> ggez::GameResult {
        if let Ok(body) = ecs.borrow::<Body>(entity) {
            self.bodies.remove(body.0);
        }

        Ok(())
    }

    fn init_bodies(&mut self, ecs: &mut recs::Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitBody);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let param: &InitBody = ecs.borrow(entity).unwrap();
            let body = RigidBodyDesc::new()
                .translation(param.location)
                .build();
            let body_handle = self.bodies.insert(body);
            ecs.unset::<InitBody>(entity).unwrap(); // todo better error handling
            ecs.set(entity, Body(body_handle)).unwrap(); // todo better error handling
        }
        Ok(())
    }

    fn refresh_bodies(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body, Physical);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body_component: &Body = ecs.get(entity).unwrap();
            let physical_component: &mut Physical = ecs.borrow_mut(entity).unwrap();
            let rigid_body = self.bodies.rigid_body(body_component.0).expect("rigid body not found");
            let pos = rigid_body.position();
            let loc = pos.translation.vector;
            physical_component.location = [loc.x, loc.y].into();
            physical_component.orientation = pos.rotation.angle();
        };

        Ok(())
    }
}