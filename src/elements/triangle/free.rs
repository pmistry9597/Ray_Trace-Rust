use nalgebra::Vector3;
use super::{Triangle, GimmeNorm, GimmeRgb};

type UniformColor = Vector3<f32>;
pub type FreeTriangle = Triangle<[Vector3<f32>; 3], UniformNorm, UniformColor>;

pub struct UniformNorm(Vector3<f32>);

impl GimmeRgb for UniformColor {
    fn get_rgb(&self, _barycentric: &(f32, f32)) -> Vector3<f32> { *self }
}

impl GimmeNorm for UniformNorm {
    fn get_norm(&self, _barycentric: &(f32, f32)) -> Vector3<f32> { self.0 }
}

impl From<Vector3<f32>> for UniformNorm {
    fn from(n: Vector3<f32>) -> Self { UniformNorm(n) }
}