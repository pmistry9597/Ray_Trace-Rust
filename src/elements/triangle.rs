use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::material::*;
use crate::elements::IsCompleteElement;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FreeTriangle {
    pub verts: [Vector3<f32>; 3],
    pub norm: Vector3<f32>, // should be normalized to unit

    pub rgb: Vector3<f32>,
    pub mat: DiffuseSpecNoBaseMaterial,
}

impl IsCompleteElement for FreeTriangle {}

struct ContinueInfo {
    seeding: SeedingRay,
}

impl InteractsWithRay for FreeTriangle {
    fn continue_ray(&self, ray: &Ray, hit_info: &HitInfo) -> Option<(Vector3<f32>, Ray, f32)> { 
        let seeding = &hit_info.continue_info.as_ref().unwrap().downcast_ref::<ContinueInfo>().unwrap().seeding;

        let (ray, p) = self.mat.gen_new_ray(ray, &hit_info.norm, &hit_info.pos, &seeding);

        Some((self.rgb, ray, p))
    }
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> { None } // maybe ill do this? will i use a light source that has triangles?
}

impl HasHitInfo for FreeTriangle {
    fn hit_info(&self, info: &HitResult, ray: &Ray) -> HitInfo {
        use nalgebra::vector;

        let continue_info = ContinueInfo { seeding: self.mat.generate_seed() };
        let emissive = match self.mat.emissive {
            Some(e) => e,
            None => {
                use nalgebra::vector;
                vector![0.0,0.0,0.0]
            }
        };

        HitInfo {
            emissive, //: vector![0.7,0.7,1.0] * atten + red_comp,
            pos: ray.d * info.l.0 + ray.o,
            norm: self.norm,
            dls: false,
            continue_info: Some(Box::new(continue_info)),
        }
    }
}

impl Hitable for FreeTriangle {
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
                        Some(HitResult{l: l.into(), intermed: None})
                    }
                }
            }
        }
    }
}