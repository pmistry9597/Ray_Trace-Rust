use super::Ray;
use nalgebra::Vector3;
use std::cmp::Ordering;

pub struct HitResult<I> {
    pub l: f32, // ray length: ray.d * l + ray.o will give you intersection point 
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

// comparison based on ray distance l
// treat NAN same as inf so we can do total ordering on l as an f32
impl<I> Ord for HitResult<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = self.l; 
        let them = other.l;
        let me_and_them = [me, them];
        let neither_nan = me_and_them.iter().all(|e| !e.is_nan());

        if neither_nan { // do this first! 99.9 percent of cases!
            me.partial_cmp(&them).expect("But neither are nan!")
        } else {
            let both_nan = me_and_them.iter().all(|e| e.is_nan());
            let one_inf = me_and_them.iter().any(|e| e.is_infinite());
            let one_nan = me_and_them.iter().any(|e| e.is_nan());
            let inf_nan = one_inf && one_nan;

            if both_nan || inf_nan {
                Ordering::Equal
            } else {
                if me.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}
impl<I> PartialOrd for HitResult<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<I> PartialEq for HitResult<I> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}
impl<I> Eq for HitResult<I> {}


#[cfg(test)]
mod hit_result_test {
    use super::*;

    #[test]
    fn test_less_than() {
        let me = HitResult{l: -1.0, intermed: ()};
        let them = HitResult{l: 1.0, intermed: ()};

        assert!(me < them);
    }
    #[test]
    fn test_greater_than() {
        let me = HitResult{l: 1.0, intermed: ()};
        let them = HitResult{l: -1.0, intermed: ()};

        assert!(me > them);
    }
    #[test]
    fn test_eq() {
        let me = HitResult{l: 1.34324, intermed: ()};
        let them = HitResult{l: 1.34324, intermed: ()};

        assert!(me == them);
    }
    #[test]
    fn test_nan_eq() {
        let me = HitResult{l: f32::NAN, intermed: ()};
        let them = HitResult{l: f32::NAN, intermed: ()};

        assert!(me == them);
    }
    #[test]
    fn test_inf_eq() {
        let me = HitResult{l: f32::INFINITY, intermed: ()};
        let them = HitResult{l: f32::INFINITY, intermed: ()};

        assert!(me == them);
    }
    #[test]
    fn test_nan_inf_eq() {
        let me = HitResult{l: f32::NAN, intermed: ()};
        let them = HitResult{l: f32::INFINITY, intermed: ()};

        assert!(me == them);

        let me = HitResult{l: f32::INFINITY, intermed: ()};
        let them = HitResult{l: f32::NAN, intermed: ()};

        assert!(me == them);
    }
}