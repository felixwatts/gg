use crate::state::PhysicalWorld;
use ggez::graphics::Color;
use ggez::nalgebra as na;
use na::Vector2;
use nphysics2d::object::{RigidBodyDesc, DefaultBodyHandle, DefaultColliderHandle, ColliderDesc, BodyPartHandle};
use nphysics2d::joint::{RevoluteConstraint};
use nphysics2d::joint::DefaultJointConstraintHandle;
use ncollide2d::shape::{ShapeHandle, Ball};
use crate::entity::Entity;
use crate::common::ControlState;

const RADIUS: f32 = 0.75;
const COLOR: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };

pub struct Gorilla {
    body: DefaultBodyHandle,
    collider: DefaultColliderHandle,
    anchor_sensor: DefaultColliderHandle,
    rope: Option<DefaultJointConstraintHandle>
}

impl Gorilla {
    pub fn new(world: &mut PhysicalWorld) -> Gorilla {
        let body = RigidBodyDesc::new()
            .translation(Vector2::new(0.0, 10.0))
            .mass(500.0)
            .build();
        let body_handle = world.bodies.insert(body);

        let shape = ShapeHandle::new(Ball::new(RADIUS));
        let collider = ColliderDesc::new(shape)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0));
        let collider_handle = world.colliders.insert(collider);

        let anchor_sensor_shape = ShapeHandle::new(Ball::new(crate::common::MAX_ROPE_LENGTH));
        let anchor_sensor_collidor = ColliderDesc::new(anchor_sensor_shape)
            .sensor(true)
            .build(BodyPartHandle(body_handle, 0));
        let anchor_sensor_collidor = world.colliders.insert(anchor_sensor_collidor);

        Gorilla{
            body: body_handle,
            collider: collider_handle,
            anchor_sensor: anchor_sensor_collidor,
            rope: None
        }
    }

    fn try_add_rope(&mut self, world: &mut PhysicalWorld) {

        assert_eq!(self.rope, None);

        let closest_anchor = world
            .geometrical_world
            .colliders_in_proximity_of(&world.colliders, self.anchor_sensor)
            .expect("")
            .min_by(|a, b| {
                let distance_to_a = (crate::common::body_to_location(a.1.body(), world) - self.location(world)).norm();
                let distance_to_b = (crate::common::body_to_location(b.1.body(), world) - self.location(world)).norm();
                if distance_to_a > distance_to_b { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less } 
            });

        if let Some((_, anchor_collider)) = closest_anchor {
            let anchor_body = anchor_collider.body();

            let constraint = RevoluteConstraint::new(
                BodyPartHandle(self.body, 0),
                BodyPartHandle(anchor_body, 0),
                [0.0, 0.0].into(),
                [0.0, 0.0].into()
            );

            self.rope = Some(world.joint_constraints.insert(constraint));
        }
    }

    fn remove_rope(&mut self, world: &mut PhysicalWorld) {
        assert_ne!(self.rope, None);
        let handle = self.rope.expect("rope not found");
        world.joint_constraints.remove(handle);
        self.rope = None;
    }
}

impl Entity for Gorilla {
    fn update(&mut self, input: &ControlState, world: &mut PhysicalWorld) -> ggez::GameResult<bool> {
        match self.rope {
            Some(_) => {
                if !input.button_1_down {
                    self.remove_rope(world);
                }
            },
            None => {
                if input.button_1_down {
                    self.try_add_rope(world);
                }
            }
        }

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