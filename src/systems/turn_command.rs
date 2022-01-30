use crate::commands::*;
use crate::systems::CurrentTurn;
use bevy::prelude::*;

fn validate_command(current_turn: Entity, event: &TurnCommandEvent) -> bool {
    match event.team {
        Some(team) => current_turn == team,
        None => false,
    }
}

pub fn turn_command(
    mut events: EventReader<TurnCommandEvent>,
    current_turn: Res<CurrentTurn>,
    mut move_entity_event: EventWriter<MoveEntity>,
    mut end_turn_event: EventWriter<EndTurn>,
    mut attack_event: EventWriter<Attack>,
) {
    for event in events.iter() {
        if validate_command(current_turn.0.expect("No current turn?"), event) {
            match event.command {
                TurnCommand::MoveEntity(move_entity) => {
                    move_entity_event.send(move_entity);
                }
                TurnCommand::EndTurn(end_turn) => {
                    end_turn_event.send(end_turn);
                }
                TurnCommand::Attack(attack) => {
                    attack_event.send(attack);
                }
            }
        } else {
            println!("Tried to execute command out of turn");
        }
    }
}
