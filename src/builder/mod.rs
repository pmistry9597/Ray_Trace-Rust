use serde::Deserialize;
use serde_yaml;
use crate::scene::Cam;
use crate::render::RenderInfo;
use inner::*;

mod inner;
mod pr;

#[derive(Deserialize, Debug)]
pub struct Scheme {
    pub render_info: RenderInfo,
    pub cam: Cam,
    pub scene_members: VecInto<MemberTypes>,
}

impl Scheme {
    pub fn from_yml(contents: String) -> Scheme {
        let scheme: Scheme = serde_yaml::from_str(&contents).expect("dodnt parse!!");
        scheme.apply_corrections()
    }

    fn apply_corrections(mut self) -> Self {
        self.cam.up = self.cam.up.normalize();
        self
    }
}