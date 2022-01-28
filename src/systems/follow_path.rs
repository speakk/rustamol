use crate::components::*;
use bevy::prelude::*;

pub fn follow_path(
    mut query: Query<(
        &mut Coordinates,
        &Path,
        &mut PathTimer,
        &mut PathCurrentIndex,
    )>,
    time: Res<Time>,
) {
    for (mut coordinates, path, mut path_timer, mut path_current_index) in query.iter_mut() {
        if path_timer.0.tick(time.delta()).just_finished() {
            path_current_index.0 += 1;
            let new_coords = path.0.get(path_current_index.0);
            if let Some(new_coords) = new_coords {
                *coordinates = *new_coords;
            } else {
                // Send path finished event
            }
        }
    }
}
