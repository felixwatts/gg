use nalgebra::Vector2;
use crate::component::radial_body::RadialBody;
use crate::component::planar_body::PlanarBody;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Body {
    Planar(PlanarBody),
    Radial(RadialBody)
}

impl Body {
    pub fn new(loc: Vector2::<f32>, vel: Vector2::<f32>, acc: Vector2::<f32>) -> Body {
        Body::Planar(PlanarBody{
            keyframe: true,
            loc,
            vel,
            accel: acc
        })
    }

    pub fn step(&mut self, duration: f32) {
        match self {
            Body::Planar(b) => b.update(duration),
            Body::Radial(b) => b.update(duration)
        }
    }

    pub fn get_loc(&self) -> Vector2::<f32> {
        match self {
            Body::Planar(b) => b.loc,
            Body::Radial(b) => b.to_planar().loc
        }
    }

    pub fn get_is_attached(&self) -> bool {
        match self {
            Body::Planar(_) => false,
            Body::Radial(_) => true
        }
    }

    pub fn get_is_keyframe_and_reset(&mut self) -> bool {
        match self {
            Body::Planar(b) => { let result = b.keyframe; b.keyframe = false; result },
            Body::Radial(b) => { let result = b.keyframe; b.keyframe = false; result }
        }
    }

    pub fn get_acc(&self) -> Vector2::<f32> {
        match self {
            Body::Planar(b) => b.accel,
            Body::Radial(b) => b.accel
        }
    }

    pub fn set_acc(&mut self, acc: Vector2::<f32>) {
        match self {
            Body::Planar(b) => { b.accel = acc; b.keyframe = true; }
            Body::Radial(b) => { b.accel = acc; b.keyframe = true; }
        }
    }

    pub fn to_attached(&self, origin: Vector2::<f32>) -> Body {
        match self {
            Body::Planar(b) => Body::Radial(b.to_radial(origin)),
            Body::Radial(_) => panic!("already attached")
        }
    }

    pub fn to_detached(&self) -> Body {
        match self {
            Body::Planar(_) => panic!("already detached"),
            Body::Radial(b) => Body::Planar(b.to_planar())
        }
    }
}