use nalgebra::{Vector3, Vector2};
use crate::elements::Decomposable;
use crate::elements::Element;
use super::*;
use crate::material::*;

// so it begins .....

pub struct Mesh {
    pub poses: Vec<Vector3<f32>>,
    pub norms: Vec<Vector3<f32>>,

    pub indices: Vec<[usize; 3]>, // each one represents a single triangle
    pub rgb_info: RgbInfo,
    pub norm_info: Option<NormInfo>,
    pub tangents: Option<Vec<Vector3<f32>>>,
    pub metal_rough: PbrMetalRoughInfo,
    // all of the above likely need to be double wrapped by Vec instead of single
    // due to all above properties existing for any primitive under the mesh

    // following indexed by primitive index
    pub textures: Vec<UVRgb32FImage>,
    pub normal_maps: Vec<UVRgb32FImage>,
    pub metal_rough_maps: Vec<UVRgb32FImage>,
}

pub struct PbrMetalRoughInfo {
    pub metal: f32,
    pub rough: f32,
    pub coords: Option<Vec<Vector2<f32>>>,
}

pub struct RgbInfo {
    pub factor: Vector3<f32>,
    pub coords: Option<Vec<Vector2<f32>>>,
}

pub struct NormInfo {
    pub scale: f32,
    pub coords: Vec<Vector2<f32>>,
}

impl Decomposable for Mesh {
    // the lifetime bound on this function was a solution that required my soul to find
    // allows me to create box of elements with a reference to the Mesh
    // that can exist for as long the Mesh does, skipping any useless Rc/Arc and crap
    fn decompose_to_elems<'e, 's>(&'s self) -> Box<dyn Iterator<Item = Element<'e>> + 's> 
    where
        's : 'e,
    {
        Box::new((0..self.indices.len()).map(
                |inner_idx| {
                    Box::new(MeshTriangle {
                        verts: VertexFromMesh {
                            index: (0, inner_idx),
                            mesh: self,
                        },
                        norm: NormFromMesh::from_mesh_and_inner_idx(self, (0, inner_idx)),

                        // below needs to be updated when textures come!
                        diverts_ray: DivertsRayFromMesh{
                            index: (0, inner_idx),
                            mesh: self,
                        },
                        rgb: RgbFromMesh{
                            index: (0, inner_idx),
                            mesh: self,
                        },
                    })} as Element<'e>))
    }
}