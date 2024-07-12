use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo};
use std::sync::Arc;

pub enum Coloring<S> {
    Solid(Vector3<f32>),
    UsePos(Arc<dyn Fn(&Vector3<f32>, &S) -> Vector3<f32> + Send + Sync>),
}

pub struct Sphere {
    pub c: Vector3<f32>,
    pub r: f32,

    // pub rgb: Vector3<f32>,
    pub coloring: Coloring<Self>,
}

impl HasHitInfo for Sphere {
    fn hit_info(&self, info: &HitResult<Self::Interm>) -> HitInfo {
        use Coloring::*;
        let pos = info.intermed;
        let rgb = match &self.coloring {
            Solid(rgb) => *rgb,
            UsePos(coloring_fn) => coloring_fn(&pos, self),
        };
        HitInfo {rgb}
    }
}

impl Hitable for Sphere {
    type Interm = Vector3<f32>;

    fn intersect(&self, ray: &Ray) -> Option<HitResult<Self::Interm>> {
        // solve quadratic equation for sphere-ray intersection, from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = ray.o - self.c;
        let dir = ray.d.dot(&oc);
        let consts = oc.dot(&oc) - self.r * self.r;

        let thing2 = dir * dir - consts;
        if thing2 > 0.0 {
            let offset = -dir;
            let thing = thing2.sqrt();
            let ls = [offset + thing, offset - thing];

            match ls.into_iter().filter(|e| *e > 0.0).reduce(|prev, e| if e < prev {e} else {prev}) {
                Some(f) => {
                    Some(HitResult{l: f.into(), intermed: (ray.o + ray.d * f)})
                },
                None => None,
            }
        } else {
            None
        }
    }
}