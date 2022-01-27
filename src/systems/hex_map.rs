use crate::components::Coordinates;
use crate::components::Hex;
use crate::components::Unit;
use bevy::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

pub type CoordinatesToHex = HashMap<Coordinates, Entity>;
pub type HexOccupants = HashMap<Coordinates, HashSet<Entity>>;

#[allow(clippy::type_complexity)]
pub fn hex_map(
    mut map: ResMut<CoordinatesToHex>,
    query: Query<(&Coordinates, Entity), Added<Hex>>,
) {
    for (coordinates, entity) in query.iter() {
        map.insert(
            Coordinates {
                q: coordinates.q,
                r: coordinates.r,
            },
            entity,
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_hex_occupants(
    mut hex_occupants: ResMut<HexOccupants>,
    // TODO: Change With<Unit> to something like With<MapOccupant> if you want to support other
    // types of items on map
    query: Query<(&Coordinates, Entity), (Changed<Coordinates>, With<Unit>)>,
) {
    for (coordinates, entity) in query.iter() {
        let hex = Coordinates {
            q: coordinates.q,
            r: coordinates.r,
        };
        println!("Occupant insert into {:?}", &hex);
        let occupants = hex_occupants.entry(hex).or_insert_with(HashSet::new);
        occupants.insert(entity);
    }
}
