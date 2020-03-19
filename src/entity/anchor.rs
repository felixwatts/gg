use nalgebra::Vector2;
use ggez::GameResult;
use nphysics2d::object::BodyStatus;
use crate::entity::*;

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult {
    let root = ecs.create_entity();
    
    with_body(ecs, root, loc, BodyStatus::Static)?;
    with_physical(ecs, root)?;
    with_sensor(ecs, root, 0.1)?;
    with_sprite(ecs, root, 0.1, [1.0, 1.0, 1.0, 1.0])?;

    Ok(())
}