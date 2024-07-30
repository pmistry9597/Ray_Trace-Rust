use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;

// for environment mapping, all rays can hit this
pub struct DistantCubeMap {}

impl IsCompleteElement for DistantCubeMap {}

impl InteractsWithRay for DistantCubeMap {
    fn shoot_new_ray(&self, _ray: &Ray, _hit_info: &HitInfo) -> Option<(Ray, f32)> { None } // cant shoot new ray silly
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> { None } // maybe ill do this? for a skybox it seems almost unnecessary since all rays can hit
}

impl HasHitInfo for DistantCubeMap {
    fn hit_info(&self, _info: &HitResult, ray: &Ray) -> HitInfo {
        use nalgebra::vector;
        HitInfo {
            rgb: vector![0.0,0.0,0.0],
            emissive: vector![0.8,0.8,1.0] * 0.1,
            pos: ray.d * f32::INFINITY,
            norm: -ray.d,
            dls: false,
            bounce_info: None,
        }
    }
}

impl Hitable for DistantCubeMap {
    fn intersect(&self, _ray: &Ray) -> Option<HitResult> { // always hits since distant and covers all
        Some(HitResult{l: f32::INFINITY.into(), intermed: None})
    }
}