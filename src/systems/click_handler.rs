use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

pub fn click_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    hex_occupants: Res<HexOccupants>,
    mut sprite_query: Query<&mut Sprite>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let position = mouse_world_coordinates;
        let hex = map::pixel_to_pointy_hex(position.x, position.y);

        println!("Left click! {:?}", hex);

        let occupants = hex_occupants.get(&hex);

        println!("Occupants? {:?}", occupants);

        if let Some(occupants) = occupants {
            for occupant in occupants.iter() {
                println!("Found occupants, setting non-green maybe");
                if let Ok(mut sprite) = sprite_query.get_mut(*occupant) {
                    sprite.color.set_g(0.0);
                }
            }
        }
    }
}
