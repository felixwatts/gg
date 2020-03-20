use crate::component::gorilla::Gorilla;
use crate::component::render::Focus;
use nalgebra::Vector2;
use ggez::GameResult;
use nphysics2d::object::BodyStatus;
use crate::entity::*;

pub fn spawn_gorilla(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult {
    let root = ecs.create_entity();

    with_body(ecs, root, loc, BodyStatus::Dynamic)?;
    with_physical(ecs, root)?;
    with_sensor(ecs, root, 5.0)?;
    with_overlapping(ecs, root)?;
    with_sprite(ecs, root, 1.0, [1.0, 0.0, 0.0, 1.0])?;
    ecs.set(root, Focus).unwrap();
    ecs.set(root, Owns(vec![])).unwrap();
    ecs.set(root, Gorilla{rope: None}).unwrap();

    Ok(())
}