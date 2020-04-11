#[cfg(test)]
use crate::testing::assert_roughly_eq;
use crate::component::body::KEYFRAME_PERIOD;
use crate::component::body::radial::RadialBody;
use nalgebra::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct PlanarBody {
    pub(super) loc: Vector2<f32>,
    pub(super) vel: Vector2<f32>,
    pub(super) acc: Vector2<f32>,
    keyframe_countdown: f32
}

impl PlanarBody {

    pub fn new(loc: Vector2::<f32>, vel: Vector2::<f32>, acc: Vector2::<f32>) -> PlanarBody {
        PlanarBody{
            keyframe_countdown: 0.0,
            loc,
            vel,
            acc
        }
    }

    pub fn update(&mut self, duration: f32) {
        // apply average velocity in this window to location
        self.loc += (self.vel + (self.acc * duration / 2.0)) * duration;
        // apply acceleration over this window to velocity
        self.vel += self.acc * duration;

        self.keyframe_countdown -= duration
    }

    pub fn set_acc(&mut self, acc: Vector2::<f32>) {
        self.acc = acc;
        self.keyframe_countdown = 0.0;
    }

    pub fn to_radial(&self, origin: Vector2::<f32>) -> RadialBody {
        let radius = self.loc - origin;
        let loc = radius.x.atan2(radius.y);
        let tangent = Vector2::new(
            (loc + (std::f32::consts::PI * 0.5)).sin(),
            (loc + (std::f32::consts::PI * 0.5)).cos()
        );
        let vel = nalgebra::Matrix::dot(&self.vel, &tangent) / radius.norm();

        RadialBody::new(
            origin,
            radius.norm(),
            loc,
            vel,
            self.acc)
    }

    pub fn get_is_keyframe_and_reset(&mut self) -> bool {
        if self.keyframe_countdown <= 0.0 {
            self.keyframe_countdown = KEYFRAME_PERIOD;
            true
        } else {
            false
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
        keyframe_countdown: 0.0,
        loc: Vector2::new(loc_x, loc_y),
        vel: Vector2::new(vel_x, vel_y),
        acc: Vector2::new(a_x, a_y)
    };

    subject.update(t);

    assert_eq!(Vector2::<f32>::new(a_x, a_y), subject.acc);
    assert_eq!(exp_loc_x, subject.loc.x);
    assert_eq!(exp_loc_y, subject.loc.y);
    assert_eq!(exp_vel_x, subject.vel.x);
    assert_eq!(exp_vel_y, subject.vel.y);
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
        keyframe_countdown: 0.0,
        loc: Vector2::new(loc_x, loc_y),
        vel: Vector2::new(vel_x, vel_y),
        acc: Vector2::new(a_x, a_y)
    };

    let actual = subject.to_radial(Vector2::new(o_x, o_y));

    assert_eq!(Vector2::new(o_x, o_y), actual.origin);
    assert_eq!(Vector2::<f32>::new(a_x, a_y), actual.acc);
    assert_eq!(exp_radius, actual.radius);
    assert_roughly_eq("loc", exp_loc, actual.loc);
    assert_roughly_eq("vel", exp_vel, actual.vel);
}

#[test]
fn test_keyframe() {
    expect_keyframe(0.0, 0.0, true);
    expect_keyframe(1.0, 0.0, false);
    expect_keyframe(0.0, 1.0, true);
    expect_keyframe(1.0, 1.0, true);

    let radial = RadialBody::new(
        Vector2::zeros(),
        0.0,
        0.0,
        0.0,
        Vector2::zeros());

    let mut subject = radial.to_planar();

    assert_eq!(0.0, subject.keyframe_countdown);
    assert_eq!(true, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
    assert_eq!(KEYFRAME_PERIOD, subject.keyframe_countdown);
}

#[cfg(test)]
fn expect_keyframe(keyframe_countdown: f32, step_size: f32, expect: bool) {
    let mut subject = PlanarBody{
        keyframe_countdown,
        loc: Vector2::<f32>::zeros(),
        vel: Vector2::<f32>::zeros(),
        acc: Vector2::<f32>::zeros()
    };

    subject.update(step_size);

    assert_eq!(expect, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
}