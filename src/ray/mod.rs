use nalgebra::Vector3;

pub use hit::*;
pub use generate::*;

mod hit;
mod generate;

#[derive(Clone)]
pub struct Ray {
    pub d: Vector3<f32>, // should be unit vector
    pub o: Vector3<f32>,
}