use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::material::*;
use serde::Deserialize;
use crate::elements::IsCompleteElement;

#[derive(Deserialize, Debug)]
// pub enum Coloring<S> {
pub enum Coloring {
    Solid(Vector3<f32>),
    // UsePos(Arc<dyn Fn(&Vector3<f32>, &S) -> Vector3<f32> + Send + Sync>),
}

pub struct BounceInfo {
    seeding: SeedingRay,
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub c: Vector3<f32>,
    pub r: f32,

    // pub rgb: Vector3<f32>,
    pub coloring: Coloring, //<Self>,
    pub mat: DiffuseSpecNoBaseMaterial,
}

impl IsCompleteElement for Sphere {}

impl InteractsWithRay for Sphere {
    fn continue_ray(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Vector3<f32>, Ray, f32)> {
        let o = &hit_info.pos;
        let norm = &hit_info.norm;
        // let bounce_info = &hit_info.bounce_info.as_ref().unwrap();
        let seeding = &hit_info.continue_info.as_ref().unwrap().downcast_ref::<BounceInfo>().unwrap().seeding;
        use Coloring::*;
        let rgb = match self.coloring {
            Solid(c) => c,
        };
        let (ray, p) = self.mat.gen_new_ray(ray, norm, o, &seeding);

        Some((rgb, ray, p))
    }
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> {
        match self.mat.emissive {
            Some(_) => Some(Box::new(DLSEmitter_{sp: self})),
            None => None,
        }
    }
}

struct DLSEmitter_<'a> {
    sp: &'a Sphere,
}
impl<'a> DLSEmitter for DLSEmitter_<'a> {
    fn dls_ray(&self, pos: &Vector3<f32>, _norm: &Vector3<f32>) -> Vector3<f32> {
        (self.sp.c - pos).normalize()
    }
}

impl HasHitInfo for Sphere {
    fn hit_info(&self, info: &HitResult, _ray: &Ray) -> HitInfo {
        let perfect_pos: &Vector3<f32> = &info.intermed.as_ref().unwrap().downcast_ref().unwrap();
        let norm = (perfect_pos - self.c).normalize();

        let pos = perfect_pos + norm * crate::EPS; // create offset from surface to prevent errors
        let emissive = if let Some(emissive) = self.mat.emissive {
            emissive.clone()
        } else {
            use nalgebra::vector;
            vector![0.0,0.0,0.0]
        };
        let continue_info = BounceInfo { seeding: self.mat.generate_seed() };

        HitInfo {emissive, pos, norm, dls: self.mat.should_dls(&continue_info.seeding), continue_info: Some(Box::new(continue_info))}
    }
}

impl Hitable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitResult> {
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
                    let pos = ray.o + ray.d * f;
                    Some(HitResult{l: f.into(), intermed: Some(Box::new(pos))})
                },
                None => None,
            }
        } else {
            None
        }
    }
}