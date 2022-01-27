use crate::systems::CoordinatesToHex;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

pub fn hex_hilight(
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    mut query: Query<&mut Sprite>,
) {
    let position = mouse_world_coordinates;
    let coordinates = map::pixel_to_pointy_hex(position.x, position.y);
    if let Some(entity) = coordinates_to_hex.get(&coordinates) {
        if let Ok(mut sprite) = query.get_mut(*entity) {
            sprite.color.set_r(1.0);
            sprite.color.set_g(0.6);
            sprite.color.set_b(0.6);
        }
    }
}
