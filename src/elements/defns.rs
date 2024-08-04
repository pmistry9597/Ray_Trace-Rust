use crate::ray::{Hitable, HasHitInfo, InteractsWithRay};

pub trait IsCompleteElement : Hitable + HasHitInfo + InteractsWithRay {}

pub type Renderable<'r> = &'r (dyn IsCompleteElement + Send + Sync); // what ends up getting used by rendering fns in the end

pub type Element<'a> = Box<dyn IsCompleteElement + Send + Sync + 'a>;

pub type Group = Box<dyn Decomposable + Send + Sync>; // what meshes do as part of the render list

pub trait Decomposable {
    fn decompose_to_elems<'e, 's>(&'s self) -> Box<dyn Iterator<Item = Element<'e>> + 's> 
    where
        's : 'e;
}