//use crate::commands;
use crate::commands::MoveEntity;
use crate::commands::TurnCommand;
use crate::components::*;
use crate::systems::CurrentTurn;
// use crate::models::path_finding;
// use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

pub fn click_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    hex_occupants: Res<HexOccupants>,
    //coordinates_to_hex: Res<CoordinatesToHex>,
    mut selected_query: Query<Entity, With<Selected>>,
    mut sprite_query: Query<&mut Sprite>,
    coordinates_query: Query<&Coordinates>,
    //mut move_entity: EventWriter<MoveEntity>,
    mut turn_commands: EventWriter<TurnCommand<MoveEntity>>,
    current_turn: Res<CurrentTurn>,
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
                turn_commands.send(TurnCommand {
                    command: Box::new(MoveEntity {
                        from: *coordinates_query.get(entity).unwrap(),
                        to: coordinates,
                    }),
                    team: current_turn.0,
                });
                // move_entity.send(MoveEntity {
                //     from: *coordinates_query.get(entity).unwrap(),
                //     to: coordinates,
                // });

                //commands.entity(entity).insert
                //let path = path_finding::get_path(
                //    *coordinates_query.get(entity).unwrap(),
                //    coordinates,
                //    &*coordinates_to_hex,
                //    &*hex_occupants,
                //);
                //if let Some(path) = path {
                //    commands
                //        .entity(entity)
                //        .insert_bundle(TimedPath::new(Path(path), None));
                //}
            }
        }
    }
}
