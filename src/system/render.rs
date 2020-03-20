use crate::component::physics::Physical;
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

    pub fn step(&mut self, ecs: &mut Ecs, context: &mut Context) -> ggez::GameResult {
        self.copy_physical_to_drawparam(ecs)?;
        self.set_focus(ecs, context)?;
        graphics::clear(context, [0.0, 0.0, 0.0, 1.0].into());
        self.draw_sprites(ecs, context)?;
        graphics::present(context)?;

        Ok(())
    }

    fn draw_sprites(&mut self, ecs: &Ecs, context: &mut Context) -> GameResult {
        self.sprite_batch.clear();

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Renderable);
        ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let component = ecs.borrow::<Renderable>(id).unwrap();
            self.sprite_batch.add(component.0);
        };

        graphics::draw(context, &self.sprite_batch, graphics::DrawParam::default())?;

        Ok(())
    }

    fn copy_physical_to_drawparam(&mut self, ecs: &mut recs::Ecs) -> GameResult {
        let mut physical_renderable_entities = vec![];
        ecs.collect_with(&component_filter!(Physical, Renderable), &mut physical_renderable_entities);
        for &entity in physical_renderable_entities.iter() {
            let physical : Physical = ecs.get(entity).unwrap(); // TODO handle not found
            let renderable : &mut Renderable = ecs.borrow_mut(entity).unwrap(); // TODO handle not found

            renderable.0.rotation = physical.orientation;
            renderable.0.dest.x = physical.location.x;
            renderable.0.dest.y = physical.location.y;
            renderable.0.scale.x = physical.size.x;
            renderable.0.scale.y = physical.size.y;
        };

        Ok(())
    }

    fn set_focus(&mut self, ecs: &recs::Ecs, context: &mut Context) -> ggez::GameResult {
        let mut focus_entities = vec![];
        ecs.collect_with(&component_filter!(Focus), &mut focus_entities);
        if let Some(&focus_entity) = focus_entities.first() {
            if let Ok(physical_component) = ecs.borrow::<crate::component::physics::Physical>(focus_entity) {
                let x_min = physical_component.location.x - 30.0;
                let y_min = physical_component.location.y + 22.5;
                let screen_rect = graphics::Rect::new(
                    x_min,
                    y_min,
                    60.0, 
                    -45.0
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