use nalgebra::Vector3;
use serde::Deserialize;
use crate::elements::{Element, Group};

#[derive(Deserialize, Debug)]
pub struct Cam {
    pub d: Vector3<f32>, // o -> center of screen, has distance
    pub o: Vector3<f32>,
    pub up: Vector3<f32>, // should be unit vector
    // in-scene dimensions, not view pixels
    pub screen_width: f32, 
    pub screen_height: f32,
    pub lens_r: Option<f32>,
}

pub struct Scene {
    pub cam: Cam,
    pub members: Vec<Member>,
}

pub enum Member {
    Elem(Element),
    Grp(Group),
}