use crate::state::PhysicalWorld;
use nphysics2d::object::DefaultBodyHandle;
use ggez::graphics;
use ggez::nalgebra as na;
use na::Vector2;
use crate::common::*;

pub trait Entity {
    fn update(&mut self, input: &ControlState, world: &mut PhysicalWorld) -> ggez::GameResult<bool>;
    fn body(&self) -> DefaultBodyHandle;
    fn radius(&self) -> f32;
    fn color(&self) -> ggez::graphics::Color;

    fn location(&self, world: &PhysicalWorld) -> na::Vector2<f32> {
        body_to_location(self.body(), world)
    }

    fn orientation(&self, world: &PhysicalWorld) -> f32 {
        let rigid_body = world.bodies.rigid_body(self.body()).expect("body not found");
        let position = rigid_body.position();
        position.rotation.angle()
    }

    fn draw_param(&self, world: &PhysicalWorld) -> Option<graphics::DrawParam> {
        let radius = self.radius();
        let location = self.location(world) - na::Vector2::new(radius, radius);
        let orientation = self.orientation(world);
        let color = self.color();
        let scale = Vector2::new(radius * 2.0, radius * 2.0);
    
        let result = graphics::DrawParam::new()
            //.offset([0.5, 0.5]) apparently not needed, I'm not sure why
            .dest([location.x, location.y])
            .rotation(orientation)
            .scale(scale)
            .color(color);

        Some(result)
    }
}