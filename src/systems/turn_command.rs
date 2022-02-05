use crate::commands::*;
use crate::systems::CurrentTurn;
use bevy::ecs::event::Events;
use bevy::ecs::event::ManualEventReader;
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
            world.resource_scope(|world, current_turn: Mut<CurrentTurn>| {
                for event in event_reader.iter(&events) {
                    if validate_command(current_turn.0.expect("No current turn?"), event) {
                        event.command.execute(world);
                    }
                }
            });
        })
    };

    system
}
