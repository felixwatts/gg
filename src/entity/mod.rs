use crate::component::InitSensor;
use crate::component::Owns;
use crate::component::Overlapping;
use crate::component::Sprite;
use crate::component::InitCollider;
use crate::component::InitBody;
use nalgebra::Vector2;
use ggez::GameResult;
use nphysics2d::object::RigidBodyDesc;
use nphysics2d::object::BodyStatus;
use nphysics2d::object::ColliderDesc;
use ncollide2d::shape::{Ball, ShapeHandle};

pub fn spawn_anchor(ecs: &mut recs::Ecs, loc: Vector2<f32>) -> GameResult {
    let root = ecs.create_entity();
    
    with_body(ecs, root, loc, BodyStatus::Static)?;
    with_sensor(ecs, root, 0.05)?;
    with_collider(ecs, root, 0.05)?;
    with_sprite(ecs, root, [1.0, 1.0, 1.0, 1.0], [0.1, 0.1].into())?;

    Ok(())
}

pub fn with_body(ecs: &mut recs::Ecs, entity: recs::EntityId, loc: Vector2<f32>, status: BodyStatus) -> GameResult {
    ecs.set(entity, InitBody(RigidBodyDesc::new()
        .translation(loc)
        .mass(1.0)
        .angular_inertia(1.0)
        .status(status))).unwrap();

        Ok(())
}

pub fn with_sensor(ecs: &mut recs::Ecs, entity: recs::EntityId, radius: f32) -> GameResult {
    let circle = ShapeHandle::new(Ball::new(radius));
    let desc = ColliderDesc::<f32>::new(circle).sensor(true);
    ecs.set(entity, InitSensor(desc)).unwrap();

    Ok(())
}

pub fn with_collider(ecs: &mut recs::Ecs, entity: recs::EntityId, radius: f32) -> GameResult {
    let circle = ShapeHandle::new(Ball::new(radius));
    let desc = ColliderDesc::<f32>::new(circle).density(1.0);
    ecs.set(entity, InitCollider(desc)).unwrap();

    Ok(())
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

pub fn with_overlapping(ecs: &mut recs::Ecs, entity: recs::EntityId) -> GameResult {
    ecs.set(entity, Overlapping(vec![])).unwrap();

    Ok(())
}

pub fn with_owns(ecs: &mut recs::Ecs, entity: recs::EntityId) -> GameResult {
    ecs.set(entity, Owns(vec![])).unwrap();

    Ok(())
}