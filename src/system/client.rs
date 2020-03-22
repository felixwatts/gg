use crate::network::ButtonState;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use ggez::Context;
use crate::state::State;
use crate::system::system::System;
use crate::network::ClientMsg;
use crate::network::Button;

pub struct ClientSystem {}

impl System for ClientSystem {
    fn key_down(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool) {
        match keycode {
            KeyCode::Space => {
                let msg = ClientMsg::ButtonStateChange(ButtonState{button: Button::One, is_down: true});
                crate::network::client_tx(&mut state.ecs, msg);
            },
            KeyCode::Return => {
                let msg = ClientMsg::ButtonStateChange(ButtonState{button: Button::Two, is_down: true});
                crate::network::client_tx(&mut state.ecs, msg);
            },
            _ => {}
        }
    }

    fn key_up(&mut self,
        state: &mut State,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods) {
        match keycode {
            KeyCode::Space => {
                let msg = ClientMsg::ButtonStateChange(ButtonState{button: Button::One, is_down: false});
                crate::network::client_tx(&mut state.ecs, msg);
            },
            KeyCode::Return => {
                let msg = ClientMsg::ButtonStateChange(ButtonState{button: Button::Two, is_down: false});
                crate::network::client_tx(&mut state.ecs, msg);
            },
            _ => {}
        }
    }
}