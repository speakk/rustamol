use crate::commands::*;
use crate::systems::CurrentTurn;
use bevy::ecs::event::Events;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::ExclusiveSystem;
use bevy::prelude::*;

fn validate_command(current_turn: Entity, event: &TurnCommandEvent) -> bool {
    match event.team {
        Some(team) => current_turn == team,
        None => false,
    }
}

pub fn system_build() -> impl FnMut(&mut World) {
    let system = move |world: &mut World| {
        let mut event_reader = ManualEventReader::<TurnCommandEvent>::from_world(world);
        world.resource_scope(|world, events: Mut<Events<TurnCommandEvent>>| {
            for event in event_reader.iter(&events) {
                event.command.execute(world);
            }
        })
    };

    system
}

// pub fn turn_command(
//     //mut events: EventReader<TurnCommandEvent>,
//     mut world: &mut World,
//     //current_turn: Res<CurrentTurn>,
// ) {
//     world.resource_scope(|world, current_turn: CurrentTurn| {
//         let events = ManualEventReader::from(world);
//     });
//     for event in events.iter() {
//         if validate_command(current_turn.0.expect("No current turn?"), event) {
//             event.command.execute(world);
//             // match event.command {
//             //     TurnCommand::MoveEntity(move_entity) => {
//             //         move_entity_event.send(move_entity);
//             //     }
//             //     TurnCommand::EndTurn(end_turn) => {
//             //         end_turn_event.send(end_turn);
//             //     }
//             //     TurnCommand::Attack(attack) => {
//             //         attack_event.send(attack);
//             //     }
//             //     TurnCommand::MoveAndAttack(move_and_attack) => {
//             //         move_and_attack_event.send(move_and_attack);
//             //     }
//             // }
//         } else {
//             println!("Tried to execute command out of turn");
//         }
//     }
// }
