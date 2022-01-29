use crate::components::*;
use crate::systems::PlaceEntity;
use bevy::prelude::*;

pub fn follow_path(
    mut query: Query<(Entity, &Path, &mut PathTimer, &mut PathCurrentIndex)>,
    time: Res<Time>,
    mut place_entity: EventWriter<PlaceEntity>,
) {
    for (entity, path, mut path_timer, mut path_current_index) in query.iter_mut() {
        if path_timer.0.tick(time.delta()).just_finished() {
            path_current_index.0 += 1;
            let new_coords = path.0.get(path_current_index.0);
            if let Some(new_coords) = new_coords {
                //*coordinates = *new_coords;
                place_entity.send(PlaceEntity {
                    entity,
                    coordinates: *new_coords,
                });
            } else {
                // Send path finished event
            }
        }
    }
}
