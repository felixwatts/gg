use recs::Ecs;
use nalgebra::Vector2;
use recs::EntityId;

pub struct Dead;

#[derive(Clone)]
pub struct Owns(pub Vec::<EntityId>);

#[derive(Clone)]
pub struct RadialLocVel {
    pub origin: EntityId,
    pub radius: f32,
    pub loc: f32,
    pub vel: f32
}

impl RadialLocVel {
    pub fn to_planar(&self, ecs: &Ecs) -> PlanarLocVel {
        let origin_loc_vel : PlanarLocVel = ecs.get(self.origin).unwrap();

        let dx = self.radius * self.loc.sin();
        let dy = self.radius * self.loc.cos();

        let radius = nalgebra::Vector2::new(dx, dy);

        let loc_planar = origin_loc_vel.loc + radius;

        let tangent = nalgebra::Vector2::new(dy, dx);
        let vel_planar = tangent * self.vel;

        PlanarLocVel {
            loc: loc_planar,
            vel: vel_planar
        }
    }
}

#[derive(Clone)]
pub struct PlanarLocVel {
    pub loc: Vector2<f32>,
    pub vel: Vector2<f32>
}

impl PlanarLocVel {
    pub fn new(loc: Vector2::<f32>) -> PlanarLocVel {
        PlanarLocVel {
            loc: loc,
            vel: Vector2::zeros()
        }
    }

    pub fn to_radial(&self, ecs: &Ecs, origin: EntityId) -> RadialLocVel {
        let origin_loc_vel : PlanarLocVel = ecs.get(origin).unwrap();

        let radius = self.loc - origin_loc_vel.loc;
        let loc_radial = (radius.x / radius.y).atanh();

        let tangent = nalgebra::Vector2::new(radius.y, radius.x);
        let vel_planar = self.vel.dot(&tangent) / tangent.norm();

        RadialLocVel {
            origin: origin,
            radius: radius.norm(),
            loc: loc_radial,
            vel: vel_planar
        }
    }

    pub fn distance_to(&self, other: PlanarLocVel) -> f32 {
        (self.loc - other.loc).norm()
    }
}

#[derive(Clone)]
pub struct Gravity(pub f32);

pub struct Sprite{
    pub color: [f32; 4],
    pub location: Vector2<f32>,
    pub orientation: f32,
    pub size: Vector2<f32>
}

pub struct Focus;

pub struct Gorilla;

pub struct Anchor;