use crate::err::GgResult;
use crate::input::InputEvent;
use crate::component::Keyboard;
use crate::component::gorilla::Gorilla;
use ggez::event::KeyMods;
use ggez::event::KeyCode;
use recs::Ecs;
use crate::system::system::System;

pub struct KeyboardSystem {
}

fn process_keyboard_event(state: &mut Ecs, keycode: &KeyCode, is_down: bool) -> GgResult{
    let mut gorilla_entities = vec![];
    state.collect_with(&component_filter!(Gorilla, Keyboard), &mut gorilla_entities);

    for gorilla_entity in gorilla_entities{
        let keyboard_component: &Keyboard = state.borrow(gorilla_entity).unwrap();
        match keyboard_component.0.get(keycode) {
            Some(&button) => {
                let gorilla_component = state.borrow_mut::<Gorilla>(gorilla_entity).unwrap();
                gorilla_component.input_events.push(InputEvent{button, is_down});
                break;
            },
            None => {}
        }
    }

    Ok(())
}

impl<TContext> System<TContext> for KeyboardSystem{
    fn key_down(&mut self,
        state: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool) {
            if repeat { return }
            process_keyboard_event(state, &keycode, true).unwrap();
        }

    fn key_up(&mut self,
        state: &mut Ecs,
        _: &mut TContext,
        keycode: KeyCode,
        _: KeyMods) {
            process_keyboard_event(state, &keycode, false).unwrap();
        }
}