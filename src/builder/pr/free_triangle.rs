use nalgebra::Vector3;
use crate::material::DiffuseSpecNoBaseMaterial;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FreeTriangle {
    pub verts: [Vector3<f32>; 3],
    pub norm: Vector3<f32>,

    pub rgb: Vector3<f32>,
    pub mat: DiffuseSpecNoBaseMaterial,
}

