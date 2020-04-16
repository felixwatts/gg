#[cfg(test)]
use crate::testing::assert_roughly_eq;
use crate::component::body::KEYFRAME_PERIOD;
use crate::component::body::planar::PlanarBody;
use nalgebra::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct RadialBody {
    pub(super) origin: Vector2<f32>,
    pub(super) radius: f32,
    pub(super) loc: f32,
    pub(super) vel: f32,

    // acceleration is expressed in planar coordinates
    pub(super) acc: Vector2<f32>,

    keyframe_countdown: f32
}

impl RadialBody {

    pub fn new(origin: Vector2<f32>, radius: f32, loc: f32, vel: f32, acc: Vector2<f32>) -> RadialBody {
        RadialBody{
            origin,
            radius,
            loc,
            vel,
            acc,
            keyframe_countdown: 0.0
        }
    }

    pub fn set_acc(&mut self, acc: Vector2::<f32>) {
        self.acc = acc;
        self.keyframe_countdown = 0.0;
    }

    fn radius(&self) -> Vector2::<f32> {
        Vector2::new(
            self.loc.sin() * self.radius,
            self.loc.cos() * self.radius)
    }

    fn tangent(&self) -> Vector2::<f32> {
        Vector2::new(
            (self.loc + (std::f32::consts::PI * 0.5)).sin(),
            (self.loc + (std::f32::consts::PI * 0.5)).cos()
        )
    }

    fn acc_along_tangent(&self) -> f32 {
        nalgebra::Matrix::dot(&self.tangent(), &self.acc)
    }

    pub fn update(&mut self, duration: f32) {
        let accel = self.acc_along_tangent();
        self.loc += (self.vel + (accel * duration / 2.0)) * duration;
        self.vel += accel * duration;

        // for some reason a small amount of damping is neccessary here to
        // stop velocity increasing continuously while swinging
        self.vel *= 1.0 - (0.1 * duration);

        self.keyframe_countdown -= duration
    }

