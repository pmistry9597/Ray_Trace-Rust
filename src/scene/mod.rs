use nalgebra::Vector3;
use basic_shape::Sphere;

pub mod test_rig;
mod basic_shape;
mod render_tools;
mod radiance;
pub use render_tools::*;

pub struct Cam {
    pub d: Vector3<f32>, // o -> center of screen, has distance
    pub o: Vector3<f32>,
    pub up: Vector3<f32>, // should be unit vector
    // in-scene dimensions, not view pixels
    pub screen_width: f32, 
    pub screen_height: f32,
}

pub struct Scene {
    pub cam: Cam,
    objs: Vec<Sphere>,
}