extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
// extern crate ggez;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use na::Vector2;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, BodyStatus, RigidBodyDesc, RigidBody, DefaultBodyHandle};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::force_generator::DefaultForceGeneratorSet;

struct Gorilla {
    body: DefaultBodyHandle
}

struct Hook {
    body: DefaultBodyHandle
}

impl Hook {
    fn new(x: f32, y: f32, bodies: &mut DefaultBodySet<f32>) -> Hook {
        let body = RigidBodyDesc::new()
            .translation(Vector2::new(x, y))
            .status(BodyStatus::Static)
            .build();
        Hook{
            body: bodies.insert(body)
        }
    }
}

struct MainState {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
    gorilla: Gorilla,
    hooks: [Hook; 5],
    pos_x: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {

        let mut bodies = DefaultBodySet::new();

        // add rigid body for gorilla
        let rigid_body_gorilla = RigidBodyDesc::new()
            .translation(Vector2::new(0.0, -10.0))
            .mass(500.0)
            .build();
        let gorilla = Gorilla{
            body: bodies.insert(rigid_body_gorilla)
        };

        let hooks = [ 
            Hook::new(-5.0, -5.0, &mut bodies), 
            Hook::new(-5.0, 5.0, &mut bodies), 
            Hook::new(5.0, -5.0, &mut bodies), 
            Hook::new(5.0, 5.0, &mut bodies), 
            Hook::new(0.0, 0.0, &mut bodies) 
        ];

        let s = MainState { 
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: bodies,
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
            gorilla: gorilla,
            hooks: hooks,
            pos_x: 0.0 
        };

        Ok(s)
    }

    fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        self.pos_x = self.pos_x % 800.0 + 1.0;
    }


}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for hook in self.hooks.iter() {

            // Get reference to the rigid body.
            let rigid_body = self.bodies.rigid_body(hook.body).expect("This rigid body does not exist.");
            // Get the position (containing both translation and rotation).
            let position = rigid_body.position();
            // Read the translation vector itself.
            let translation = position.translation.vector;

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(translation.x, translation.y),
                0.1,
                2.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
