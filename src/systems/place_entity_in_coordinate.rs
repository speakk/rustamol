use crate::components::Coordinates;
use bevy::prelude::*;

pub struct PlaceEntity {
    pub entity: Entity,
    pub coordinates: Coordinates,
}

pub struct CoordinatesChanged {
    pub entity: Entity,
    pub from: Option<Coordinates>,
    pub to: Coordinates,
}

pub fn place_entity_in_coordinate(
    mut events: EventReader<PlaceEntity>,
    mut coordinates_changed: EventWriter<CoordinatesChanged>,
    mut query: Query<&mut Coordinates>,
) {
    for event in events.iter() {
        let mut current_coordinates = query
            .get_mut(event.entity)
            .expect("Entity didn't have coordinates");
        coordinates_changed.send(CoordinatesChanged {
            entity: event.entity,
            from: Some(*current_coordinates),
            to: event.coordinates,
        });

        *current_coordinates = event.coordinates;
    }
}

pub fn detect_coordinates_added(
    query: Query<(Entity, &Coordinates), Added<Coordinates>>,
    mut coordinates_changed: EventWriter<CoordinatesChanged>,
) {
    for (entity, coordinates) in query.iter() {
        coordinates_changed.send(CoordinatesChanged {
            entity,
            from: None,
            to: *coordinates,
        });
    }
}
