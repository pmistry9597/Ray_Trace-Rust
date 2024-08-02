use crate::ray::{Hitable, HasHitInfo, InteractsWithRay};

pub mod sphere;
pub mod distant_cube_map;
pub mod triangle;

pub trait IsCompleteElement : Hitable + HasHitInfo + InteractsWithRay {}
pub type Element = Box<dyn IsCompleteElement + Send + Sync>;