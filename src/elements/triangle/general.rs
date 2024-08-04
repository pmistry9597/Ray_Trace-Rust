use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;
use std::ops::Index;

// #[derive(Deserialize, Debug)]
pub struct Triangle<V, N, C, D> 
{
    pub verts: V,
    pub norm: N, // should be normalized to unit

    pub rgb: C,
    pub diverts_ray: D,
}

pub trait GimmeNorm {
    fn get_norm(&self, barycentric: &(f32, f32)) -> Vector3<f32>;
}

pub trait GimmeRgb {
    fn get_rgb(&self, barycentric: &(f32, f32)) -> Vector3<f32>;
}

pub trait DivertsRay {
    type Seeding;
    fn divert_ray_seed(&self, barycentric: &(f32, f32)) -> Self::Seeding;
    fn divert_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, seeding: &Self::Seeding) -> (Ray, f32);
}

type Barycentric = (f32, f32); // u, v barycentric, w calculated as 1 - u - v
#[derive(Clone)]
struct Intermed {
    baryc: Barycentric
}

impl<V, N, C, D, S: 'static> IsCompleteElement for Triangle<V, N, C, D> 
where
    V : Index<usize, Output = Vector3<f32>>,
    N : GimmeNorm,
    C : GimmeRgb,
    D : DivertsRay<Seeding = S>,
{}

struct ContinueInfo<S> {
    seeding: S,
    baryc: Barycentric,
}

impl<V, N, C, D, S: 'static> InteractsWithRay for Triangle<V, N, C, D> 
where
    V : Index<usize, Output = Vector3<f32>>,
    N : GimmeNorm,
    C : GimmeRgb,
    D : DivertsRay<Seeding = S>,
{
    fn continue_ray(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Vector3<f32>, Ray)> { 
        let cont_info: &ContinueInfo<S> = &hit_info.continue_info.as_ref().unwrap().downcast_ref().unwrap();
        // let seeding = cont_info.seeding;

        let (ray, p) = self.diverts_ray.divert_new_ray(ray, &hit_info.norm, &hit_info.pos, &cont_info.seeding);

        // let intermed: &ContinueInfo = &hit_info.continue_info.as_ref().unwrap().downcast_ref().unwrap();
        let rgb = self.rgb.get_rgb(&cont_info.baryc);

        Some((rgb / p, ray))
    }
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> { None } // maybe ill do this? will i use a light source that has triangles?
}

impl<V, N, C, D, S: 'static> HasHitInfo for Triangle<V, N, C, D> 
where
    V : Index<usize, Output = Vector3<f32>>,
    N : GimmeNorm,
    C : GimmeRgb,
    D : DivertsRay<Seeding = S>,
{
    fn hit_info(&self, info: &HitResult, ray: &Ray) -> HitInfo {
        let intermed: &Intermed = &info.intermed.as_ref().unwrap().downcast_ref().unwrap();
        let continue_info = ContinueInfo { seeding: self.diverts_ray.divert_ray_seed(&intermed.baryc), baryc: intermed.baryc.clone() };

        let norm = self.norm.get_norm(&intermed.baryc);
        let pos = ray.d * info.l.0 + ray.o + norm * crate::EPS; // create offset from surface to prevent errors

        HitInfo {
            emissive: Vector3::zeros(),
            pos,
            norm,
            dls: false,
            continue_info: Some(Box::new(continue_info)),
        }
    }
}

impl<V, N, C, D, S: 'static> Hitable for Triangle<V, N, C, D> 
where
    V : Index<usize, Output = Vector3<f32>>,
    N : GimmeNorm,
    C : GimmeRgb,
    D : DivertsRay<Seeding = S>,
{
    fn intersect(&self, ray: &Ray) -> Option<HitResult> { // always hits since distant and covers all
        // adapted moller trumbore from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        // for rapid intersection test using cramer's rule to solve for barycentric coordinates

        let e1 = self.verts[1] - self.verts[0];
        let e2 = self.verts[2] - self.verts[0];
        let ray_x_e2 = ray.d.cross(&e2);
        let det = e1.dot(&ray_x_e2);

        if det.abs() < crate::EPS { // means triangle is parallel to ray
            None
        } else {
            let inv_det = 1.0 / det;
            let rhs = ray.o - self.verts[0];
            let u = inv_det * rhs.dot(&ray_x_e2);

            if u < 0.0 || u > 1.0 {
                None
            } else {
                let rhs_x_e1 = rhs.cross(&e1);
                let v = inv_det * ray.d.dot(&rhs_x_e1);

                if v < 0.0 || (u + v) > 1.0 {
                    None
                } else {
                    let l = inv_det * e2.dot(&rhs_x_e1);

                    if l < crate::EPS {
                        None
                    } else {
                        Some(HitResult{l: l.into(), intermed: Some(Box::new(Intermed{baryc: (u, v)}))})
                    }
                }
            }
        }
    }
}