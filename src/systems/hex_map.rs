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
    //for (mut transform, mut sprite, _) in hexes.iter_mut() {}
    // for (mut transform, layer, _) in sprites.iter_mut() {
    //     // TODO: Fix magic number
    //     transform.translation.z = 10.0 + -transform.translation.y * 0.01;

    //     if let Some(layer) = layer {
    //         transform.translation.z += layer.0 as f32;
    //     }
    // }
}
