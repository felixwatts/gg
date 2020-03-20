pub mod anchor;
pub mod gorilla;

use crate::component::lifecycle::Owns;
use crate::component::physics::Overlapping;
use crate::component::physics::InitCollider;
use crate::component::render::Renderable;
use nalgebra::Vector2;
use ggez::GameResult;
use crate::component::physics::InitBody;
use nphysics2d::object::RigidBodyDesc;
use nphysics2d::object::BodyStatus;
use nphysics2d::object::ColliderDesc;
use ncollide2d::shape::{Ball, ShapeHandle};

fn with_body(ecs: &mut recs::Ecs, entity: recs::EntityId, loc: Vector2<f32>, status: BodyStatus) -> GameResult {
    ecs.set(entity, InitBody(RigidBodyDesc::new()
        .translation(loc)
        .mass(1.0)
        .status(status))).unwrap();

        Ok(())
}

fn with_physical(ecs: &mut recs::Ecs, entity: recs::EntityId) -> GameResult {
    ecs.set(entity, crate::component::physics::Physical{
        location: nalgebra::Vector2::<f32>::new(0.0, 0.0),
        orientation: 0.0
    }).unwrap();

        Ok(())
}

fn with_sensor(ecs: &mut recs::Ecs, entity: recs::EntityId, radius: f32) -> GameResult {
    let circle = ShapeHandle::new(Ball::new(radius));
    let desc = ColliderDesc::<f32>::new(circle).sensor(true);
    ecs.set(entity, InitCollider(desc)).unwrap();

    Ok(())
}

fn with_sprite(ecs: &mut recs::Ecs, entity: recs::EntityId, radius: f32, color: [f32; 4]) -> GameResult {
    ecs.set(entity, Renderable(ggez::graphics::DrawParam::new()
        .offset([0.5, 0.5])
        .color(color.into())
        .scale([radius*2.0, radius*2.0]))).unwrap();

    Ok(())
}

fn with_overlapping(ecs: &mut recs::Ecs, entity: recs::EntityId) -> GameResult {
    ecs.set(entity, Overlapping(vec![])).unwrap();

    Ok(())
}

fn with_owns(ecs: &mut recs::Ecs, entity: recs::EntityId) -> GameResult {
    ecs.set(entity, Owns(vec![])).unwrap();

    Ok(())
}