use nalgebra::Vector3;
use super::{Triangle, GimmeNorm};

pub type FreeTriangle = Triangle<[Vector3<f32>; 3], UniformNorm>;

pub struct UniformNorm(Vector3<f32>);

impl GimmeNorm for UniformNorm {
    fn get_norm(&self, _pos: &Vector3<f32>) -> Vector3<f32> { self.0 }
}

impl From<Vector3<f32>> for UniformNorm {
    fn from(n: Vector3<f32>) -> Self { UniformNorm(n) }
}