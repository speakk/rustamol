use crate::commands::*;
use crate::systems::CurrentTurn;
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
    current_turn: Res<CurrentTurn>,
    mut move_entity_event: EventWriter<MoveEntity>,
) {
    for event in events.iter() {
        if validate_command(current_turn.0.expect("No current turn?"), event) {
            match event.command {
                TurnCommand::MoveEntity(move_entity) => {
                    move_entity_event.send(move_entity);
                }
            }
        } else {
            println!("Tried to execute command out of turn");
        }
    }
}
