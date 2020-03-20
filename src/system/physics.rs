use nphysics2d::object::BodyPartHandle;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::{DefaultJointConstraintSet, RevoluteConstraint};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use recs::{Ecs, EntityId};
use nalgebra::{Vector2};
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
        self.init_colliders(ecs)?;
        self.init_revolute_joints(ecs)?;

        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        self.refresh_bodies(ecs)?;
        self.refresh_joints(ecs)?;
        self.refresh_overlapping(ecs)?;

        Ok(())
    }

    pub fn teardown_entity(&mut self, ecs: &mut recs::Ecs, entity: recs::EntityId) -> ggez::GameResult {
        if let Ok(revolute_joint) = ecs.borrow::<RevoluteJoint>(entity) {
            println!("remove joint");
            self.joint_constraints.remove(revolute_joint.0);
        }

        if let Ok(collider) = ecs.borrow::<Collider>(entity) {
            self.colliders.remove(collider.0);
        }

        if let Ok(body) = ecs.borrow::<Body>(entity) {
            println!("remove body");
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
            let body = param.0.build();
            println!("add body");
            let body_handle = self.bodies.insert(body);
            ecs.unset::<InitBody>(entity).unwrap(); // todo better error handling
            ecs.set(entity, Body(body_handle)).unwrap(); // todo better error handling
        }
        Ok(())
    }

    fn init_colliders(&mut self, ecs: &mut recs::Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitCollider, Body);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : Body = ecs.get(entity).unwrap();
            let param: &InitCollider = ecs.borrow(entity).unwrap();
            let mut collider = param.0.build(BodyPartHandle(body.0, 0));
            collider.set_user_data(Some(Box::new(entity)));
            let collider_handle = self.colliders.insert(collider);
            ecs.unset::<InitCollider>(entity).unwrap(); // todo better error handling
            ecs.set(entity, Collider(collider_handle)).unwrap(); // todo better error handling
        }
        Ok(())
    }

    fn init_revolute_joints(&mut self, ecs: &mut recs::Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitRevoluteJoint);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let param: InitRevoluteJoint = ecs.get(entity).unwrap();
            if let Ok(body1) = ecs.get::<Body>(param.end1) {
                if let Ok(body2) = ecs.get::<Body>(param.end2) {

                    let revolute_constraint = RevoluteConstraint::new(
                        BodyPartHandle(body1.0, 0),
                        BodyPartHandle(body2.0, 0),
                        param.anchor1,
                        param.anchor2,
                    );
            
                    println!("add joint");
                    let joint_handle = self.joint_constraints.insert(revolute_constraint);
                    ecs.unset::<InitRevoluteJoint>(entity).unwrap();
                    ecs.set(entity, RevoluteJoint(joint_handle)).unwrap();
                }
            }            
        }
        Ok(())
    }

    fn refresh_bodies(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body, Physical);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body_component: Body = ecs.get(entity).unwrap();
            let physical_component: &mut Physical = ecs.borrow_mut(entity).unwrap();
            let rigid_body = self.bodies.rigid_body(body_component.0).unwrap();
            let pos = rigid_body.position();
            let loc = pos.translation.vector;
            physical_component.location = [loc.x, loc.y].into();
            physical_component.orientation = pos.rotation.angle();
        };

        Ok(())
    }

    fn refresh_joints(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(RevoluteJoint, Physical);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let joint_component: RevoluteJoint = ecs.get(entity).unwrap();
            let physical_component: &mut Physical = ecs.borrow_mut(entity).unwrap();
            let (anchor1, anchor2) = self.joint_constraints.get(joint_component.0).unwrap().anchors();
            let body1 = self.bodies.rigid_body(anchor1.0).unwrap();
            let body2 = self.bodies.rigid_body(anchor2.0).unwrap();
            let pos1 = body1.position().translation.vector;
            let pos2 = body2.position().translation.vector;
            let offset = pos2 - pos1;

            // not sure why this is necessary, probably something to do with
            // Matrix::angle giving smallest angle, not clockwise angle
            let orientation = if offset.x < 0.0 {
                nalgebra::Matrix::angle(&Vector2::y(), &offset)
            } else {
                nalgebra::Matrix::angle(&-Vector2::y(), &offset)
            };

            let length = offset.norm();

            physical_component.location = pos1 + (offset / 2.0);
            physical_component.orientation = orientation;
            physical_component.size.y = length;
        };

        Ok(())
    }

    fn refresh_overlapping(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Collider, Overlapping);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let collider_component: Collider = ecs.get(entity).unwrap();
            let overlapping_component: &mut Overlapping = ecs.borrow_mut(entity).unwrap();
            overlapping_component.0.clear();

            if let Some(overlapping_colliders) = self
                .geometrical_world
                .colliders_in_proximity_of(&self.colliders, collider_component.0) {
                    for (_, overlapping_collider) in overlapping_colliders {
                        let overlapping_entity : recs::EntityId = *overlapping_collider
                            .user_data()
                            .unwrap()
                            .downcast_ref()
                            .unwrap();
                        overlapping_component.0.push(overlapping_entity);
                    }
            }
        };

        Ok(())
    }
}