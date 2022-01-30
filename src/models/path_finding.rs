use crate::components::Coordinates;
use crate::models::map::NEIGHBOR_DIRECTIONS;
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use pathfinding::prelude::bfs;

impl Coordinates {
    fn successors(
        &self,
        coordinates_to_hex: &CoordinatesToHex,
        hex_occupants: &HexOccupants,
    ) -> Vec<Coordinates> {
        //println!("Looking for successors");
        let Coordinates { q, r } = self;
        let nodes: Vec<Coordinates> = NEIGHBOR_DIRECTIONS
            .clone()
            .into_iter()
            .map(|dir| Coordinates {
                q: q + dir.q,
                r: r + dir.r,
            })
            .filter(|coord| coordinates_to_hex.0.contains_key(coord))
            .filter(|coord| {
                hex_occupants
                    .0
                    .get(coord)
                    .filter(|occupants| !occupants.is_empty())
                    .is_none()
            })
            .collect();

        nodes
    }
}

pub fn get_path(
    from: Coordinates,
    to: Coordinates,
    coordinates_to_hex: &CoordinatesToHex,
    hex_occupants: &HexOccupants,
) -> Option<Vec<Coordinates>> {
    bfs(
        &from,
        |p| p.successors(coordinates_to_hex, hex_occupants),
        |p| *p == to,
    )
}
