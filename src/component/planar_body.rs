use nalgebra::Vector2;

#[derive(Clone)]
pub struct PlanarBody {
    pub loc: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub accel: Vector2<f32>
}

impl PlanarBody {
    pub fn update(&mut self, duration: f32) {
        // apply average velocity in this window to location
        self.loc += (self.vel + (self.accel * duration / 2.0)) * duration;
        // apply acceleration over this window to velocity
        self.vel += self.accel * duration;
    }

    // pub fn to_radial(&self, ecs: &Ecs, origin: EntityId) -> RadialLocVel {
    //     let origin_loc_vel : PlanarBody = ecs.get(origin).unwrap();

    //     let radius = self.loc - origin_loc_vel.loc;
    //     let loc_radial = (radius.x / radius.y).atanh();

    //     let tangent = nalgebra::Vector2::new(radius.y, radius.x);
    //     let vel_planar = self.vel.dot(&tangent) / tangent.norm();

    //     RadialLocVel {
    //         origin: origin,
    //         radius: radius.norm(),
    //         loc: loc_radial,
    //         vel: vel_planar
    //     }
    // }

    pub fn distance_to(&self, other: PlanarBody) -> f32 {
        (self.loc - other.loc).norm()
    }
}

#[test]
fn test_update() {
    // if no vel or accel then no change
    expect_update(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    // if vel and no accel then linear change in loc and no change in vel
    expect_update(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    expect_update(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 0.5);
    expect_update(0.0, 0.0, -1.0, -1.0, 0.0, 0.0, -1.0, -1.0, -1.0, -1.0, 1.0);

    // accel leads to change in vel and loc
    expect_update(0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0);
    
    // negative accel lead to negative vel and loc
    expect_update(0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, 1.0);

    // accel is added to velocity
    expect_update(0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.5, 1.5, 2.0, 2.0, 1.0);

    // accel and vel applied linearly with time
    expect_update(0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.125, 0.125, 0.5, 0.5, 0.5);
}

#[cfg(test)]
fn expect_update(loc_x: f32, loc_y: f32, vel_x: f32, vel_y: f32, a_x: f32, a_y: f32, 
    exp_loc_x: f32, exp_loc_y: f32, exp_vel_x: f32, exp_vel_y: f32, t: f32) {
    let mut subject = PlanarBody{
        loc: Vector2::new(loc_x, loc_y),
        vel: Vector2::new(vel_x, vel_y),
        accel: Vector2::new(a_x, a_y)
    };

    subject.update(t);

    assert_eq!(Vector2::<f32>::new(a_x, a_y), subject.accel);
    assert_eq!(exp_loc_x, subject.loc.x);
    assert_eq!(exp_loc_y, subject.loc.y);
    assert_eq!(exp_vel_x, subject.vel.x);
    assert_eq!(exp_vel_y, subject.vel.y);
}