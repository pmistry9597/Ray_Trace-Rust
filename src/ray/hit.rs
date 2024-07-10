use super::Ray;
use std::cmp::Ordering;

pub struct HitResult<I> {
    pub l: f32, // ray length: ray.d * l + ray.o will give you intersection point 
    pub intermed: I, // data to be plugged into hit_info should the result be required, can be () if not needed
}

pub struct HitInfo {
    pub rgb: [u8; 3],
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
        // self.height.cmp(&other.height)
        let me = self.l; 
        let them = other.l;
        
        let me_and_them = [me, them];

        let both_nan = me_and_them.iter().all(|e| e.is_nan());
        let one_inf = me_and_them.iter().any(|e| e.is_finite());
        let one_nan = me_and_them.iter().any(|e| e.is_nan());
        let inf_nan = one_inf && one_nan;

        if both_nan || inf_nan {
            Ordering::Equal
        } else if one_nan {
            if me.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            me.partial_cmp(&them).expect("Above cases should handle this issue!")
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
        self.l == other.l
    }
}
impl<I> Eq for HitResult<I> {}