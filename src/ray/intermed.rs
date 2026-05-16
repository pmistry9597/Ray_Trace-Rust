use nalgebra::Vector3;

pub enum Intermed {
    Sphere{perfect_pos: Vector3<f32>},
    Triangle{baryc: TriangleBarycentric},
}

type TriangleBarycentric = (f32, f32);