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

struct Rope {
    end_anchor: DefaultBodyHandle,
    end_gorilla: DefaultBodyHandle,
    joint_ends: DefaultJointConstraintHandle,
    joint_anchor: DefaultJointConstraintHandle,
    joint_gorilla: DefaultJointConstraintHandle
}

impl Rope {
    fn new(
        world: &mut PhysicalWorld,
        anchor: DefaultBodyHandle, 
        gorilla: DefaultBodyHandle) -> Rope {

        let loc_anchor = crate::common::body_to_location(anchor, world);
        let loc_gorilla = crate::common::body_to_location(gorilla, world);
        let separation = loc_anchor - loc_gorilla;

        // rope end at anchor
        let end_anchor_desc = RigidBodyDesc::new()
            .translation(loc_anchor)
            .build();
        let end_anchor_handle = world.bodies.insert(end_anchor_desc);

        // rope end at gorilla
        let end_gorilla_desc = RigidBodyDesc::new()
            .translation(loc_gorilla)
            .build();
        let end_gorilla_handle = world.bodies.insert(end_gorilla_desc);
 
        // rope ends cannot move relative to each other
        let body_gorilla = world.bodies.rigid_body(gorilla).expect("");
        let body_anchor = world.bodies.rigid_body(anchor).expect("");
        let constraint_ends = nphysics2d::joint::FixedConstraint::new(
            BodyPartHandle(anchor, 0),
            BodyPartHandle(gorilla, 0),                
            na::Point2::origin(),
            body_anchor.position().rotation,
            separation.into(),
            body_gorilla.position().rotation
        );
        let joint_ends_handle = world.joint_constraints.insert(constraint_ends);

        // rope end at gorilla can rotate but not move relative to gorilla
        let constraint_gorilla_rope_desc = RevoluteConstraint::new(
            BodyPartHandle(gorilla, 0),
            BodyPartHandle(end_gorilla_handle, 0),                
            na::Point2::origin(),
            na::Point2::origin()
        );
        let joint_gorilla_handle = world.joint_constraints.insert(constraint_gorilla_rope_desc);

        // rope end at anchor can rotate but not move relative to anchor
        let constraint_anchor_rope_desc = RevoluteConstraint::new(
            BodyPartHandle(anchor, 0),
            BodyPartHandle(end_anchor_handle, 0),                
            na::Point2::origin(),
            na::Point2::origin()
        );
        let joint_anchor_handle = world.joint_constraints.insert(constraint_anchor_rope_desc);
        
        Rope {
            end_anchor: end_anchor_handle,
            end_gorilla: end_gorilla_handle,
            joint_ends: joint_ends_handle,
            joint_anchor: joint_anchor_handle,
            joint_gorilla: joint_gorilla_handle
        }
    }

    fn kill(&self, world: &mut PhysicalWorld) {
        world.joint_constraints.remove(self.joint_anchor);
        world.joint_constraints.remove(self.joint_gorilla);
        world.joint_constraints.remove(self.joint_ends);
        world.bodies.remove(self.end_anchor);
        world.bodies.remove(self.end_gorilla);
    }
}

pub struct Gorilla {
    body: DefaultBodyHandle,
    collider: DefaultColliderHandle,
    anchor_sensor: DefaultColliderHandle,
    rope: Option<Rope>
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
        match self.rope {
            Some(_) => panic!(),
            None => {
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
                    self.rope = Some(Rope::new(world, anchor_body, self.body));
                }
            }
        }
    }

    fn remove_rope(&mut self, world: &mut PhysicalWorld) {
        match &self.rope {
            None => panic!(),
            Some(rope) => {
                rope.kill(world);
                self.rope = None
            }
        }
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