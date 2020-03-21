use crate::component::planar_body::PlanarBody;
use nalgebra::Vector2;

#[derive(Clone)]
pub struct RadialBody {
    pub origin: Vector2<f32>,
    pub radius: f32,
    pub loc: f32,
    pub vel: f32,

    // acceleration is expressed in planar coordinates
    pub accel: Vector2<f32>,
}

impl RadialBody {

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

    fn accel_along_tangent(&self) -> f32 {
        nalgebra::Matrix::dot(&self.tangent(), &self.accel)
    }

    pub fn update(&mut self, duration: f32) {
        let accel = self.accel_along_tangent();
        self.loc += (self.vel + (accel * duration / 2.0)) * duration;
        self.vel += accel * duration;
    }

    pub fn to_planar(&self) -> PlanarBody {
        let loc = self.origin + self.radius();
        let vel = self.tangent() * self.vel;
        let accel = self.accel;
        PlanarBody{
            loc,
            vel,
            accel
        }
    }

    // pub fn to_planar(&self, ecs: &Ecs) -> RadialBody {
    //     let origin_loc_vel : RadialBody = ecs.get(self.origin).unwrap();

    //     let dx = self.radius * self.loc.sin();
    //     let dy = self.radius * self.loc.cos();

    //     let radius = nalgebra::Vector2::new(dx, dy);

    //     let loc_planar = origin_loc_vel.loc + radius;

    //     let tangent = nalgebra::Vector2::new(dy, dx);
    //     let vel_planar = tangent * self.vel;

    //     RadialBody {
    //         loc: loc_planar,
    //         vel: vel_planar
    //     }
    // }
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
fn assert_roughly_eq(expected: f32, actual: f32) {
    assert!((expected - actual).abs() < 0.0001, "{} != {}", expected, actual);
}

#[cfg(test)]
fn expect_accel_along_tangent(loc: f32, radius: f32, ax: f32, ay: f32, expected: f32) {
    let subject = RadialBody {
        origin: Vector2::<f32>::zeros(),
        radius: radius,
        loc: loc,
        vel: 0.0,
        accel: Vector2::new(ax, ay),
    };

    let actual = subject.accel_along_tangent();

    assert_roughly_eq(expected, actual);
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
        origin: Vector2::<f32>::zeros(),
        radius: radius,
        loc: loc,
        vel :0.0,
        accel: Vector2::<f32>::zeros(),
    };

    let actual = subject.radius();

    assert_roughly_eq(x, actual.x);
    assert_roughly_eq(y, actual.y);
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
        origin: Vector2::<f32>::zeros(),
        radius: radius,
        loc: loc,
        vel: 0.0,
        accel: Vector2::<f32>::zeros(),
    };

    let actual = subject.tangent();

    assert_roughly_eq(x, actual.x);
    assert_roughly_eq(y, actual.y);
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
fn expect_update(radius: f32, loc: f32, vel: f32, ax: f32, ay: f32, t: f32, exp_loc: f32, exp_vel: f32) {
    let mut subject = RadialBody{
        origin: Vector2::<f32>::zeros(),
        radius,
        loc,
        vel,
        accel: Vector2::new(ax, ay)
    };

    subject.update(t);

    assert_eq!(Vector2::<f32>::zeros(), subject.origin);
    assert_eq!(Vector2::<f32>::new(ax, ay), subject.accel);
    assert_eq!(radius, subject.radius);
    assert_roughly_eq(exp_loc, subject.loc);
    assert_roughly_eq(exp_vel, subject.vel);
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

    // acc affects acc of planar
    expect_to_planar(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0);
}

#[cfg(test)]
fn expect_to_planar(ox: f32, oy: f32, radius: f32, loc: f32, vel: f32, ax: f32, ay: f32, 
    exp_loc_x: f32, exp_loc_y: f32, exp_vel_x: f32, exp_vel_y: f32) {
    let subject = RadialBody{
        origin: Vector2::<f32>::new(ox, oy),
        radius,
        loc,
        vel,
        accel: Vector2::new(ax, ay)
    };

    let actual = subject.to_planar();

    assert_eq!(Vector2::new(ax, ay), actual.accel);
    assert_roughly_eq(exp_loc_x, actual.loc.x);
    assert_roughly_eq(exp_loc_y, actual.loc.y);
    assert_roughly_eq(exp_vel_x, actual.vel.x);
    assert_roughly_eq(exp_vel_y, actual.vel.y);
}