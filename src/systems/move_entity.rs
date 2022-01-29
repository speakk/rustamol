use crate::commands::*;
use crate::components::Path;
use crate::components::TimedPath;
use crate::models::path_finding;
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use bevy::prelude::*;

pub fn move_entity(
    mut events: EventReader<MoveEntity>,
    mut commands: Commands,
    hex_occupants: Res<HexOccupants>,
    coordinates_to_hex: Res<CoordinatesToHex>,
) {
    for event in events.iter() {
        let occupants = hex_occupants.0.get(&event.from);

        if let Some(occupants) = occupants {
            for entity in occupants.iter() {
                let path = path_finding::get_path(
                    event.from,
                    event.to,
                    &*coordinates_to_hex,
                    &*hex_occupants,
                );
                if let Some(path) = path {
                    commands
                        .entity(*entity)
                        .insert_bundle(TimedPath::new(Path(path), None));
                }
            }
        }
    }
}
