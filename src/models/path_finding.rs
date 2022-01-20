use crate::components::Coordinates;
use crate::models::map::NEIGHBOR_DIRECTIONS;
use pathfinding::prelude::bfs;

impl Coordinates {
    fn successors(&self) -> Vec<Coordinates> {
        let Coordinates { q, r } = self;
        NEIGHBOR_DIRECTIONS
            .clone()
            .into_iter()
            .map(|dir| Coordinates {
                q: q + dir.q,
                r: r + dir.r,
            })
            .collect()
    }
}

pub fn get_path(from: Coordinates, to: Coordinates) -> Option<Vec<Coordinates>> {
    bfs(&from, |p| p.successors(), |p| *p == to)
}
