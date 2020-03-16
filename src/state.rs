
use ggez::event::KeyMods;
use ggez::Context;
use ggez::event::KeyCode;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use na::Vector2;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::force_generator::DefaultForceGeneratorSet;

use crate::anchor::Anchor;
use crate::gorilla::Gorilla;
use crate::entity::Entity;

pub struct PhysicalWorld {
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>
}

impl PhysicalWorld {
    fn new() -> PhysicalWorld {
        PhysicalWorld { 
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new()
        }
    }

    fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );
    }
}

pub struct State {
    pub entities: Vec<Box<dyn Entity>>,
    sprite_batch: graphics::spritebatch::SpriteBatch,
    focal_entity: Option<usize>,
    physical_world: PhysicalWorld,
    input_state: crate::common::ControlState
}

impl State {
    pub fn new(gfx: graphics::Image) -> ggez::GameResult<State> {
        let sprite_batch = graphics::spritebatch::SpriteBatch::new(gfx);

        let mut state = State { 
            physical_world: PhysicalWorld::new(),
            entities: vec![],
            sprite_batch: sprite_batch,
            focal_entity: Some(0),
            input_state: crate::common::ControlState{ button_1_down: false, button_2_down: false }
        };

        let e0 = Box::new(Gorilla::new(&mut state.physical_world));
        let e1 = Box::new(Anchor::new(-5.0, -5.0,&mut state.physical_world));
        let e2 = Box::new(Anchor::new(-5.0, 5.0, &mut state.physical_world));
        let e3 = Box::new(Anchor::new(5.0, -5.0, &mut state.physical_world));
        let e4 = Box::new(Anchor::new(5.0, 5.0, &mut state.physical_world));
        let e5 = Box::new(Anchor::new(0.0, 0.0, &mut state.physical_world));

        state.entities.push(e0);
        state.entities.push(e1);
        state.entities.push(e2);
        state.entities.push(e3);
        state.entities.push(e4);
        state.entities.push(e5);

        Ok(state)
    }

    fn step(&mut self) {
        self.physical_world.step();
    }

    fn set_camera(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        let centre = match self.focal_entity {
            Some(id) => self.entities[id].location(&self.physical_world),
            None => [0.0, 0.0].into()
        };
        let x_min = centre.x - 10.0;
        let y_min = centre.y + 7.5;
        let screen_rect = graphics::Rect::new(
            x_min,
            y_min,
            20.0, 
            -15.0
        );
        graphics::set_screen_coordinates(
            context, 
            screen_rect
        )
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.step();
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, [0.0, 0.0, 0.0, 1.0].into());

        self.set_camera(context)?;

        self.sprite_batch.clear();

        let mut i = self.entities.len();
        while i > 0 {
            i = i - 1;
            let entity = &mut self.entities[i];
            let is_alive = entity.update(&self.input_state, &mut self.physical_world)?;
            if is_alive {
                if let Some(draw_param) = entity.draw_param(&self.physical_world) {
                    self.sprite_batch.add(draw_param);
                }
            } else {
                self.entities.remove(i);
            }
        }

        graphics::draw(context, &self.sprite_batch, graphics::DrawParam::default())?;

        graphics::present(context)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => {
                self.input_state.button_1_down = true
            },
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match keycode {
            KeyCode::Space => {
                self.input_state.button_1_down = false
            },
            _ => {}
        }
    }
}