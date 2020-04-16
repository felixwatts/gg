use crate::colors::{RED, CYAN};
use crate::err::GgResult;
use crate::system::System;
use ggez::GameResult;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct LocalSetup{
    engine: Engine<ggez::Context>
}

impl LocalSetup{
    pub fn new(context: &mut ggez::Context) -> GgResult<LocalSetup>{

        let init_systems: Vec::<Box::<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::local_init::LocalInitSystem(vec![
                (RED, [-1.5, 5.0], KeyCode::LControl, KeyCode::LAlt),
                (CYAN, [1.5, 5.0], KeyCode::Left, KeyCode::Right)
            ])),
        ];

        let systems: Vec::<Box::<dyn System<ggez::Context>>> = vec![
            Box::new(crate::system::keyboard::KeyboardSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_latency_compensation_enabled: false}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];

        let engine = Engine::new(systems, Some(init_systems), context)?;
    
        Ok(LocalSetup{
            engine
        })
    }
}

impl EventHandler for LocalSetup {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.engine.update(context)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.engine.draw(context)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.engine.key_down_event(context, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, context: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.engine.key_up_event(context, keycode, keymod);
    }
}