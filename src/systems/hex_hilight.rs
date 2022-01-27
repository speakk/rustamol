use crate::models::ShaderMaterial;
use crate::systems::CoordinatesToHex;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;

pub fn hex_hilight(
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    query: Query<&Handle<ShaderMaterial>>,
    mut assets: ResMut<Assets<ShaderMaterial>>,
) {
    let position = mouse_world_coordinates;
    let coordinates = map::pixel_to_pointy_hex(position.x, position.y);
    if let Some(entity) = coordinates_to_hex.get(&coordinates) {
        if let Ok(sprite) = query.get(*entity) {
            let mat = assets.get_mut(sprite);
            if let Some(mat) = mat {
                mat.color.set_r(1.0);
                mat.color.set_g(0.6);
                mat.color.set_b(0.6);
            }
        }
    }
}
