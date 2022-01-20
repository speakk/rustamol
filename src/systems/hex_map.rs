use crate::components::Hex;
use crate::components::MapCoordinates;
use bevy::prelude::*;

use crate::models::map;

use std::collections::HashMap;

pub type CoordinatesToHex = HashMap<map::Hex, Entity>;

pub fn hex_map(
    mut map: ResMut<CoordinatesToHex>,
    query: Query<(&MapCoordinates, Entity), Added<Hex>>,
) {
    for (coordinates, entity) in query.iter() {
        map.insert(
            map::Hex {
                q: coordinates.q,
                r: coordinates.r,
            },
            entity,
        );
    }
}
