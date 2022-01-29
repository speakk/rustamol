use crate::commands::MoveEntity;
use crate::commands::TurnCommand;
use crate::commands::TurnCommandEvent;
use crate::components::*;
//use crate::systems::CurrentTurn;
// use crate::models::path_finding;
// use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

#[allow(clippy::too_many_arguments)]
pub fn click_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    hex_occupants: Res<HexOccupants>,
    mut selected_query: Query<Entity, With<Selected>>,
    mut sprite_query: Query<&mut Sprite>,
    coordinates_query: Query<&Coordinates>,
    mut turn_commands: EventWriter<TurnCommandEvent>,
    player_controlled: Query<Entity, With<PlayerControlled>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let position = mouse_world_coordinates;
        let coordinates = map::pixel_to_pointy_hex(position.x, position.y);
        //let hex = coordinates_to_hex.get(&coordinates);
        let occupants = hex_occupants.get(&coordinates);

        if let Some(occupants) = occupants {
            for entity in selected_query.iter_mut() {
                commands.entity(entity).remove::<Selected>();
            }

            for occupant in occupants.iter() {
                commands.entity(*occupant).insert(Selected);
                if let Ok(mut sprite) = sprite_query.get_mut(*occupant) {
                    sprite.color.set_g(0.0);
                }
            }
        } else {
            for entity in selected_query.iter_mut() {
                turn_commands.send(TurnCommandEvent {
                    command: TurnCommand::MoveEntity(MoveEntity {
                        from: *coordinates_query.get(entity).unwrap(),
                        to: coordinates,
                    }),
                    team: player_controlled.get_single().ok(),
                });
            }
        }
    }
}
