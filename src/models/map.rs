use std::cmp;

#[derive(PartialEq, Debug)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

pub fn axial_distance(a: Hex, b: Hex) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

pub enum MapShape {
    Hexagonal,
    Square,
}

pub fn create_grid(radius: i32, shape: MapShape) -> Vec<Hex> {
    match shape {
        MapShape::Hexagonal => {
            let mut hexes: Vec<Hex> = vec![];

            for q in -radius..radius {
                let r1: i32 = cmp::max(-radius, -q - radius);
                let r2: i32 = cmp::min(radius, -q + radius);

                for r in r1..r2 {
                    hexes.push(Hex { q, r });
                }
            }

            return hexes;
        }
        _ => vec![],
    }
}
