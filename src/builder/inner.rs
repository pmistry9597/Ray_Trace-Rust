use serde::Deserialize;
use crate::elements::sphere::Sphere;
use crate::elements::Element;

#[derive(Deserialize, Debug)]
pub struct VecInto<T>(Vec<T>); // wrapper st if elements have into one type to another, easily convert this vec into vec of another

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

impl From<ElementType> for Element {
    fn from(val: ElementType) -> Self {
        use ElementType::*;
        match val {
            Sphere(s) => Box::new(s)
        }
    }
}