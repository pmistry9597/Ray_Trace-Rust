use serde::Deserialize;
use crate::elements::sphere::Sphere;

#[derive(Deserialize, Debug)]
pub struct VecInto<T>(Vec<T>);

impl<A, B> From<VecInto<A>> for Vec<B> 
where
    B: From<A>
{
    fn from(val: VecInto<A>) -> Self {
        let VecInto(contents) = val;
        contents.into_iter().map(|t| t.into()).collect()
    }
}

#[derive(Deserialize, Debug)]
pub enum ElementType {
    Sphere(Sphere), //{c: [f32; 3], r: f32, coloring: [f32; 3], mat: CommonMaterial},
}

impl From<ElementType> for Sphere {
    fn from(val: ElementType) -> Self {
        use ElementType::*;
        match val {
            Sphere(s) => s
        }
    }
}