// precursor structs for constructing elements
use serde::Deserialize;
use crate::elements::distant_cube_map::FaceImagewUVScale;

#[derive(Deserialize, Debug)]
pub struct PathwUVScale(String, f32, f32);

// for environment mapping, all rays can hit this
#[derive(Deserialize, Debug)]
pub struct DistantCubeMap {
    pub neg_z: PathwUVScale,
    pub pos_z: PathwUVScale,
    pub neg_x: PathwUVScale,
    pub pos_x: PathwUVScale,
    pub neg_y: PathwUVScale,
    pub pos_y: PathwUVScale,
}

impl From<PathwUVScale> for FaceImagewUVScale {
    fn from(pathwuvscale: PathwUVScale) -> Self { 
        let PathwUVScale (path, us, vs) = pathwuvscale;
        (image::open(path).unwrap().into_rgb32f().into(), us, vs)
    }
}