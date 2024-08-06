use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;
use crate::material::UVRgb32FImage;
use crate::accel::Aabb;

pub type FaceImagewUVScale = (UVRgb32FImage, f32, f32);

// for environment mapping, all rays can hit this
pub struct DistantCubeMap {
    pub neg_z: FaceImagewUVScale,
    pub pos_z: FaceImagewUVScale,
    pub neg_x: FaceImagewUVScale,
    pub pos_x: FaceImagewUVScale,
    pub neg_y: FaceImagewUVScale,
    pub pos_y: FaceImagewUVScale,
}

impl IsCompleteElement for DistantCubeMap {}

impl InteractsWithRay for DistantCubeMap {
    fn continue_ray(&self, _ray: &Ray, _hit_info: &HitInfo) -> Option<(Vector3<f32>, Ray)> { None } // cant shoot new ray silly
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> { None } // maybe ill do this? for a skybox it seems almost unnecessary since all rays can hit
}

impl HasHitInfo for DistantCubeMap {
    fn hit_info(&self, _info: &HitResult, ray: &Ray) -> HitInfo {
        let comps: &[f32] = (&ray.d).into();
        let (max_idx, max_c) = comps.iter().enumerate()
            .reduce(|(prev_i, prev_c), (i, c)| if c.abs() > prev_c.abs() {(i, c)} else {(prev_i, prev_c)})
            .unwrap();
        
        let d = ray.d.normalize();
        use std::cmp::Ordering;
        let (u, v, fact, face) = 
            match (max_idx, max_c.partial_cmp(&0.0).expect(&format!("wtf {}", max_c))) {
                (0, Ordering::Less) => (d.z, d.y, d.x, &self.neg_x),

                (0, Ordering::Greater) => (d.z, d.y, d.x, &self.pos_x),

                (1, Ordering::Less) => (d.x, d.z, d.y, &self.neg_y),

                (1, Ordering::Greater) => (d.x, d.z, d.y, &self.pos_y),

                (2, Ordering::Less) => (d.x, d.y, d.z, &self.neg_z),

                (2, Ordering::Greater) => (d.x, d.y, d.z, &self.pos_z),

                _ => { panic!("this should be impossible!!") },
        };

        HitInfo {
            emissive: sample_face(u, v, fact, face), //: vector![0.7,0.7,1.0] * atten + red_comp,
            pos: ray.d * f32::INFINITY,
            norm: -ray.d,
            dls: false,
            continue_info: None,
        }
    }
}

fn sample_face(u: f32, v: f32, fact: f32, facewscale: &FaceImagewUVScale) -> Vector3<f32> {
    let (_, us, vs) = *facewscale;
    let face = &facewscale.0;
    let (u, v) = (u * us / fact, v * vs / fact);
    let (u, v) = (0.5 * u + 0.5, 0.5 * v + 0.5);
    
    face.get_pixel(u, v)
}

impl Hitable for DistantCubeMap {
    fn intersect(&self, _ray: &Ray) -> Option<HitResult> { // always hits since distant and covers all
        Some(HitResult{l: f32::INFINITY.into(), intermed: None})
    }
    fn give_aabb(&self) -> Option<Aabb> { None }
}