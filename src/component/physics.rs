use nphysics2d::object::DefaultColliderHandle;
use nphysics2d::object::ColliderDesc;
use nphysics2d::object::RigidBodyDesc;
use nalgebra::{Point2, Vector2};
use nphysics2d::object::DefaultBodyHandle;
use recs::EntityId;
use nphysics2d::joint::DefaultJointConstraintHandle;

pub struct InitBody(pub RigidBodyDesc::<f32>);

#[derive(Clone)]
pub struct Physical {
    pub location: Vector2<f32>,
    pub orientation: f32
}

#[derive(Clone)]
pub struct Body (pub DefaultBodyHandle);

#[derive(Clone)]
pub struct InitRevoluteJoint {
    pub end1: EntityId,
    pub end2: EntityId,
    pub anchor1: Point2<f32>,
    pub anchor2: Point2<f32>
}

pub struct RevoluteJoint(pub DefaultJointConstraintHandle);

pub struct InitCollider(pub ColliderDesc::<f32>);

#[derive(Clone)]
pub struct Collider(pub DefaultColliderHandle);

pub struct Overlapping(pub Vec::<recs::EntityId>);