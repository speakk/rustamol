use crate::components::{Coordinates, Selected};
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;
use crate::models::path_finding;

pub type HilightedPath = Vec<Coordinates>;

pub fn find_path_hilight(
    selected_query: Query<&Coordinates, With<Selected>>,
    mouse_world_coordinates: Res<MouseWorldCoordinates>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    mut hilighted_path: ResMut<HilightedPath>,
    hex_occupants: Res<HexOccupants>,
) {
    let position = mouse_world_coordinates;
    let hex_coordinates = map::pixel_to_pointy_hex(position.x, position.y);

    if let Some(_) = coordinates_to_hex.get(&hex_coordinates) {
        let entity_coordinates = selected_query.get_single();
        if let Ok(entity_coordinates) = entity_coordinates {
            //for entity_coordinates in selected_query.iter_mut() {
            let path = path_finding::get_path(
                *entity_coordinates,
                hex_coordinates,
                hex_occupants.into_inner(),
            );

            if let Some(path) = path {
                *hilighted_path = path;
            }
        }
    }
}

pub fn path_hilight(
    hilighted_path: ResMut<HilightedPath>,
    mut query: Query<&mut Sprite>,
    coordinates_to_hex: Res<CoordinatesToHex>,
) {
    for coordinate in hilighted_path.iter() {
        let entity = coordinates_to_hex.get(coordinate);
        if let Some(entity) = entity {
            if let Ok(mut sprite) = query.get_mut(*entity) {
                sprite.color.set_b(0.3);
                sprite.color.set_g(0.6);
            }
        }
    }
}
