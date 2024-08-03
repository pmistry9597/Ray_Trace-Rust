use nalgebra::Vector3;
use crate::elements::Decomposable;
use crate::elements::Element;
use super::*;

use crate::material::*;
use nalgebra::vector;

// so it begins .....

pub struct Mesh {
    pub poses: Vec<Vector3<f32>>,
    pub norms: Vec<Vector3<f32>>,

    pub indices: Vec<[usize; 3]>, // each one represents a single triangle
    // pub triangles: Vec<MeshTriangle<'a>>, // indexes into above
}

impl Decomposable for Mesh {
    fn decompose_to_elems<'e, 's>(&'s self) -> Box<dyn Iterator<Item = Element<'e>> + 's> 
    where
        's : 'e,
    {
        Box::new((0..self.indices.len()).map(
                |index| {
                    Box::new(MeshTriangle {
                        verts: VertexFromMesh {
                            index,
                            mesh: self,
                        },
                        norm: NormFromMesh {
                            index,
                            mesh: self,
                        },

                        // below needs to be updated when textures come!
                        mat: DiffuseSpecNoBaseMaterial{
                            divert_ray: DivertRayMethod::Spec,
                            emissive: Some(vector![0.1,0.1,0.1]),
                        },
                        rgb: vector![0.7,0.7,0.99],
                    })} as Element<'e>))
    }
}