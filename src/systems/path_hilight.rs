use crate::components::{Coordinates, Selected};
use crate::systems::CoordinatesToHex;
use crate::systems::HexOccupants;
use crate::systems::MouseWorldCoordinates;
use bevy::prelude::*;

use crate::models::map;
use crate::models::path_finding;

pub type HilightedPath = Option<Vec<Coordinates>>;

pub type LastHoveredCoordinates = Option<Coordinates>;

pub fn last_hovered_coordinates(
    mut last_hovered_coordinates: ResMut<LastHoveredCoordinates>,
    mouse_position: Res<MouseWorldCoordinates>,
    coordinates_to_hex: Res<CoordinatesToHex>,
) {
    let hex_coordinates = map::pixel_to_pointy_hex(mouse_position.x, mouse_position.y);
    let hex = coordinates_to_hex.0.get(&hex_coordinates);

    if hex.is_some() {
        *last_hovered_coordinates = Some(hex_coordinates);
    } else {
        *last_hovered_coordinates = None;
    }
}

pub fn find_path_hilight(
    selected_query: Query<&Coordinates, With<Selected>>,
    mut hilighted_path: ResMut<HilightedPath>,
    hex_occupants: Res<HexOccupants>,
    coordinates_to_hex: Res<CoordinatesToHex>,
    last_hovered_coordinates: Res<LastHoveredCoordinates>,
) {
    if last_hovered_coordinates.is_changed() {
        if let Some(coordinates) = last_hovered_coordinates.into_inner() {
            let entity_coordinates = selected_query.get_single();
            if let Ok(entity_coordinates) = entity_coordinates {
                let path = path_finding::get_path(
                    *entity_coordinates,
                    *coordinates,
                    coordinates_to_hex.into_inner(),
                    hex_occupants.into_inner(),
                );

                if let Some(path) = path {
                    *hilighted_path = Some(path);
                } else {
                    //*hilighted_path = None;
                }
            }
        }
    }
}

pub fn path_hilight(
    hilighted_path: ResMut<HilightedPath>,
    mut query: Query<&mut Sprite>,
    coordinates_to_hex: Res<CoordinatesToHex>,
) {
    if let Some(hilighted_path) = hilighted_path.into_inner() {
        for coordinate in hilighted_path.iter() {
            let entity = coordinates_to_hex.0.get(coordinate);
            if let Some(entity) = entity {
                if let Ok(mut sprite) = query.get_mut(*entity) {
                    sprite.color.set_b(0.3);
                    sprite.color.set_g(0.6);
                }
            }
        }
    }
}
