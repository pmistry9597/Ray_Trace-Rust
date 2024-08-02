use crate::ray::{Hitable, HasHitInfo, InteractsWithRay};

pub mod sphere;
pub mod distant_cube_map;
pub mod triangle;
pub mod mesh;

pub trait IsCompleteElement : Hitable + HasHitInfo + InteractsWithRay {}
pub type Renderable<'r> = &'r (dyn IsCompleteElement + Send + Sync); // what ends up getting used by rendering fns in the end
pub type Element = Box<dyn IsCompleteElement + Send + Sync>;
pub type Group = Box<dyn Decomposable + Send + Sync>;

pub trait Decomposable {
    fn decompose_to_elems(&self) -> Box<dyn Iterator<Item = Renderable> + '_>; //Box<dyn Iterator<Item = Element> + '_>;
}