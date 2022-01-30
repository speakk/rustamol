use crate::components::Coordinates;
use crate::components::Hex;
use crate::components::Unit;
use crate::systems::CoordinatesChanged;
use bevy::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct CoordinatesToHex(pub HashMap<Coordinates, Entity>);
#[derive(Default)]
pub struct HexOccupants(pub HashMap<Coordinates, HashSet<Entity>>);

pub struct DespawnHex(pub Entity);

pub fn hex_map(
    mut map: ResMut<CoordinatesToHex>,
    query: Query<(&Coordinates, Entity), Added<Hex>>,
) {
    for (coordinates, entity) in query.iter() {
        map.0.insert(
            Coordinates {
                q: coordinates.q,
                r: coordinates.r,
            },
            entity,
        );
    }
}

pub fn despawn_hex(
    mut map: ResMut<CoordinatesToHex>,
    mut events: EventReader<DespawnHex>,
    coordinates_query: Query<&Coordinates>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let coordinates = coordinates_query
            .get(event.0)
            .expect("No coordinates on hex on entity removal");
        let hex = map
            .0
            .get(coordinates)
            .expect("Tried to remove hex from where there was none");
        if hex == &event.0 {
            map.0.remove(coordinates);
        }
        commands.entity(event.0).despawn();
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_hex_occupants(
    mut hex_occupants: ResMut<HexOccupants>,
    // TODO: Change With<Unit> to something like With<MapOccupant> if you want to support other
    // types of items on map
    //query: Query<(&Coordinates, Entity), (Changed<Coordinates>, With<Unit>)>,
    mut coordinates_changed: EventReader<CoordinatesChanged>,
    units: Query<&Unit>,
) {
    for event in coordinates_changed.iter() {
        if units.get(event.entity).is_ok() {
            if let Some(from) = event.from {
                let occupants = hex_occupants.0.entry(from).or_insert_with(HashSet::new);
                let hmm = occupants.remove(&event.entity);
                println!("Removed! {}", hmm);
            }
            let occupants = hex_occupants.0.entry(event.to).or_insert_with(HashSet::new);
            occupants.insert(event.entity);
        }
    }
}
