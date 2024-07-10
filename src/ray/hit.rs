use super::Ray;

pub struct HitResult<I> {
    pub l: f32, // ray length: ray.d * l + ray.o will give you intersection point 
    pub intermed: I, // data to be plugged into hit_info should the result be required, can be () if not needed
}

pub struct HitInfo {
    rgb: [u8; 3],
}

pub trait Hitable<I> { // use I to determine if should select this object
    fn intersect(&self, ray: &Ray) -> Option<HitResult<I>>;
}

pub trait HasHitInfo<I> : Hitable<I> {
    fn hit_info(info: I) -> HitInfo;
}