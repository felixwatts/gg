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
fn test_update_applies_vel_to_pos() {
    let loc = Vector2::<f32>::zeros();
    let vel = Vector2::new(3.0, -2.0);
    let accel = Vector2::zeros();
    let time = 1.0;

    let mut subject = PlanarBody{
        loc,
        vel,
        accel
    };

    subject.update(time);

    assert_eq!(3.0, subject.loc.x);
    assert_eq!(-2.0, subject.loc.y);
    assert_eq!(3.0, subject.vel.x);
    assert_eq!(-2.0, subject.vel.y);
}

#[test]
fn test_update_applies_vel_to_pos_for_time() {
    let loc = Vector2::<f32>::zeros();
    let vel = Vector2::new(3.0, -2.0);
    let accel = Vector2::zeros();
    let time = 0.5;

    let mut subject = PlanarBody{
        loc,
        vel,
        accel
    };

    subject.update(time);

    assert_eq!(1.5, subject.loc.x);
    assert_eq!(-1.0, subject.loc.y);
    assert_eq!(3.0, subject.vel.x);
    assert_eq!(-2.0, subject.vel.y);
}

#[test]
fn test_update_applies_accel_to_vel() {
    let loc = Vector2::<f32>::zeros();
    let vel = Vector2::new(3.0, -2.0);
    let accel = Vector2::new(0.0, -5.0);
    let time = 1.0;

    let mut subject = PlanarBody{
        loc,
        vel,
        accel
    };

    subject.update(time);

    assert_eq!(3.0, subject.loc.x);
    assert_eq!(-4.5, subject.loc.y);
    assert_eq!(3.0, subject.vel.x);
    assert_eq!(-7.0, subject.vel.y);
}

#[test]
fn test_update_applies_accel_to_vel_for_time() {
    let loc = Vector2::<f32>::zeros();
    let vel = Vector2::new(3.0, -2.0);
    let accel = Vector2::new(0.0, -5.0);
    let time = 0.5;

    let mut subject = PlanarBody{
        loc,
        vel,
        accel
    };

    subject.update(time);

    assert_eq!(1.5, subject.loc.x);
    assert_eq!(-1.625, subject.loc.y);
    assert_eq!(3.0, subject.vel.x);
    assert_eq!(-4.5, subject.vel.y);
}