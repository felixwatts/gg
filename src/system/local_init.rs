use crate::component::Focus;
use crate::system::gorilla::spawn_anchor;
use crate::input::new_key_mapping;
use crate::err::GgResult;
use recs::Ecs;
use crate::system::system::System;
use ggez::event::KeyCode;
use crate::colors::Color;

pub struct LocalInitSystem(pub Vec::<(Color, [f32;2], KeyCode, KeyCode)>);

impl<TContext> System<TContext> for LocalInitSystem {
    fn init(&mut self, state: &mut Ecs, _: &TContext) -> GgResult {

        for player in self.0.iter() {
            crate::system::gorilla::spawn_gorilla(
                state, 
                player.1.into(), 
                player.0, 
                Some(new_key_mapping(player.2, player.3)),
                false)?;
        }

        spawn_anchor(state, [-3.0, -3.0].into())?;
        spawn_anchor(state, [-3.0, 3.0].into())?;
        let centre = spawn_anchor(state, [0.0, 0.0].into())?;
        spawn_anchor(state, [3.0, -3.0].into())?;
        spawn_anchor(state, [3.0, 3.0].into())?;

        state.set(centre, Focus)?;

        Ok(())
    }
}