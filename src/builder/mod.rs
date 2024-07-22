use serde::Deserialize;
use serde_yaml;
use crate::scene;

#[derive(Deserialize, Debug)]
pub struct Scheme {
    base: Base,
    cam: Cam,
}

impl Scheme {
    pub fn render_info(&self) -> scene::RenderInfo {
        scene::RenderInfo {
            samps_per_pix: self.base.samps_per_pix,
            dir_light_samp: self.base.dir_light_samp,
        }
    }
    pub fn cam(&self) -> scene::Cam {
        scene::Cam {
            d: self.cam.d.into(),
            o: self.cam.o.into(),
            up: self.cam.up.into(),
            screen_width: self.cam.screen_width,
            screen_height: self.cam.screen_height,
            lens_r: self.cam.lens_r,
        }
    }
    pub fn from_yml(contents: String) -> Scheme {
        serde_yaml::from_str(&contents).expect("dodnt work!!")
    }
}

#[derive(Deserialize, Debug)]
struct Base {
    samps_per_pix: i32,
    dir_light_samp: bool,
}

#[derive(Deserialize, Debug)]
struct Cam {
    d: [f32; 3],
    o: [f32; 3],
    up: [f32; 3],
    screen_width: f32, 
    screen_height: f32,
    lens_r: Option<f32>,
}