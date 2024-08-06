use crate::ray::Ray;
use std::iter::zip;

pub struct Aabb {
    pub bounds: [PlaneBounds; 3],
}

pub struct PlaneBounds {
    pub low: f32, high: f32,
}

impl Aabb {
    pub fn get_entry_exit(&self, ray: &Ray) -> Option<((usize, f32), (usize, f32))> { // return axis, entry exit t for ray
        // adapted from https://gamedev.stackexchange.com/questions/18436/most-efficient-aabb-vs-ray-collision-algorithms/18459#18459

        let all_entry_exit: Vec<(usize, (f32, f32))> = zip(self.bounds.iter(), zip(ray.d.iter(), ray.o.iter()))
            .enumerate()
            .map(|(a, (b, (d, o)))| {
                let d = if d.abs() < crate::EPS { 
                    if *d < 0.0 { -crate::EPS } else { crate:: EPS}
                } else { *d };
                let f = 1.0 / d;
                (a, ((b.low - o) * f, (b.high - o) * f))
            }).collect();

        let min_axis = all_entry_exit.iter()
            .map(|(a, (l, h))| (a, l.min(*h)) )
            .reduce(|(ap, vp), (a, v)| {
                if vp < v {
                    (a, v)
                } else {
                    (ap, vp)
                }
            }).expect("no min?");
        let max_axis = all_entry_exit.iter()
            .map(|(a, (l, h))| (a, l.max(*h)) )
            .reduce(|(ap, vp), (a, v)| {
                if vp > v {
                    (a, v)
                } else {
                    (ap, vp)
                }
            }).expect("no max?");

        let (min_a, min_d) = min_axis;
        let (max_a, max_d) = max_axis;

        if max_d < 0.0 || min_d > max_d {
            None
        } else {
            Some(((*min_a, min_d), (*max_a, max_d)))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::{Vector3, vector};

    #[test]
    fn test_straight_axes() {
        let aabb = Aabb {
            bounds: [
                PlaneBounds {low: -1.0, high: 1.0},
                PlaneBounds {low: -1.0, high: 1.0},
                PlaneBounds {low: -1.0, high: 1.0},
            ],
        };

        for a in 1..3 {
            let d = {
                let mut v = Vector3::zeros();
                v[a] = 1.0;
                v
            };
            let ray = Ray{d, o: d * -3.0};

            assert_eq!(aabb.get_entry_exit(&ray), Some(((a, 2.0), (a, 4.0))))
        }  
    }

    #[test]
    fn test_xy_hits_both() {
        let aabb = Aabb {
            bounds: [
                PlaneBounds {low: -1.0, high: 1.0},
                PlaneBounds {low: -1.0, high: 1.0},
                PlaneBounds {low: -1.0, high: 1.0},
            ],
        };

        let ray = Ray{d: vector![2.1, 1.0, 0.0], o: vector![-2.0, 0.0, 0.0]};

        assert_eq!(aabb.get_entry_exit(&ray), Some(((0, 1.0/2.1), (1, 1.0))))  
    }
}