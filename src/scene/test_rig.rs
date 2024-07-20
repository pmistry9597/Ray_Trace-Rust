use super::{Scene, Cam};
use nalgebra::vector;
use super::basic_shape::Coloring::*;
use super::basic_shape::Sphere;
use crate::material::{DivertRayMethod, CommonMaterial};
// use std::sync::Arc;

pub fn walled() -> Scene {
    let cam = Cam {
        d: vector![0.0, 0.0, -5.0],
        o: vector![0.0, 0.0, 0.0],
        up: vector![0.0, 1.0, 0.0].normalize(),
        screen_width: 10.0,
        screen_height: 5.0,
        lens_r: Some(0.1),
        // lens_r: None,
    };

    let wr_x = 15.0;
    let wall_r = 500.0;
    let wr_y = 10.0;
    let wr_z = -30.0;

    use DivertRayMethod::*;

    let walls: Vec<Sphere> = vec![
        Sphere{c: vector![wr_x + wall_r, 0.0, -10.0], r: wall_r, 
            // coloring: UsePos(Arc::new(
            //     |pos, sph| vector![(-pos[0] + sph.r + sph.c[0]).abs()/(2.0*sph.r), 0.1, 0.4])),
            coloring: Solid(vector![0.25, 0.25, 0.75]),
            mat: CommonMaterial{ divert_ray:  Diff, emissive: None, },
        },
        Sphere{c: vector![-wr_x - wall_r, 0.0, -10.0], r: wall_r, 
            coloring: Solid(vector![0.75, 0.25, 0.25]),
            mat: CommonMaterial{ divert_ray:  Diff, emissive: None, },
        },
        Sphere{c: vector![0.0, -wr_y - wall_r, -10.0], r: wall_r, 
            coloring: Solid(vector![0.75, 0.75, 0.75]),
            mat: CommonMaterial{ divert_ray:  Diff, emissive: None, },
        },
        Sphere{c: vector![0.0, 0.0, wr_z - wall_r], r: wall_r, 
            coloring: Solid(vector![0.75, 0.75, 0.75]),
            mat: CommonMaterial{ divert_ray: Diff, emissive: None, },
        },
    ];
    let elements = vec![
        Sphere{c: vector![1.0, 0.5, -20.0], r: 4.0, 
            // coloring: UsePos(Arc::new(
            //     |pos, sph| vector![(pos[2] - sph.c[2]).abs()/sph.r, 0.2, 0.8])),
            coloring: Solid(vector![0.6, 0.0, 0.8]),
            mat: CommonMaterial{ divert_ray: Diff, emissive: None, },
        },
        Sphere{c: vector![-3.0, 0.0, -6.0], r: 1.0, 
            // coloring: UsePos(Arc::new(
            //     |pos, sph| vector![0.8, (pos[0] + sph.r - sph.c[0]).abs()/(2.0*sph.r), 0.1])),
            coloring: Solid(vector![1.0, 1.0, 1.0]),
            mat: CommonMaterial{ divert_ray: Spec, emissive: None, },
        },
        Sphere{c: vector![1.0, -1.5, -6.0], r: 0.5, 
            coloring: Solid(vector![0.2, 1.0, 0.5]),
            mat: CommonMaterial{ divert_ray: DiffSpec(0.7), emissive: None, },
        },
        Sphere{c: vector![-10.0, -7.0, -20.0], r: 2.0, 
            coloring: Solid(vector![1.0, 1.0, 1.0]),
            mat: CommonMaterial{ divert_ray: Dielectric(1.0, 1.3), emissive: None, },
        },
        Sphere{c: vector![-2.0, 1.5, -6.0], r: 0.5, 
            coloring: Solid(vector![0.7, 0.7, 1.0]),
            mat: CommonMaterial{ divert_ray: Dielectric(1.0, 1.3), emissive: None, },
        },
        Sphere{c: vector![2.0, 1.5, -6.0], r: 0.5, 
            coloring: Solid(vector![1.0, 0.5, 0.7]),
            mat: CommonMaterial{ divert_ray: Dielectric(1.0, 1.3), emissive: None, },
        },
    ];
    let lights = vec![
        Sphere{c: vector![0.0, 10.0, -15.0], r: 4.0, coloring: Solid(vector![0.0,0.0,0.0]),
            mat: CommonMaterial{ divert_ray:  Diff, emissive: Some(vector![1.0, 1.0, 1.0] * 5.0)},
        },
        Sphere{c: vector![1.0, 1.0, -7.0], r: 0.4, coloring: Solid(vector![1.0, 1.0, 1.0]),
            mat: CommonMaterial{ divert_ray:  Spec, emissive: Some(vector![1.0, 1.0, 1.0] * 15.0)},
        },
    ];

    Scene {
        cam,
        objs: walls.into_iter().chain(elements.into_iter()).chain(lights.into_iter()).collect(),
    }
}