use crate::component::sprite::Sprite;
use crate::system::System;
use crate::component::gorilla::{Gorilla, GorillaEvent};
use crate::err::GgResult;
use recs::Ecs;
use recs::EntityId;
use nalgebra::Vector2;

pub struct TagGameSystem{
    on_it_player: Option<EntityId>,
    victory_anchor: Option<EntityId>
}

fn get_new_players(state: &mut Ecs) -> Vec::<EntityId> {
    let mut players = vec![];
    state.collect_with(&component_filter!(Gorilla), &mut players);
    players
        .drain(..)
        .filter(|p| state
            .borrow::<Gorilla>(*p)
            .unwrap()
            .events
            .iter()
            .any(|e| match e { GorillaEvent::Enter() => true, _ => false }))
        .collect::<Vec::<_>>()
}

impl TagGameSystem{
    pub fn new() -> TagGameSystem{
        TagGameSystem{
            on_it_player: None,
            victory_anchor: None
        }
    }
    
    fn set_player_state(&mut self, player: EntityId, is_on_it: bool, state: &mut Ecs) -> GgResult {
        if is_on_it {
            self.on_it_player = Some(player);                              
            state.borrow_mut::<Sprite>(player).unwrap().src_loc = Vector2::new(0.0, 0.0);
        } else {
            if let Some(currently_on_it) = self.on_it_player {
                if currently_on_it == player {
                    self.on_it_player = None;
                }
            }

            state.borrow_mut::<Sprite>(player).unwrap().src_loc = Vector2::new(0.0, 16.0/32.0);
        }

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

            if players.is_empty() { 
                return Ok(())
            }

            // new players are not on it
            for &player in get_new_players(state).iter() {
                self.set_player_state(player, false, state)?;
            }

            // on-it player left game
            if let Some(on_it_player) = self.on_it_player {
                if !state.exists(on_it_player) {
                    self.on_it_player = None;
                }
            }

            // if there is no on-it player then make someone on-it
            if self.on_it_player.is_none() {
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
                        if let GorillaEvent::AttachToAnchor(anchor) = event {
                            if *anchor == victory_anchor {
                                self.set_player_state(on_it_player, false, state)?;
                                self.set_player_state(player, true, state)?;

                                break;
                            }
                        }
                    }
                }
            }

            Ok(())
        }
}