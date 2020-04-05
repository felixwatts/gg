use nalgebra::Vector2;
use crate::component::radial_body::RadialBody;
use crate::component::planar_body::PlanarBody;
use serde::{Serialize, Deserialize};

pub const KEYFRAME_PERIOD: f32 = 0.25f32;

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Body {
    Static(StaticBody),
    Planar(PlanarBody),
    Radial(RadialBody)
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct StaticBody{
    loc: Vector2::<f32>,
    keyframe: bool
}

impl Body {
    pub fn new_static(loc: Vector2::<f32>) -> Body {
        Body::Static(StaticBody{
            keyframe: true,
            loc
        })
    }

    pub fn new_dynamic(loc: Vector2::<f32>, vel: Vector2::<f32>, acc: Vector2::<f32>) -> Body {
        Body::Planar(PlanarBody{
            keyframe_countdown: 0.0,
            loc,
            vel,
            acc
        })
    }

    pub fn step(&mut self, duration: f32) {
        match self {
            Body::Static(_) => {},
            Body::Planar(b) => b.update(duration),
            Body::Radial(b) => b.update(duration)
        }
    }

    pub fn get_loc(&self) -> Vector2::<f32> {
        match self {
            Body::Static(b) => b.loc,
            Body::Planar(b) => b.loc,
            Body::Radial(b) => b.to_planar().loc
        }
    }

    pub fn get_is_attached(&self) -> bool {
        match self {
            Body::Static(_) => false,
            Body::Planar(_) => false,
            Body::Radial(_) => true
        }
    }

    pub fn get_is_keyframe_and_reset(&mut self) -> bool {
        match self {
            Body::Static(b) => { let result = b.keyframe; b.keyframe = false; result },
            Body::Planar(b) => b.get_is_keyframe_and_reset(),
            Body::Radial(b) => b.get_is_keyframe_and_reset()
        }
    }

    pub fn set_acc(&mut self, acc: Vector2::<f32>) {
        match self {
            Body::Static(_) => panic!("cannot modify a static body"),
            Body::Planar(b) => { b.acc = acc; b.keyframe_countdown = 0.0; }
            Body::Radial(b) => { b.acc = acc; b.keyframe_countdown = 0.0; }
        }
    }

    pub fn to_attached(&self, origin: Vector2::<f32>) -> Body {
        match self {
            Body::Static(_) => panic!("cannot modify a static body"),
            Body::Planar(b) => Body::Radial(b.to_radial(origin)),
            Body::Radial(_) => panic!("already attached")
        }
    }

    pub fn to_detached(&self) -> Body {
        match self {
            Body::Static(_) => panic!("cannot modify a static body"),
            Body::Planar(_) => panic!("already detached"),
            Body::Radial(b) => Body::Planar(b.to_planar())
        }
    }
}