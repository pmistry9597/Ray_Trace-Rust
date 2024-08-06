use nalgebra::Vector3;

pub use hit::*;
pub use generate::*;
pub use closest_hit::closest_ray_hit;

mod hit;
mod generate;
mod closest_hit;

#[derive(Clone)]
pub struct Ray {
    pub d: Vector3<f32>, // should be unit vector
    pub o: Vector3<f32>,
}