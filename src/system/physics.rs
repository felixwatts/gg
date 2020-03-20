use crate::component::InitSensor;
use crate::component::Sensor;
use crate::component::Overlapping;
use crate::component::InitRevoluteJoint;
use crate::component::InitCollider;
use crate::component::InitBody;
use crate::component::Body;
use crate::component::Collider;
use crate::component::RevoluteJoint;
use crate::component::Sprite;
use ggez::GameResult;
use ggez::Context;
use crate::system::system::System;
use nphysics2d::object::BodyPartHandle;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::{DefaultJointConstraintSet, RevoluteConstraint};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use recs::{Ecs, EntityId};
use nalgebra::{Vector2};


pub struct PhysicsSystem {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>
}

impl System for PhysicsSystem {
    fn update(&mut self, ecs: &mut Ecs, _: &Context) -> GameResult {
        self.init_bodies(ecs)?;
        self.init_colliders(ecs)?;
        self.init_sensors(ecs)?;
        self.init_revolute_joints(ecs)?;

        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        self.refresh_body_sprites(ecs)?;
        self.refresh_joint_sprites(ecs)?;
        self.refresh_overlapping(ecs)?;

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, ecs: &mut Ecs) -> GameResult {
        if let Ok(revolute_joint) = ecs.borrow::<RevoluteJoint>(entity) {
            self.joint_constraints.remove(revolute_joint.0);
        }

        if let Ok(collider) = ecs.borrow::<Collider>(entity) {
            self.colliders.remove(collider.0);
        }

        if let Ok(sensor) = ecs.borrow::<Sensor>(entity) {
            self.colliders.remove(sensor.0);
        }

        if let Ok(body) = ecs.borrow::<Body>(entity) {
            self.bodies.remove(body.0);
        }

        Ok(())
    }
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new()
        }
    }

    fn init_bodies(&mut self, ecs: &mut recs::Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitBody);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let param: &InitBody = ecs.borrow(entity).unwrap();
            let body = param.0.build();
            let body_handle = self.bodies.insert(body);
            ecs.unset::<InitBody>(entity).unwrap();
            ecs.set(entity, Body(body_handle)).unwrap();
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
            ecs.set(entity, Collider(collider_handle)).unwrap();
            ecs.unset::<InitCollider>(entity).unwrap();
        }
        Ok(())
    }

    fn init_sensors(&mut self, ecs: &mut recs::Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitSensor, Body);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : Body = ecs.get(entity).unwrap();
            let param: &InitSensor = ecs.borrow(entity).unwrap();
            let mut collider = param.0.build(BodyPartHandle(body.0, 0));
            collider.set_user_data(Some(Box::new(entity)));
            let collider_handle = self.colliders.insert(collider);
            ecs.set(entity, Sensor(collider_handle)).unwrap();
            ecs.unset::<InitSensor>(entity).unwrap();
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
            
                    let joint_handle = self.joint_constraints.insert(revolute_constraint);
                    ecs.unset::<InitRevoluteJoint>(entity).unwrap();
                    ecs.set(entity, RevoluteJoint(joint_handle)).unwrap();
                }
            }            
        }
        Ok(())
    }

    fn refresh_body_sprites(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body, Sprite);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body_component: Body = ecs.get(entity).unwrap();
            let body = self.bodies.rigid_body(body_component.0).unwrap();
            let orientation = body.position().rotation.angle();
            let loc = body.position().translation.vector;

            let sprite: &mut Sprite = ecs.borrow_mut(entity).unwrap();                        
            sprite.location = [loc.x, loc.y].into();
            sprite.orientation = orientation;
        };

        Ok(())
    }

    fn refresh_joint_sprites(&mut self, ecs: &mut Ecs) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(RevoluteJoint, Sprite);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let joint_component: RevoluteJoint = ecs.get(entity).unwrap();
            let physical_component: &mut Sprite = ecs.borrow_mut(entity).unwrap();
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
        let filter = component_filter!(Sensor, Overlapping);
        ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            if ecs.has::<Overlapping>(entity).unwrap() {
                let collider_component: Sensor = ecs.get(entity).unwrap();
                let mut overlapping_component = Overlapping(vec![]);
    
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
                        overlapping_component.0.sort_by(|&a, &b| {
                            let da = self.distance_between(ecs, a, entity);
                            let db = self.distance_between(ecs, b, entity);
                            if da > db { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
                        });
                }
    
                ecs.set(entity, overlapping_component).unwrap();
            }
        };

        Ok(())
    }

    fn distance_between(&self, ecs: &mut recs::Ecs, e1: EntityId, e2: EntityId) -> f32 {
        let e1_body = self.bodies.rigid_body(ecs.borrow::<Body>(e1).unwrap().0).unwrap();
        let e2_body = self.bodies.rigid_body(ecs.borrow::<Body>(e2).unwrap().0).unwrap();

        let e1_loc = e1_body.position().translation.vector;
        let e2_loc = e2_body.position().translation.vector;

        let offset = e1_loc - e2_loc;

        offset.norm()
    }
}