use recs::Ecs;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::force_generator::DefaultForceGeneratorSet;

pub struct PhysicalWorld {
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>
}

impl PhysicalWorld{
    pub fn new() -> PhysicalWorld {
        PhysicalWorld{
            mechanical_world: DefaultMechanicalWorld::<f32>::new([0.0, -9.81].into()),
            geometrical_world: DefaultGeometricalWorld::<f32>::new(),
            bodies: DefaultBodySet::<f32>::new(),
            colliders: DefaultColliderSet::<f32>::new(),
            joint_constraints: DefaultJointConstraintSet::<f32>::new(),
            force_generators: DefaultForceGeneratorSet::<f32>::new()
        }
    }
}

pub struct State {
    pub world: PhysicalWorld,
    pub ecs: Ecs
}