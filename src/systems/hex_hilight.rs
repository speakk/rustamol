use crate::systems::CoordinatesToHex;
use bevy::prelude::*;

use crate::models::map;

// TODO:
// Next up:
// Get a way to get a Hex based on q and r, aka store the entities in some sort of map

pub fn hex_hilight(
    windows: Res<Windows>,
    mut coordinates_to_hex: ResMut<CoordinatesToHex>,
    mut query: Query<&mut Transform>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        println!("{}", position);
        let coordinates = map::pixel_to_pointy_hex(position.x, position.y, 0.0, 0.0);
        //let entity = coordinatesToHex.get(
        match coordinates_to_hex.get(&coordinates) {
            Some(entity) => {
                //let &mut transform = query.get(*entity).unwrap();
                if let Ok(mut transform) = query.get_mut(*entity) {
                    transform.translation.y -= 1.0;
                }
                println!("Found one!")
            }
            _ => println!("No"),
        }
    }
    //for (mut transform, mut sprite, _) in hexes.iter_mut() {}
    // for (mut transform, layer, _) in sprites.iter_mut() {
    //     // TODO: Fix magic number
    //     transform.translation.z = 10.0 + -transform.translation.y * 0.01;

    //     if let Some(layer) = layer {
    //         transform.translation.z += layer.0 as f32;
    //     }
    // }
}
