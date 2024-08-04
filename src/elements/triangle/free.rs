use nalgebra::Vector3;
use super::{Triangle, GimmeNorm, GimmeRgb, DivertsRay};
use crate::material::*;
use crate::ray::Ray;

type UniformColor = Vector3<f32>;
pub type FreeTriangle = Triangle<[Vector3<f32>; 3], UniformNorm, UniformColor, DiffuseSpecNoBaseMaterial>;

pub struct UniformNorm(Vector3<f32>);

impl GimmeRgb for UniformColor {
    fn get_rgb(&self, _barycentric: &(f32, f32)) -> Vector3<f32> { *self }
}

impl GimmeNorm for UniformNorm {
    fn get_norm(&self, _barycentric: &(f32, f32)) -> Vector3<f32> { self.0 }
}

impl DivertsRay for DiffuseSpecNoBaseMaterial {
    type Seeding = SeedingRay;

    fn divert_ray_seed(&self, _ray: &Ray, _norm: &Vector3<f32>, _barycentric: &(f32, f32)) -> SeedingRay {
        self.generate_seed()
    }
    fn divert_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, seeding: &SeedingRay) -> (Ray, f32) {
        self.gen_new_ray(ray, norm, o, seeding)
    }
}

impl From<Vector3<f32>> for UniformNorm {
    fn from(n: Vector3<f32>) -> Self { UniformNorm(n) }
}