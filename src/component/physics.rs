use nalgebra::Vector2;
use nphysics2d::object::DefaultBodyHandle;
use recs::EntityId;
use nphysics2d::joint::DefaultJointConstraintHandle;

pub struct InitBody {
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub radius: f32,
}

pub struct Physical {
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub radius: f32,
}

pub struct Body (pub DefaultBodyHandle);

pub struct InitRevoluteJoint {
    pub end1: EntityId,
    pub end2: EntityId,
    pub anchor1: Vector2<f32>,
    pub anchor2: Vector2<f32>
}

pub struct RevoluteJoint (DefaultJointConstraintHandle);