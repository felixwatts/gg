use crate::state::PhysicalWorld;
use nphysics2d::object::DefaultBodyHandle;
use ggez::nalgebra as na;

pub struct ControlState {
    pub button_1_down: bool,
    pub button_2_down: bool
}

pub const MAX_ROPE_LENGTH: f32 = 5.0;

pub fn body_to_location(body: DefaultBodyHandle, world: &PhysicalWorld) -> na::Vector2<f32> {
    let rigid_body = world.bodies.rigid_body(body).expect("body not found");
    let position = rigid_body.position();
    let translation = position.translation.vector;

    [translation.x,  translation.y].into()
}