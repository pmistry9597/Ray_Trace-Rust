use serde::Deserialize;
use serde_yaml;
use crate::scene::{Cam, RenderInfo};
// use inner::*;

// mod inner;

#[derive(Deserialize, Debug)]
pub struct Scheme {
    pub render_info: RenderInfo,
    pub cam: Cam,
    // pub scene_objs: Vec<ObjectType>,
}

impl Scheme {
    // pub fn render_info(&self) -> RenderInfo {
    //     scene::RenderInfo {
    //         samps_per_pix: self.base.samps_per_pix,
    //         dir_light_samp: self.base.dir_light_samp,
    //     }
    // }
    // pub fn cam(&self) -> Cam {
    //     scene::Cam {
    //         d: self.cam.d.into(),
    //         o: self.cam.o.into(),
    //         up: self.cam.up.into(),
    //         screen_width: self.cam.screen_width,
    //         screen_height: self.cam.screen_height,
    //         lens_r: self.cam.lens_r,
    //     }
    // }
    pub fn from_yml(contents: String) -> Scheme {
        serde_yaml::from_str(&contents).expect("dodnt work!!")
    }
}