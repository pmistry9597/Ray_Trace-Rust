use nalgebra::Vector3;
use crate::elements::triangle::{Triangle, GimmeNorm};
use super::Mesh;
use std::ops::Index;

pub type MeshTriangle<'a> = Triangle<VertexFromMesh<'a>, NormFromMesh<'a>>;

#[derive(Clone)]
pub struct VertexFromMesh<'m> {
    pub index: usize,
    pub mesh: &'m Mesh,
}

impl Index<usize> for VertexFromMesh<'_> {
    type Output = Vector3<f32>;

    fn index(&self, vert_idx: usize) -> &Vector3<f32> {
        &self.mesh.poses[self.mesh.indices[self.index][vert_idx]]
    }
}

#[derive(Clone)]
pub struct NormFromMesh<'m> {
    pub index: usize,
    pub mesh: &'m Mesh,
}

impl GimmeNorm for NormFromMesh<'_> {
    fn get_norm(&self, _pos: &Vector3<f32>) -> Vector3<f32> {
        // temporary, get the avg of all norms
        // later may use enum to use barycentric weighting or normal mapping
        let cum: Vector3<f32> = self.mesh.indices[self.index].iter()
            .map(|i| self.mesh.norms[*i])
            .sum();
        cum / 3.0
    }
}
