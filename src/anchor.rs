use crate::state::PhysicalWorld;
use ggez::graphics::Color;
use ggez::nalgebra as na;
use na::Vector2;
use nphysics2d::object::{BodyStatus, RigidBodyDesc, DefaultBodyHandle, ColliderDesc, BodyPartHandle, DefaultColliderHandle};
use ncollide2d::shape::{ShapeHandle, Ball};

use crate::common::*;
use crate::state::State;
use crate::entity::Entity;

const RADIUS: f32 = 0.25;
const COLOR: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

pub struct Anchor {
    body: DefaultBodyHandle,
    collider: DefaultColliderHandle
}

impl Anchor {
    pub fn new(x: f32, y: f32, world: &mut PhysicalWorld) -> Anchor {
        let body = RigidBodyDesc::new()
            .translation(Vector2::new(x, y))
            .status(BodyStatus::Static)
            .build();
        let body_handle = world.bodies.insert(body);

        let shape = ShapeHandle::new(Ball::new(RADIUS));
        let collider = ColliderDesc::new(shape)
            .sensor(true)
            .build(BodyPartHandle(body_handle, 0));
        let collider_handle = world.colliders.insert(collider);
        
        Anchor{
            body: body_handle,
            collider: collider_handle
        }
    }
}

impl Entity for Anchor {
    fn update(&mut self, _input: &ControlState, world: &mut PhysicalWorld) -> ggez::GameResult<bool> {
        Ok(true)
    }

    fn body(&self) -> DefaultBodyHandle {
        self.body
    }

    fn radius(&self) -> f32 {
        RADIUS
    }

    fn color(&self) -> ggez::graphics::Color {
        COLOR
    }
}