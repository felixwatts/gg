use crate::err::GgResult;
use crate::system::system::System;
use ggez::GameResult;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use ggez::event::EventHandler;
use crate::engine::Engine;

pub struct LocalSetup{
    engine: Engine
}

impl LocalSetup{
    pub fn new(context: &mut ggez::Context) -> GgResult<LocalSetup>{
        let systems: Vec::<Box::<dyn System>> = vec![
            Box::new(crate::system::gorilla::GorillaSystem{is_local: true}),
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::render::RenderSystem::new(context)?),
        ];

        let engine = Engine::new(systems, context)?;
    
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