use crate::commands::EndTurn;
use crate::commands::TurnCommandEvent;
use crate::components::PlayerControlled;

use bevy::prelude::*;

pub fn keyboard_handler(
    keys: Res<Input<KeyCode>>,
    mut turn_commands: EventWriter<TurnCommandEvent>,
    player_controlled: Query<Entity, With<PlayerControlled>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // TODO: Add this back in
        // turn_commands.send(TurnCommandEvent {
        //     command: TurnCommand::EndTurn(EndTurn),
        //     team: player_controlled.get_single().ok(),
        // });
    }
}
