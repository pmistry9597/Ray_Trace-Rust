use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo};

pub struct Sphere {
    pub c: Vector3<f32>,
    pub r: f32,

    pub rgb: Vector3<f32>,
}

impl HasHitInfo for Sphere {
    fn hit_info(&self, info: &HitResult<Self::Interm>) -> HitInfo {
        HitInfo {rgb: self.rgb}
    }
}

impl Hitable for Sphere {
    type Interm = ();

    fn intersect(&self, ray: &Ray) -> Option<HitResult<()>> {
        // solve quadratic equation for sphere-ray intersection, from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = ray.o - self.c;
        let dir = ray.d.dot(&oc);
        let consts = oc.dot(&oc) - self.r * self.r;

        let thing2 = dir * dir - consts;
        if thing2 > 0.0 {
            let offset = dir.abs();
            let thing = thing2.sqrt();
            let ls = [offset + thing, offset - thing];

            match ls.into_iter().filter(|e| *e > 0.0).reduce(|prev, e| if e < prev {e} else {prev}) {
                Some(l) => Some(HitResult{l, intermed: ()}),
                None => None,
            }
        } else {
            None
        }
    }
}