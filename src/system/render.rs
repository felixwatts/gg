use crate::state::State;
use crate::component::Focus;
use crate::component::Sprite;
use ggez::graphics::DrawParam;
use crate::system::system::System;
use recs::{Ecs, EntityId};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::Context;
use crate::err::GgResult;
use ggez::graphics;

pub struct RenderSystem {
    sprite_batch: SpriteBatch,
}

impl System for RenderSystem {
    fn draw(&mut self, state: &State, context: &mut Context) -> GgResult {
        self.set_focus(state, context)?;
        graphics::clear(context, [0.0, 0.0, 0.0, 1.0].into());
        self.draw_sprites(state, context)?;
        graphics::present(context)?;

        Ok(())
    }
}

fn entity_to_draw_param(entity: EntityId, ecs: &Ecs) -> DrawParam {
    let sprite : &Sprite = ecs.borrow(entity).unwrap();
    DrawParam::new()
        .offset([0.5, 0.5])
        .color(sprite.color.into())
        .scale(sprite.size)
        .rotation(sprite.orientation)
        .dest([sprite.location.x, sprite.location.y])
}

impl RenderSystem {
    pub fn new(context: &mut Context) -> GgResult<RenderSystem> {
        let gfx = ggez::graphics::Image::new(context, "/1px.png")?;
        Ok(RenderSystem {
            sprite_batch: SpriteBatch::new(gfx)
        })
    }

    fn draw_sprites(&mut self, state: &State, context: &mut Context) -> GgResult {
        self.sprite_batch.clear();

        let mut sprite_entities = vec![];
        state.ecs.collect_with(&component_filter!(Sprite), &mut sprite_entities);

        let draw_params = sprite_entities.iter().map(|&entity| entity_to_draw_param(entity, &state.ecs));
        for draw_param in draw_params {
            self.sprite_batch.add(draw_param);
        }

        graphics::draw(context, &self.sprite_batch, graphics::DrawParam::default())?;

        Ok(())
    }

    fn set_focus(&mut self, state: &State, context: &mut Context) -> GgResult {
        let mut focus_entities = vec![];
        state.ecs.collect_with(&component_filter!(Focus), &mut focus_entities);
        if let Some(&focus_entity) = focus_entities.first() {
            if let Ok(sprite) = state.ecs.borrow::<Sprite>(focus_entity) {
                let x_min = sprite.location.x - 6.0;
                let y_min = sprite.location.y + 4.5;
                let screen_rect = graphics::Rect::new(
                    x_min,
                    y_min,
                    12.0, 
                    -9.0
                );
                graphics::set_screen_coordinates(
                    context, 
                    screen_rect
                )?;
            }
        }

        Ok(())
    }
}