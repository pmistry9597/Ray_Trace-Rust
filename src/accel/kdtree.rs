use crate::elements::Renderable;
use super::{Aabb, PlaneBounds};
use crate::ray::{Ray, closest_ray_hit, ClosestRayHit};
use nalgebra::Vector3;

pub struct KdTree<'k> {
    aabb: Aabb,
    node: Node<'k>,
    unconditional: &'k Vec<(usize, Renderable<'k>)>,
}

pub enum Node<'n> {
    Branch { axis: usize, split: f32, low: Box<Node<'n>>, high: Box<Node<'n>> },
    Leaf (Vec<(usize, Renderable<'n>)>),
}

impl<'k> KdTree<'k> {
    pub fn build(elems_and_aabbs: &Vec<(usize, Renderable<'k>, Aabb)>, unconditional: &'k Vec<(usize, Renderable<'k>)>) -> Self {
        // test one split along x for now
        let count = elems_and_aabbs.len();
        let aabbs: Vec<&Aabb> = elems_and_aabbs.iter().map(|(_,_,aabb)| aabb).collect();
        // split by avg
        let split = (&aabbs).into_iter().map(|aabb| aabb.centroid()).sum::<Vector3<f32>>() / (count as f32);

        let aabb = {
            let min_axes: Vec<f32> = (0..3).map(
                |a| (&aabbs).into_iter().map(|aabb| aabb.bounds[a].low)
                    .reduce(|pl, l| pl.min(l))
                    .unwrap()
                )
                .collect();
            let max_axes: Vec<f32> = (0..3).map(
                |a| (&aabbs).into_iter().map(|aabb| aabb.bounds[a].high)
                    .reduce(|ph, h| ph.max(h))
                    .unwrap()
                )
                .collect();
            Aabb {
                bounds: [
                    PlaneBounds {low: min_axes[0], high: max_axes[0]},
                    PlaneBounds {low: min_axes[1], high: max_axes[1]},
                    PlaneBounds {low: min_axes[2], high: max_axes[2]},
                ]
            }
        };

        let (low, high): (Vec<(usize, Renderable)>, Vec<(usize, Renderable)>) = {
            let mut low: Vec<(usize, Renderable)> = vec![];
            let mut high: Vec<(usize, Renderable)> = vec![];

            elems_and_aabbs.iter().for_each(|(i, e, aabb)| {
                // this can handle case of element in both nodes
                if aabb.bounds[0].high > split.x {
                    high.push((*i, *e));
                }
                if aabb.bounds[0].low < split.x {
                    low.push((*i, *e));
                }
            });

            (low, high)
        };

        KdTree {
            aabb,
            unconditional,
            node: Node::Branch { axis: 0, split: split.x, low: Box::new(Node::Leaf(low)), high: Box::new(Node::Leaf(high))},
        }
    }

    pub fn closest_ray_hit(&self, ray: &Ray) -> ClosestRayHit {
        let enters_domain = self.aabb.get_entry_exit(ray);
        match enters_domain {
            None => (vec![], None),
            Some(((_, entry_t), (_, exit_t))) => self.stack_search(ray, entry_t, exit_t),
        }
    }
    
    fn stack_search(&self, ray: &Ray, entry_t: f32, exit_t: f32) -> ClosestRayHit {
        // adapted from https://dcgi.fel.cvut.cz/home/havran/ARTICLES/cgf2011.pdf 
        let mut stack: Vec<(&Node, f32, f32)> = vec![(&self.node, entry_t, exit_t)];
        
        use Node::*;
        while !stack.is_empty() {
            let (mut current_node, entry_t, mut exit_t) = stack.pop().unwrap();
            while let Branch {axis, split, low, high} = current_node {
                let a = *axis;
                let t = (split - ray.o[a]) / ray.d[a]; // apparently split is a point in the paper? lets see how it goes
                let (near, far) = if ray.o[a] < *split {(low, high)} else {(high, low)};
                if t >= exit_t || t < 0.0 {
                    current_node = near;
                } else if t <= entry_t {
                    current_node = far;
                } else {
                    stack.push((far, t, exit_t));
                    current_node = near;
                    exit_t = t;
                }
            }
            
            if let Leaf(elems) = current_node {
                let (hit_results, idxo) = closest_ray_hit(ray, elems.into_iter().map(|e| *e));
                if let Some(elem_idx) = idxo {
                    let hit_result = &hit_results[elem_idx].1.as_ref().unwrap();
                    if hit_result.l.0 <= exit_t { // handle case of if the primitive lies on the split plane and the intersection is beyond this node
                        return (hit_results, idxo);
                    }
                }
            }
        }

        closest_ray_hit(ray, self.unconditional.iter().map(|e| *e))
    }
}