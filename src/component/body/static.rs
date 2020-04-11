use nalgebra::Vector2;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct StaticBody{
    pub(super) loc: Vector2::<f32>,
    keyframe: bool
}

impl StaticBody{
    pub fn new(loc: Vector2::<f32>) -> StaticBody {
        StaticBody{
            loc,
            keyframe: true
        }
    }

    pub fn get_is_keyframe_and_reset(&mut self) -> bool {
        let result = self.keyframe; 
        self.keyframe = false; 
        result
    }
}

#[test]
fn test_keyframe() {
    let mut subject = StaticBody::new(Vector2::zeros());
    assert_eq!(true, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
    assert_eq!(false, subject.get_is_keyframe_and_reset());
}