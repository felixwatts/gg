use ggez::event::EventHandler;
use ggez::GameResult;
use ggez::Context;
use crate::entity::anchor::*;
use crate::entity::gorilla::*;

pub struct Engine {
    ecs: recs::Ecs,
    render_system: crate::system::render::Render,
    physics_system: crate::system::physics::Physics,
    teardown_system: crate::system::teardown::Teardown,
    gorilla_system: crate::system::gorilla::GorillaSystem
}

impl EventHandler for Engine {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {

        self.physics_system.step(&mut self.ecs)?;
        self.gorilla_system.step(&mut self.ecs, context)?;
        self.teardown_system.step(&mut self.ecs, &mut self.physics_system, &mut self.gorilla_system)?;

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        self.render_system.step(&mut self.ecs, context)?;

        Ok(())
    }
}

impl Engine {
    pub fn new(context: &mut ggez::Context) -> GameResult<Engine> {
        let mut engine = Engine{
            ecs: recs::Ecs::new(),
            render_system: crate::system::render::Render::new(context)?,
            physics_system: crate::system::physics::Physics::new(),
            teardown_system: crate::system::teardown::Teardown::new(),
            gorilla_system: crate::system::gorilla::GorillaSystem::new()
        };

        spawn_anchor(&mut engine.ecs, [-5.0, -5.0].into())?;
        spawn_anchor(&mut engine.ecs, [-5.0, 5.0].into())?;
        spawn_anchor(&mut engine.ecs, [0.0, 0.0].into())?;
        spawn_anchor(&mut engine.ecs, [5.0, -5.0].into())?;
        spawn_anchor(&mut engine.ecs, [5.0, 5.0].into())?;

        spawn_gorilla(&mut engine.ecs, [-2.5, 10.0].into())?;

        Ok(engine)
    }
}