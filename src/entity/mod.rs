use recs::EntityId;
use crate::component::Anchor;
use crate::component::Sprite;
use crate::component::PlanarLocVel;
use nalgebra::Vector2;
use ggez::GameResult;

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult<EntityId> {
    let root = ecs.create_entity();
    ecs.set(root, Anchor).unwrap();
    ecs.set(root, PlanarLocVel::new(loc)).unwrap();
    with_sprite(ecs, root, [1.0, 1.0, 1.0, 1.0], [0.1, 0.1].into())?;

    Ok(root)
}

pub fn with_sprite(ecs: &mut recs::Ecs, entity: recs::EntityId, color: [f32; 4], size: Vector2<f32>) -> GameResult {
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