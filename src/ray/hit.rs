use super::Ray;
use nalgebra::Vector3;
use std::cmp::Ordering;

pub struct HitResult<I> {
    pub l: RayLen, // ray length: ray.d * l + ray.o will give you intersection point 
    pub intermed: I, // data to be plugged into hit_info should the result be required, can be () if not needed
}

pub struct HitInfo {
    pub rgb: Vector3<f32>,
}

pub trait Hitable { // use I to determine if should select this object
    type Interm;

    fn intersect(&self, ray: &Ray) -> Option<HitResult<Self::Interm>>;
}

pub trait HasHitInfo : Hitable {
    fn hit_info(&self, info: &HitResult<Self::Interm>) -> HitInfo;
}

#[derive(Clone)]
pub struct RayLen(f32);

impl From<f32> for RayLen {
    fn from(f: f32) -> RayLen {RayLen(f)}
}
impl From<RayLen> for f32 {
    fn from(l: RayLen) -> f32 {l.0}
}

// comparison based on ray distance l
// treat NAN same as inf so we can do total ordering on l as an f32
impl Ord for RayLen {
    fn cmp(&self, other: &Self) -> Ordering {
        // let me = self.l; 
        // let them = other.l;
        let me_and_them = [self.0, other.0];
        let neither_nan = me_and_them.iter().all(|e| !e.is_nan());
        
        if neither_nan { // do this first! 99.9 percent of cases!
            self.0.partial_cmp(&other.0).expect("But neither are nan!")
        } else {
            let both_nan = me_and_them.iter().all(|e| e.is_nan());
            let one_inf = me_and_them.iter().any(|e| e.is_infinite());
            let one_nan = me_and_them.iter().any(|e| e.is_nan());
            let inf_nan = one_inf && one_nan;

            if both_nan || inf_nan {
                Ordering::Equal
            } else {
                if self.0.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}
impl PartialOrd for RayLen {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for RayLen {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}
impl Eq for RayLen {}


#[cfg(test)]
mod ray_len_test {
    use super::*;

    #[test]
    fn test_less_than() {
        let me = RayLen(-1.0);
        let them = RayLen(1.0);

        assert!(me < them);
    }
    #[test]
    fn test_greater_than() {
        let me = RayLen(1.0);
        let them = RayLen(-1.0);

        assert!(me > them);
    }
    #[test]
    fn test_eq() {
        let me = RayLen(1.34324);
        let them = RayLen(1.34324);

        assert!(me == them);
    }
    #[test]
    fn test_nan_eq() {
        let me = RayLen(f32::NAN);
        let them = RayLen(f32::NAN);

        assert!(me == them);
    }
    #[test]
    fn test_inf_eq() {
        let me = RayLen(f32::INFINITY);
        let them = RayLen(f32::INFINITY);

        assert!(me == them);
    }
    #[test]
    fn test_nan_inf_eq() {
        let me = RayLen(f32::NAN);
        let them = RayLen(f32::INFINITY);

        assert!(me == them);

        let me = RayLen(f32::INFINITY);
        let them = RayLen(f32::NAN);

        assert!(me == them);
    }
}