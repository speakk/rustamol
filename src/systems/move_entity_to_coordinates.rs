use crate::components::MapCoordinates;
use crate::components::Origin;
use crate::models::pointy_hex_to_pixel;
use bevy::prelude::*;

pub fn move_entity_to_coordinates(
    mut query: Query<(&MapCoordinates, &mut Transform, Option<&Origin>), Changed<MapCoordinates>>,
) {
    for (coordinates, mut transform, origin) in query.iter_mut() {
        *transform = pointy_hex_to_pixel(coordinates.q, coordinates.r);
        if let Some(origin) = origin {
            transform.translation += origin.0;
        }
    }
}
