use nphysics2d::force_generator::DefaultForceGeneratorHandle;
use nalgebra::Vector2;
use nphysics2d::object::DefaultColliderHandle;
use nphysics2d::object::ColliderDesc;
use nphysics2d::joint::DefaultJointConstraintHandle;
use nalgebra::Point2;
use recs::EntityId;
use nphysics2d::object::DefaultBodyHandle;
use nphysics2d::object::RigidBodyDesc;

pub struct Dead;

#[derive(Clone)]
pub struct Owns(pub Vec::<recs::EntityId>);

pub struct InitBody(pub RigidBodyDesc::<f32>);

#[derive(Clone)]
pub struct Body (pub DefaultBodyHandle);

#[derive(Clone)]
pub struct InitRevoluteJoint {
    pub end1: EntityId,
    pub end2: EntityId,
    pub anchor1: Point2<f32>,
    pub anchor2: Point2<f32>
}

#[derive(Clone)]
pub struct RevoluteJoint(pub DefaultJointConstraintHandle);

pub struct InitCollider(pub ColliderDesc::<f32>);

pub struct InitSensor(pub ColliderDesc::<f32>);

#[derive(Clone)]
pub struct Collider(pub DefaultColliderHandle);

#[derive(Clone)]
pub struct Sensor(pub DefaultColliderHandle);

pub struct Overlapping(pub Vec::<recs::EntityId>);

pub struct InitForce{
    pub entity: EntityId,
    pub force: Vector2::<f32>
}

pub struct Force(pub DefaultForceGeneratorHandle);

pub struct Sprite{
    pub color: [f32; 4],
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>
}

pub struct Focus;

#[derive(Clone)]
pub struct Gorilla {
    pub rope: Option<EntityId>,
    pub force: Option<EntityId>
}