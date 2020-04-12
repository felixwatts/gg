use crate::component::sprite::Sprite;
use crate::system::system::System;
use crate::component::gorilla::{Gorilla, GorillaEvent};
use crate::err::GgResult;
use recs::Ecs;
use recs::EntityId;
use nalgebra::Vector2;

const SPRITE_SIZE_NORMAL: f32 = 0.3;
const SPRITE_SIZE_ON_IT: f32 = 0.6;

pub struct TagGameSystem{
    on_it_player: Option<EntityId>,
    victory_anchor: Option<EntityId>
}

impl TagGameSystem{
    pub fn new() -> TagGameSystem{
        TagGameSystem{
            on_it_player: None,
            victory_anchor: None
        }
    }
    
    fn set_player_state(&mut self, player: EntityId, is_on_it: bool, state: &mut Ecs) -> GgResult {
        match is_on_it {
            true => {
                self.on_it_player = Some(player);
                state.borrow_mut::<Sprite>(player).unwrap().size = Vector2::new(SPRITE_SIZE_ON_IT, SPRITE_SIZE_ON_IT);
            },
            false => {
                if let Some(currently_on_it) = self.on_it_player {
                    if currently_on_it == player {
                        self.on_it_player = None;
                    }
                }

                state.borrow_mut::<Sprite>(player).unwrap().size = Vector2::new(SPRITE_SIZE_NORMAL, SPRITE_SIZE_NORMAL);
            }
        };

        Ok(())
    }
}

impl<TContext> System<TContext> for TagGameSystem{
    fn update(
        &mut self, 
        state: &mut Ecs, 
        _: &TContext) -> GgResult {

            let mut players = vec![];
            state.collect_with(&component_filter!(Gorilla), &mut players);

            if players.len() == 0 { 
                return Ok(())
            }

            // on-it player left game
            if let Some(on_it_player) = self.on_it_player {
                if !state.exists(on_it_player) {
                    self.on_it_player = None;
                }
            }

            // if there is no on-it player then make someone on-it
            if let None = self.on_it_player {
                self.set_player_state(players[0], true, state)?
            }

            let on_it_player = self.on_it_player.unwrap();

            for event in state.borrow::<Gorilla>(on_it_player).unwrap().events.iter() {
                match event {
                    GorillaEvent::AttachToAnchor(anchor) => {
                        self.victory_anchor = Some(*anchor);
                    },
                    GorillaEvent::DetachFromAnchor() => {
                        self.victory_anchor = None
                    },
                    _ => {}
                }
            }

            // process victory
            if let Some(victory_anchor) = self.victory_anchor {
                for &player in players.iter() {
                    if player == on_it_player {
                        continue;
                    }

                    for event in state.borrow::<Gorilla>(player).unwrap().events.iter() {
                        match event {
                            GorillaEvent::AttachToAnchor(anchor) => {
                                if *anchor == victory_anchor {
                                    self.set_player_state(on_it_player, false, state)?;
                                    self.set_player_state(player, true, state)?;

                                    break;
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }

            Ok(())
        }
}