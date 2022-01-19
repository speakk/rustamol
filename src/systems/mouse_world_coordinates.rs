use crate::components::MainCamera;
use bevy::prelude::*;

#[derive(Default)]
pub struct MouseWorldCoordinates {
    pub x: f32,
    pub y: f32,
}

// Snatched from bevy cheat book
pub fn mouse_world_coordinates(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    query: Query<&Transform, With<MainCamera>>,
    mut res: ResMut<MouseWorldCoordinates>,
) {
    // get the primary window
    let window = windows.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = query.single();

        // apply the camera transform
        let position_world = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        res.x = position_world.x;
        res.y = position_world.y;
    }
}
