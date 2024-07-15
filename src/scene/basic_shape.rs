use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay};
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
    pub mat: CommonMaterial,
}

// material stuff, likely needs to go in a new file because its SOOO important
pub struct CommonMaterial {
    pub emissive: Option<Vector3<f32>>,
    pub spec_or_diff: SpecDiff,
}
pub enum SpecDiff {
    Spec,
    Diff,
    Both,
}
fn spec(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    let d = ray.d - norm * 2.0 * ray.d.dot(&norm);
    Ray {d, o: o.clone()}
}
fn diff(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    // cosine sampling source: https://www.mathematik.uni-marburg.de/~thormae/lectures/graphics1/code/ImportanceSampling/importance_sampling_notes.pdf
    let xd = ray.d - norm * (ray.d.dot(&norm));
    let yd = xd.cross(norm);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();

    let phi = std::f32::consts::PI * 2.0 * u;
    let thet = v.sqrt().asin();

    // use nalgebra::vector;
    let cphi = phi.cos();
    let sthet = thet.sin();
    let sphi = phi.sin();
    let d = xd * (cphi * sthet) + yd * (sphi * sthet) + norm * thet.cos();
    // let d: Vec<f32> = vector![
    //     cphi * sthet,
    //     sphi * sthet,
    //     cthet,
    // ];
    Ray {d, o: o.clone()}
}
impl CommonMaterial {
    fn gen_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
        use SpecDiff::*;
        match self.spec_or_diff {
            Spec => {
                spec(ray, norm, o)
            },
            Diff => {
                diff(ray, norm, o)
            },
            Both => {
                // CHANGE THIS
                unimplemented!();
            }
        }
    }
}

impl InteractsWithRay for Sphere {
    fn shoot_new_ray(&self, ray: &Ray, hit_info: &HitInfo<Self::BounceInfo>) -> Ray {
        let o = &hit_info.pos;
        let norm = &hit_info.norm;

        self.mat.gen_new_ray(ray, norm, o)
    }
    fn does_dls(&self) -> bool {
        use SpecDiff::*;
        matches!(self.mat.spec_or_diff, Diff) || matches!(self.mat.spec_or_diff, Both)
    }
    fn emits(&self) -> bool {
        self.mat.emissive.is_some()
    }
}

impl HasHitInfo for Sphere {
    type BounceInfo = ();

    fn hit_info(&self, info: &HitResult<Self::Interm>) -> HitInfo<Self::BounceInfo> {
        use Coloring::*;
        let pos = info.intermed;
        let rgb = match &self.coloring {
            Solid(rgb) => *rgb,
            UsePos(coloring_fn) => coloring_fn(&pos, self),
        };
        let emissive = if let Some(emissive) = self.mat.emissive {
            emissive.clone()
        } else {
            use nalgebra::vector;
            vector![0.0,0.0,0.0]
        };
        let norm = (pos - self.c).normalize();
        HitInfo {rgb, emissive, pos, norm, bounce_info: Some(())}
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