    pub fn to_planar(&self) -> PlanarBody {
        let loc = self.origin + self.radius();
        let vel = self.tangent() * self.vel * self.radius;

        PlanarBody::new(
            loc,
            vel,
            self.acc
        )
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
fn test_accel_along_tangent() {
    expect_accel_along_tangent(0.0, 1.0, 0.0, 0.0, 0.0);
    expect_accel_along_tangent(0.0, 1.0, 1.0, 0.0, 1.0);
    expect_accel_along_tangent(0.0, 1.0, 2.0, 0.0, 2.0);
    expect_accel_along_tangent(0.0, 1.0, -1.0, 0.0, -1.0);

    expect_accel_along_tangent(0.0, 2.0, 1.0, 0.0, 1.0);

    expect_accel_along_tangent(0.0, 1.0, 1.0, 1.0, 1.0);

    expect_accel_along_tangent(std::f32::consts::PI * 0.5, 1.0, 1.0, 0.0, 0.0);
    expect_accel_along_tangent(std::f32::consts::PI * 0.5, 1.0, 0.0, -1.0, 1.0);

    expect_accel_along_tangent(std::f32::consts::PI * 1.0, 1.0, 0.0, 1.0, 0.0);
    expect_accel_along_tangent(std::f32::consts::PI * 1.0, 1.0, -1.0, 0.0, 1.0);

    expect_accel_along_tangent(std::f32::consts::PI * 1.5, 1.0, 1.0, 0.0, 0.0);
    expect_accel_along_tangent(std::f32::consts::PI * 1.5, 1.0, 0.0, 1.0, 1.0);
}

#[cfg(test)]
fn expect_accel_along_tangent(loc: f32, radius: f32, ax: f32, ay: f32, expected: f32) {
    let subject = RadialBody {
        keyframe_countdown: 0.0,
        origin: Vector2::<f32>::zeros(),
        radius,
        loc,
        vel: 0.0,
        acc: Vector2::new(ax, ay),
    };

    let actual = subject.acc_along_tangent();

    assert_roughly_eq("accel_along_tangent", expected, actual);
}

#[test]
fn test_radius() {
    expect_radius(0.0, 1.0, 0.0, 1.0);
    expect_radius(0.0, 7.0, 0.0, 7.0);

    expect_radius(std::f32::consts::PI / 2.0, 1.0, 1.0, 0.0);
    expect_radius(std::f32::consts::PI / 2.0, 7.0, 7.0, 0.0);

    expect_radius(std::f32::consts::PI, 1.0, 0.0, -1.0);
    expect_radius(std::f32::consts::PI, 7.0, 0.0, -7.0);

    expect_radius(std::f32::consts::PI * 1.5, 1.0, -1.0, 0.0);
    expect_radius(std::f32::consts::PI * 1.5, 7.0, -7.0, 0.0);
}

#[cfg(test)]
fn expect_radius(loc: f32, radius: f32, x: f32, y: f32) {
    let subject = RadialBody {
        keyframe_countdown: 0.0,
        origin: Vector2::<f32>::zeros(),
        radius,
        loc,
        vel :0.0,
        acc: Vector2::<f32>::zeros(),
    };

    let actual = subject.radius();

    assert_roughly_eq("x", x, actual.x);
    assert_roughly_eq("y", y, actual.y);
}

#[test]
fn test_tangent() {
    expect_tangent(0.0, 1.0, 1.0, 0.0);
    expect_tangent(0.0, 7.0, 1.0, 0.0);

    expect_tangent(std::f32::consts::PI / 2.0, 1.0, 0.0, -1.0);
    expect_tangent(std::f32::consts::PI / 2.0, 7.0, 0.0, -1.0);

    expect_tangent(std::f32::consts::PI, 1.0, -1.0, 0.0);
    expect_tangent(std::f32::consts::PI, 7.0, -1.0, 0.0);

    expect_tangent(std::f32::consts::PI * 1.5, 1.0, 0.0, 1.0);
    expect_tangent(std::f32::consts::PI * 1.5, 7.0, 0.0, 1.0);
}

#[cfg(test)]
fn expect_tangent(loc: f32, radius: f32, x: f32, y: f32) {
    let subject = RadialBody {
        keyframe_countdown: 0.0,
        origin: Vector2::<f32>::zeros(),
        radius,
        loc,
        vel: 0.0,
        acc: Vector2::<f32>::zeros(),
    };

    let actual = subject.tangent();

    assert_roughly_eq("x", x, actual.x);
    assert_roughly_eq("y", y, actual.y);
}

#[test]
fn test_update() {
    // if no vel or accel then no change
    expect_update(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    // if vel and no accel then linear change in loc and no change in vel
    expect_update(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    expect_update(1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5, 1.0);
    expect_update(1.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, -1.0);

    // if accel then non-linear change in loc and linear change in vel, relative
    // to alignment of accel vector to tangent
    expect_update(1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0);
    // accel along tangent leads to change in vel and loc
    expect_update(1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.5, 1.0);
    // negative accel lead to negative vel and loc
    expect_update(1.0, 0.0, 0.0, -1.0, 0.0, 1.0, -0.5, -1.0);
    // accel is added to velocity
    expect_update(1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.5, 2.0);
    // accel and vel applied linearly with time
    expect_update(1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.125, 0.5);
}

#[cfg(test)]
// expected vel should be without damping, damping factor will be accounted for inside this fn
fn expect_update(radius: f32, loc: f32, vel: f32, ax: f32, ay: f32, t: f32, exp_loc: f32, exp_vel: f32) {
    let mut subject = RadialBody{
        keyframe_countdown: 0.0,
        origin: Vector2::<f32>::zeros(),
        radius,
        loc,
        vel,
        acc: Vector2::new(ax, ay)
    };

    subject.update(t);

    assert_eq!(Vector2::<f32>::zeros(), subject.origin);
    assert_eq!(Vector2::<f32>::new(ax, ay), subject.acc);
    assert_eq!(radius, subject.radius);
    assert_roughly_eq("loc", exp_loc, subject.loc);
    assert_roughly_eq("vel", exp_vel * (1.0 - (0.1 * t)), subject.vel);
}

#[test]
fn test_to_planar() {
    // simplest case
    expect_to_planar(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    // origin x affects loc of planar
    expect_to_planar(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0);

    // origin y affects loc of planar
    expect_to_planar(0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0);

    // radius affects loc of planar
    expect_to_planar(0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0);

    // loc affects loc of planar
    expect_to_planar(0.0, 0.0, 1.0, std::f32::consts::PI * 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

    // vel affects vel of planar
    expect_to_planar(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0);

    // radius affects loc of planar
    expect_to_planar(0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 2.0, 0.0);

    // acc affects acc of planar
    expect_to_planar(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0);
}

#[cfg(test)]
fn expect_to_planar(ox: f32, oy: f32, radius: f32, loc: f32, vel: f32, ax: f32, ay: f32, 
    exp_loc_x: f32, exp_loc_y: f32, exp_vel_x: f32, exp_vel_y: f32) {
    let subject = RadialBody{
        keyframe_countdown: 0.0,
        origin: Vector2::<f32>::new(ox, oy),
        radius,
        loc,
        vel,
        acc: Vector2::new(ax, ay)
    };

    let actual = subject.to_planar();

    assert_eq!(Vector2::new(ax, ay), actual.acc);
    assert_roughly_eq("loc.x", exp_loc_x, actual.loc.x);
    assert_roughly_eq("loc.y", exp_loc_y, actual.loc.y);
    assert_roughly_eq("vel.x", exp_vel_x, actual.vel.x);
    assert_roughly_eq("vel.y", exp_vel_y, actual.vel.y);
}

#[test]
fn test_keyframe() {
    expect_keyframe(0.0, 0.0, true);
    expect_keyframe(1.0, 0.0, false);
    expect_keyframe(0.0, 1.0, true);
    expect_keyframe(1.0, 1.0, true);

    let planar = PlanarBody::new(
        Vector2::<f32>::zeros(),
        Vector2::<f32>::zeros(),
        Vector2::<f32>::zeros()
    );

    let mut subject = planar.to_radial(Vector2::<f32>::zeros());

    assert_eq!(0.0, subject.keyframe_countdown);
    assert_eq!(true, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
    assert_eq!(KEYFRAME_PERIOD, subject.keyframe_countdown); 
}

#[cfg(test)]
fn expect_keyframe(keyframe_countdown: f32, step_size: f32, expect: bool) {
    let mut subject = RadialBody{
        keyframe_countdown,
        origin: Vector2::<f32>::zeros(),
        radius: 1.0,
        loc: 0.0,
        vel: 0.0,
        acc: Vector2::<f32>::zeros()
    };

    subject.update(step_size);

    assert_eq!(expect, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
}