use crate::component::radial_body::RadialBody;
use nalgebra::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct PlanarBody {
    pub keyframe: bool,
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

    pub fn to_radial(&self, origin: Vector2::<f32>) -> RadialBody {
        let radius = self.loc - origin;
        let loc = radius.x.atan2(radius.y);
        let tangent = Vector2::new(
            (loc + (std::f32::consts::PI * 0.5)).sin(),
            (loc + (std::f32::consts::PI * 0.5)).cos()
        );
        let vel = nalgebra::Matrix::dot(&self.vel, &tangent) / radius.norm();
        RadialBody{
            keyframe: true,
            origin,
            radius: radius.norm(),
            loc,
            vel,
            accel: self.accel
        }
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
        keyframe: true,
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

#[cfg(test)]
fn assert_roughly_eq(name: &'static str, expected: f32, actual: f32) {
    assert!((expected - actual).abs() < 0.0001, "{}: {} != {}", name, expected, actual);
}

#[test]
fn test_to_radial() {
    // from.loc handled
    expect_to_radial(0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    expect_to_radial(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * 0.5, 0.0);
    expect_to_radial(0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * 1.0, 0.0);
    expect_to_radial(-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * -0.5, 0.0);

    // origin handled
    expect_to_radial(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0);
    expect_to_radial(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, std::f32::consts::PI * 0.5, 0.0);
    expect_to_radial(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, std::f32::consts::PI * 1.0, 0.0);
    expect_to_radial(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, std::f32::consts::PI * -0.5, 0.0);

    // from.vel handled
    expect_to_radial(0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0);
    expect_to_radial(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    expect_to_radial(0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0);

    expect_to_radial(1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * 0.5, 1.0);
    expect_to_radial(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * 0.5, 0.0);
    expect_to_radial(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, std::f32::consts::PI * 0.5, -1.0);

    // TODO two more quadrants

    // from.accel handled
    expect_to_radial(0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    expect_to_radial(0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    expect_to_radial(0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    expect_to_radial(0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
}

#[cfg(test)]
fn expect_to_radial(loc_x: f32, loc_y: f32, vel_x: f32, vel_y: f32, a_x: f32, a_y: f32, o_x: f32, o_y: f32, 
    exp_radius: f32, exp_loc: f32, exp_vel: f32) {
    let subject = PlanarBody{
        keyframe: true,
        loc: Vector2::new(loc_x, loc_y),
        vel: Vector2::new(vel_x, vel_y),
        accel: Vector2::new(a_x, a_y)
    };

    let actual = subject.to_radial(Vector2::new(o_x, o_y));

    assert_eq!(Vector2::new(o_x, o_y), actual.origin);
    assert_eq!(Vector2::<f32>::new(a_x, a_y), actual.accel);
    assert_eq!(exp_radius, actual.radius);
    assert_roughly_eq("loc", exp_loc, actual.loc);
    assert_roughly_eq("vel", exp_vel, actual.vel);
}