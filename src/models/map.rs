use crate::components::Coordinates;
use bevy::transform::components::Transform;
use std::cmp;

const HEX_SIZE: f32 = (32.0 / 2.0) * 1.1;
const HEX_LAYOUT_SIZE_X: f32 = HEX_SIZE;
const HEX_LAYOUT_SIZE_Y: f32 = HEX_SIZE * 0.82;

pub struct Matrix {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
}

lazy_static::lazy_static! {
    pub static ref POINTY_HEX_MATRIX: Matrix = Matrix {
        f0: 3_f32.sqrt(),
        f1: 3_f32.sqrt() / 2.0,
        f2: 0.0,
        f3: 3.0 / 2.0,
        b0: 3_f32.sqrt() / 3.0,
        b1: -1.0 / 3.0,
        b2: 0.0,
        b3: 2.0 / 3.0,
    };
}

lazy_static::lazy_static! {
    pub static ref NEIGHBOR_DIRECTIONS: Vec<Coordinates> = vec![
        Coordinates { q: 1, r: 0 },
        Coordinates { q: 1, r: -1 },
        Coordinates { q: 0, r: -1 },
        Coordinates { q: -1, r: 0 },
        Coordinates { q: -1, r: 1 },
        Coordinates { q: 0, r: 1 },
    ];
}

pub fn axial_distance(a: Coordinates, b: Coordinates) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

pub fn pointy_hex_to_pixel(q: i32, r: i32) -> Transform {
    let q = q as f32;
    let r = r as f32;
    let x = (POINTY_HEX_MATRIX.f0 * q + POINTY_HEX_MATRIX.f1 * r) * HEX_LAYOUT_SIZE_X;
    let y = (POINTY_HEX_MATRIX.f2 * q + POINTY_HEX_MATRIX.f3 * r) * HEX_LAYOUT_SIZE_Y;

    Transform::from_xyz(x, y, 0.0)
}

pub fn axial_round(fraq_q: f32, fraq_r: f32) -> Coordinates {
    let fraq_s = -fraq_q - fraq_r;
    let mut q = fraq_q.round();
    let mut r = fraq_r.round();
    let s = fraq_s.round();

    let q_diff = (q - fraq_q).abs();
    let r_diff = (r - fraq_r).abs();
    let s_diff = (s - fraq_s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    }

    return Coordinates {
        q: q as i32,
        r: r as i32,
    };
}

pub fn pixel_to_pointy_hex(x: f32, y: f32) -> Coordinates {
    let q = (3.0_f32.sqrt() / 3.0 * x - 1.0 / 3.0 * y) / HEX_LAYOUT_SIZE_X;
    let r = (2.0 / 3.0 * y) / HEX_LAYOUT_SIZE_Y;
    return axial_round(q, r);
}

// pub fn pixel_to_pointy_hex(x: f32, y: f32, origin_x: f32, origin_y: f32) -> Hex {
//     let final_x: f32 = (x - origin_x) / HEX_LAYOUT_SIZE_X;
//     let final_y: f32 = (y - origin_y) / HEX_LAYOUT_SIZE_Y;
//     let q: i32 = (POINTY_HEX_MATRIX.b0 * final_x + POINTY_HEX_MATRIX.b1 * final_y).round() as i32;
//     let r: i32 = (POINTY_HEX_MATRIX.b2 * final_x + POINTY_HEX_MATRIX.b3 * final_y).round() as i32;
//
//     Hex { q, r }
// }

pub enum MapShape {
    Hexagonal,
    Square,
}

pub fn create_grid(radius: i32, shape: MapShape) -> Vec<Coordinates> {
    match shape {
        MapShape::Hexagonal => {
            let mut hexes: Vec<Coordinates> = vec![];

            for q in -radius..radius {
                let r1: i32 = cmp::max(-radius, -q - radius);
                let r2: i32 = cmp::min(radius, -q + radius);

                for r in r1..r2 {
                    hexes.push(Coordinates { q, r });
                }
            }

            return hexes;
        }
        _ => vec![],
    }
}
