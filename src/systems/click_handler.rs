use crate::commands::Attack;
use crate::commands::MoveEntity;
use crate::commands::TurnCommand;
use crate::commands::TurnCommandEvent;
use crate::components::*;
use crate::systems::CurrentTurn;
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
    current_turn: Res<CurrentTurn>,
    is_in_team: Query<&IsInTeam>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let position = mouse_world_coordinates;
        let coordinates = map::pixel_to_pointy_hex(position.x, position.y);
        let target_occupants = hex_occupants.0.get(&coordinates);

        if let Some(target_occupants) = target_occupants.filter(|list| !list.is_empty()) {
            if selected_query.iter().count() > 0 {
                let enemies_in_target: Vec<&Entity> = target_occupants
                    .iter()
                    .filter(|occupant| {
                        is_in_team
                            .get(**occupant)
                            .expect("Occupant was not in any team")
                            .team
                            != current_turn.0.unwrap()
                    })
                    .collect();

                if enemies_in_target.is_empty() {
                    for entity in selected_query.iter_mut() {
                        commands.entity(entity).remove::<Selected>();
                    }

                    for occupant in target_occupants.iter() {
                        commands.entity(*occupant).insert(Selected);
                        if let Ok(mut sprite) = sprite_query.get_mut(*occupant) {
                            sprite.color.set_g(0.0);
                        }
                    }
                } else {
                    for entity in selected_query.iter_mut() {
                        turn_commands.send(TurnCommandEvent {
                            command: TurnCommand::Attack(Attack {
                                from: *coordinates_query.get(entity).unwrap(),
                                to: coordinates,
                            }),
                            team: player_controlled.get_single().ok(),
                        });
                    }
                }
            } else {
                for entity in selected_query.iter_mut() {
                    commands.entity(entity).remove::<Selected>();
                }

                for occupant in target_occupants.iter() {
                    commands.entity(*occupant).insert(Selected);
                    if let Ok(mut sprite) = sprite_query.get_mut(*occupant) {
                        sprite.color.set_g(0.0);
                    }
                }
            }
        } else {
            if selected_query.iter().into_iter().count() > 0 {
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
}
