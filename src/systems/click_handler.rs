use crate::components::Selected;
use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

pub fn click_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    hex_occupants: Res<HexOccupants>,
    mut selected_query: Query<Entity, With<Selected>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let position = mouse_world_coordinates;
        let hex = map::pixel_to_pointy_hex(position.x, position.y);

        let occupants = hex_occupants.get(&hex);

        for entity in selected_query.iter_mut() {
            commands.entity(entity).remove::<Selected>();
        }

        if let Some(occupants) = occupants {
            for occupant in occupants.iter() {
                commands.entity(*occupant).insert(Selected);
                if let Ok(mut sprite) = sprite_query.get_mut(*occupant) {
                    sprite.color.set_g(0.0);
                }
            }
        }
    }
}
