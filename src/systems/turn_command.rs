use crate::commands::*;
use crate::components::Path;
use crate::components::TimedPath;
use crate::models::path_finding;
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use bevy::prelude::*;
//use core::any::Any;

// fn validate_command(current_turn: Entity, command: TurnCommand) -> bool {
//     match command.team {
//         Some(team) => current_turn == team,
//         None => false,
//     }
// }

// MOVE ENTITY
pub fn turn_command(
    mut events: EventReader<TurnCommand<MoveEntity>>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    hex_occupants: Res<HexOccupants>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let MoveEntity { from, to } = *event.command;
        println!("{:?}", event.command.from);
        let occupants = hex_occupants.get(&from);

        if let Some(occupants) = occupants {
            for entity in occupants.iter() {
                let path = path_finding::get_path(from, to, &*coordinates_to_hex, &*hex_occupants);
                if let Some(path) = path {
                    commands
                        .entity(*entity)
                        .insert_bundle(TimedPath::new(Path(path), None));
                }
            }
        }
    }
}
