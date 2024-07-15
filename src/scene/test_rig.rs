use super::{Scene, Cam};
use nalgebra::vector;
use super::basic_shape::Coloring::*;
use super::basic_shape::Sphere;
use crate::material::{SpecDiff, CommonMaterial};
use std::sync::Arc;

pub fn walled() -> Scene {
    let cam = Cam {
        d: vector![0.0, 0.0, -5.0],
        o: vector![0.0, 0.0, 0.0],
        up: vector![0.0, 1.0, 0.0].normalize(),
        screen_width: 10.0,
        screen_height: 5.0,
    };

    let wr_x = 15.0;
    let wall_r = 500.0;
    let wr_y = 10.0;
    let wr_z = -30.0;

    use SpecDiff::*;

    let walls: Vec<Sphere> = vec![
        Sphere{c: vector![wr_x + wall_r, 0.0, -10.0], r: wall_r, 
            coloring: UsePos(Arc::new(
                |pos, sph| vector![(-pos[0] + sph.r + sph.c[0]).abs()/(2.0*sph.r), 0.1, 0.4])),
            mat: CommonMaterial{ spec_or_diff:  Diff, emissive: None, },
        },
        Sphere{c: vector![-wr_x - wall_r, 0.0, -10.0], r: wall_r, 
            coloring: Solid(vector![0.0, 0.4, 0.0]),
            mat: CommonMaterial{ spec_or_diff:  Diff, emissive: None, },
        },
        Sphere{c: vector![0.0, -wr_y - wall_r, -10.0], r: wall_r, 
            coloring: Solid(vector![0.1, 0.0, 0.4]),
            mat: CommonMaterial{ spec_or_diff:  Diff, emissive: None, },
        },
        Sphere{c: vector![0.0, 0.0, wr_z - wall_r], r: wall_r, 
            coloring: Solid(vector![0.4, 0.0, 0.2]),
            mat: CommonMaterial{ spec_or_diff: Diff, emissive: None, },
        },
    ];
    let elements = vec![
        Sphere{c: vector![1.0, 0.5, -20.0], r: 4.0, 
            // coloring: UsePos(Arc::new(
            //     |pos, sph| vector![(pos[2] - sph.c[2]).abs()/sph.r, 0.2, 0.8])),
            coloring: Solid(vector![0.6, 0.0, 0.8]),
            mat: CommonMaterial{ spec_or_diff: Diff, emissive: None, },
        },
        Sphere{c: vector![-3.0, -1.0, -6.0], r: 1.0, 
            // coloring: UsePos(Arc::new(
            //     |pos, sph| vector![0.8, (pos[0] + sph.r - sph.c[0]).abs()/(2.0*sph.r), 0.1])),
            coloring: Solid(vector![1.0, 1.0, 1.0]),
            mat: CommonMaterial{ spec_or_diff: Spec, emissive: None, },
        },
    ];
    let lights = vec![
        Sphere{c: vector![0.0, 4.0, -15.0], r: 1.0, coloring: Solid(vector![1.0,1.0,1.0]),
            mat: CommonMaterial{ spec_or_diff:  Diff, emissive: Some(vector![1.0, 1.0, 1.0])},
        },
        Sphere{c: vector![3.0, -2.0, -10.0], r: 1.0, coloring: Solid(vector![1.0,1.0,1.0]),
            mat: CommonMaterial{ spec_or_diff:  Diff, emissive: Some(vector![1.0, 1.0, 1.0])},
        },
    ];

    Scene {
        cam,
        objs: walls.into_iter().chain(elements.into_iter()).chain(lights.into_iter()).collect(),
    }
}