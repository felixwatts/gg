use crate::component::Sprite;
use nalgebra::Vector2;
use crate::err::GgResult;

pub fn with_sprite(ecs: &mut recs::Ecs, entity: recs::EntityId, color: [f32; 4], size: Vector2<f32>) -> GgResult {
    ecs.set(entity, 
        Sprite{
            color: color,
            location: [0.0, 0.0].into(),
            orientation: 0.0,
            size: size
        }
    ).unwrap();

    Ok(())
}