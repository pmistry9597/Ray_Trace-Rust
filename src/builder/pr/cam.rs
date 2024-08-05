use serde::Deserialize;
use nalgebra::{Vector3, Matrix4};
use crate::scene;

#[derive(Deserialize, Debug)]
pub struct Cam {
    pub d: Vector3<f32>, // o -> center of screen, has distance
    pub o: Vector3<f32>,
    pub up: Vector3<f32>, // should be unit vector
    // in-scene dimensions, not view pixels
    pub screen_width: f32, 
    pub screen_height: f32,
    pub lens_r: Option<f32>,

    view_eulers: [f32; 3],
}

impl From<Cam> for scene::Cam {
    fn from(c_: Cam) -> Self {
        let Cam {
            d, o, up, screen_width, screen_height, lens_r, view_eulers
        } = c_;
        let [r, p, y] = view_eulers;

        let rot = Matrix4::<f32>::from_euler_angles(r, p, y);
        let rot = rot.fixed_resize::<3,3>(0.0);
        let d = rot * d;
        let up = rot * up;

        Self { d, o, up, screen_width, screen_height, lens_r }
    }
}