use serde::Deserialize;
use crate::elements::sphere::Sphere;
// use crate::elements::Element;
use crate::scene::Member;
use crate::elements::distant_cube_map;
use crate::elements::triangle;
use crate::elements::mesh::Mesh;
use super::pr;

#[derive(Deserialize, Debug)]
pub struct VecInto<T>(Vec<T>); // wrapper st if elements have into one type to another, easily convert this vec into vec of another

impl<A, B> From<VecInto<A>> for Vec<B> 
where
    B: From<A>
{
    fn from(val: VecInto<A>) -> Self {
        let VecInto(contents) = val;
        contents.into_iter().map(|t| t.into()).collect()
    }
}

#[derive(Deserialize, Debug)]
pub enum MemberTypes {
    Sphere(Sphere),
    DistantCubeMap(pr::DistantCubeMap),
    FreeTriangle(pr::FreeTriangle),
    
    MeshFromNode(pr::MeshFromNode),
}

impl From<MemberTypes> for Member<'_> {
    fn from(val: MemberTypes) -> Self {
        use MemberTypes::*;
        match val {
            Sphere(s) => Member::Elem(Box::new(s)),
            DistantCubeMap(prcs) => Member::Elem(
                Box::new(distant_cube_map::DistantCubeMap {
                    neg_z: prcs.neg_z.into(),
                    pos_z: prcs.pos_z.into(),
                    neg_x: prcs.neg_x.into(),
                    pos_x: prcs.pos_x.into(),
                    neg_y: prcs.neg_y.into(),
                    pos_y: prcs.pos_y.into(),
                })),
            FreeTriangle(t) => Member::Elem(
                Box::new(
                    triangle::FreeTriangle {
                        norm: t.norm.normalize().into(),
                        verts: t.verts,
                        rgb: t.rgb,
                        mat: t.mat,
                    },
            )),
            MeshFromNode(nfm) => {
                nfm.diagnostics();
                // Member::Elem(Box::new(pr::DummyElement{}))
                use nalgebra::vector;
                use crate::elements::mesh::*;
                // use std::cell::RefCell;
                let mesh = {
                    let mut mesh = Box::new(Mesh{
                        poses: vec![
                            vector![0.0, 0.0, -20.0],
                            vector![-5.0, 5.0, -20.0],
                            vector![5.0, 5.0, -20.0],
                            vector![0.0, 10.0, -20.0],
                        ],
                        norms: vec![
                            vector![0.0, 0.0, 1.0],
                            vector![0.0, 0.0, 1.0],
                            vector![0.0, 0.0, 1.0],
                            vector![0.0, 0.0, 1.0],
                        ],
                        indices: vec![
                            [0, 1, 2],
                            [3, 1, 2],
                        ],
                        // triangles: RefCell::new(vec![]),
                    });
                    mesh
                };
                Member::Grp(mesh)
            }
        }
    }
}