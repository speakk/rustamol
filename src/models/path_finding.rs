use crate::components::Coordinates;
use crate::models::map::NEIGHBOR_DIRECTIONS;
use crate::systems::HexOccupants;
use pathfinding::prelude::bfs;

impl Coordinates {
    fn successors(&self, hex_occupants: &HexOccupants) -> Vec<Coordinates> {
        println!("Looking for successors");
        let Coordinates { q, r } = self;
        NEIGHBOR_DIRECTIONS
            .clone()
            .into_iter()
            .map(|dir| Coordinates {
                q: q + dir.q,
                r: r + dir.r,
            })
            .filter(|coord| !hex_occupants.contains_key(coord))
            .collect()
    }
}

pub fn get_path(
    from: Coordinates,
    to: Coordinates,
    hex_occupants: &HexOccupants,
) -> Option<Vec<Coordinates>> {
    bfs(&from, |p| p.successors(hex_occupants), |p| *p == to)
}
