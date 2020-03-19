use recs::{Ecs, EntityId};
use crate::component::render::Renderable;
use crate::component::render::Focus;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::Context;
use ggez::GameResult;
use ggez::graphics;

pub struct Render {
    sprite_batch: SpriteBatch,
}

impl<'a> Render {
    pub fn new(context: &mut Context) -> GameResult<Render> {
        let gfx = ggez::graphics::Image::new(context, "/1px.png")?;
        Ok(Render {
            sprite_batch: SpriteBatch::new(gfx)
        })
    }

    pub fn step(&mut self, ecs: &Ecs, context: &mut Context) -> ggez::GameResult {

        graphics::clear(context, [0.0, 0.0, 0.0, 1.0].into());

        self.set_focus(ecs, context)?;

        self.sprite_batch.clear();

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Renderable);
        ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let component = ecs.borrow::<Renderable>(id).unwrap();
            self.sprite_batch.add(component.0);
        };

        graphics::draw(context, &self.sprite_batch, graphics::DrawParam::default())?;

        graphics::present(context)?;

        Ok(())
    }

    fn set_focus(&mut self, ecs: &recs::Ecs, context: &mut Context) -> ggez::GameResult {
        let mut focus_entities = vec![];
        ecs.collect_with(&component_filter!(Focus), &mut focus_entities);
        if let Some(&focus_entity) = focus_entities.first() {
            if let Ok(physical_component) = ecs.borrow::<crate::component::physics::Physical>(focus_entity) {
                let x_min = physical_component.location.x - 10.0;
                let y_min = physical_component.location.y + 7.5;
                let screen_rect = graphics::Rect::new(
                    x_min,
                    y_min,
                    20.0, 
                    -15.0
                );
                return graphics::set_screen_coordinates(
                    context, 
                    screen_rect
                );
            }
        }

        Ok(())
    }
}