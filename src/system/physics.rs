use crate::state::State;
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
use nphysics2d::joint::RevoluteConstraint;
use recs::EntityId;
use nalgebra::{Vector2};

pub struct PhysicsSystem {
}

impl System for PhysicsSystem {
    fn update(&mut self, state: &mut State, _: &Context) -> GameResult {
        self.init_bodies(state)?;
        self.init_colliders(state)?;
        self.init_sensors(state)?;
        self.init_revolute_joints(state)?;

        state.world.mechanical_world.step(
            &mut state.world.geometrical_world,
            &mut state.world.bodies,
            &mut state.world.colliders,
            &mut state.world.joint_constraints,
            &mut state.world.force_generators
        );

        self.refresh_body_sprites(state)?;
        self.refresh_joint_sprites(state)?;
        self.refresh_overlapping(state)?;

        Ok(())
    }

    fn teardown_entity(&mut self, entity: EntityId, state: &mut State) -> GameResult {
        if let Ok(revolute_joint) = state.ecs.borrow::<RevoluteJoint>(entity) {
            state.world.joint_constraints.remove(revolute_joint.0);
        }

        if let Ok(collider) = state.ecs.borrow::<Collider>(entity) {
            state.world.colliders.remove(collider.0);
        }

        if let Ok(sensor) = state.ecs.borrow::<Sensor>(entity) {
            state.world.colliders.remove(sensor.0);
        }

        if let Ok(body) = state.ecs.borrow::<Body>(entity) {
            state.world.bodies.remove(body.0);
        }

        Ok(())
    }
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
        }
    }

    fn init_bodies(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitBody);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let param: &InitBody = state.ecs.borrow(entity).unwrap();
            let body = param.0.build();
            let body_handle = state.world.bodies.insert(body);
            state.ecs.unset::<InitBody>(entity).unwrap();
            state.ecs.set(entity, Body(body_handle)).unwrap();
        }
        Ok(())
    }

    fn init_colliders(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitCollider, Body);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : Body = state.ecs.get(entity).unwrap();
            let param: &InitCollider = state.ecs.borrow(entity).unwrap();
            let mut collider = param.0.build(BodyPartHandle(body.0, 0));
            collider.set_user_data(Some(Box::new(entity)));
            let collider_handle = state.world.colliders.insert(collider);
            state.ecs.set(entity, Collider(collider_handle)).unwrap();
            state.ecs.unset::<InitCollider>(entity).unwrap();
        }
        Ok(())
    }

    fn init_sensors(&mut self,  state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitSensor, Body);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body : Body = state.ecs.get(entity).unwrap();
            let param: &InitSensor = state.ecs.borrow(entity).unwrap();
            let mut collider = param.0.build(BodyPartHandle(body.0, 0));
            collider.set_user_data(Some(Box::new(entity)));
            let collider_handle = state.world.colliders.insert(collider);
            state.ecs.set(entity, Sensor(collider_handle)).unwrap();
            state.ecs.unset::<InitSensor>(entity).unwrap();
        }
        Ok(())
    }

    fn init_revolute_joints(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(InitRevoluteJoint);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let param: InitRevoluteJoint = state.ecs.get(entity).unwrap();
            if let Ok(body1) = state.ecs.get::<Body>(param.end1) {
                if let Ok(body2) = state.ecs.get::<Body>(param.end2) {

                    let revolute_constraint = RevoluteConstraint::new(
                        BodyPartHandle(body1.0, 0),
                        BodyPartHandle(body2.0, 0),
                        param.anchor1,
                        param.anchor2,
                    );
            
                    let joint_handle = state.world.joint_constraints.insert(revolute_constraint);
                    state.ecs.unset::<InitRevoluteJoint>(entity).unwrap();
                    state.ecs.set(entity, RevoluteJoint(joint_handle)).unwrap();
                }
            }            
        }
        Ok(())
    }

    fn refresh_body_sprites(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Body, Sprite);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let body_component: Body = state.ecs.get(entity).unwrap();
            let body = state.world.bodies.rigid_body(body_component.0).unwrap();
            let orientation = body.position().rotation.angle();
            let loc = body.position().translation.vector;

            let sprite: &mut Sprite = state.ecs.borrow_mut(entity).unwrap();                        
            sprite.location = [loc.x, loc.y].into();
            sprite.orientation = orientation;
        };

        Ok(())
    }

    fn refresh_joint_sprites(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(RevoluteJoint, Sprite);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            let joint_component: RevoluteJoint = state.ecs.get(entity).unwrap();
            let physical_component: &mut Sprite = state.ecs.borrow_mut(entity).unwrap();
            let (anchor1, anchor2) = state.world.joint_constraints.get(joint_component.0).unwrap().anchors();
            let body1 = state.world.bodies.rigid_body(anchor1.0).unwrap();
            let body2 = state.world.bodies.rigid_body(anchor2.0).unwrap();
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

    fn refresh_overlapping(&mut self, state: &mut State) -> ggez::GameResult {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Sensor, Overlapping);
        state.ecs.collect_with(&filter, &mut ids);
        for &entity in ids.iter() {
            if state.ecs.has::<Overlapping>(entity).unwrap() {
                let collider_component: Sensor = state.ecs.get(entity).unwrap();
                let mut overlapping_component = Overlapping(vec![]);
    
                if let Some(overlapping_colliders) = state
                    .world
                    .geometrical_world
                    .colliders_in_proximity_of(&state.world.colliders, collider_component.0) {                    
                        for (_, overlapping_collider) in overlapping_colliders {
                            let overlapping_entity : recs::EntityId = *overlapping_collider
                                .user_data()
                                .unwrap()
                                .downcast_ref()
                                .unwrap();
                            overlapping_component.0.push(overlapping_entity);
                        }
                        overlapping_component.0.sort_by(|&a, &b| {
                            let da = self.distance_between(state, a, entity);
                            let db = self.distance_between(state, b, entity);
                            if da > db { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less }
                        });
                }
    
                state.ecs.set(entity, overlapping_component).unwrap();
            }
        };

        Ok(())
    }

    fn distance_between(&self, state: &State, e1: EntityId, e2: EntityId) -> f32 {
        let e1_body = state.world.bodies.rigid_body(state.ecs.borrow::<Body>(e1).unwrap().0).unwrap();
        let e2_body = state.world.bodies.rigid_body(state.ecs.borrow::<Body>(e2).unwrap().0).unwrap();

        let e1_loc = e1_body.position().translation.vector;
        let e2_loc = e2_body.position().translation.vector;

        let offset = e1_loc - e2_loc;

        offset.norm()
    }
}