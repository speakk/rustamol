use crate::commands::*;
use crate::components::Path;
use crate::components::TimedPath;
use crate::models::path_finding;
use crate::systems::CoordinatesToHex;
use crate::systems::CurrentTurn;
use crate::systems::HexOccupants;
use bevy::prelude::*;

fn validate_command(current_turn: Entity, event: &TurnCommandEvent) -> bool {
    match event.team {
        Some(team) => current_turn == team,
        None => false,
    }
}

// MOVE ENTITY
pub fn turn_command(
    mut events: EventReader<TurnCommandEvent>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    hex_occupants: Res<HexOccupants>,
    mut commands: Commands,
    current_turn: Res<CurrentTurn>,
) {
    for event in events.iter() {
        if validate_command(current_turn.0.expect("No current turn?"), event) {
            match event.command {
                TurnCommand::MoveEntity(MoveEntity { from, to }) => {
                    let occupants = hex_occupants.get(&from);

                    if let Some(occupants) = occupants {
                        for entity in occupants.iter() {
                            let path = path_finding::get_path(
                                from,
                                to,
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
        } else {
            println!("Tried to execute command out of turn");
        }
    }
}
