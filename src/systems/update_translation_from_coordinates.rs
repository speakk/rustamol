use crate::components::Coordinates;
use crate::components::Origin;
use crate::models::pointy_hex_to_pixel;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_translation_from_coordinates(
    mut query: Query<
        (&Coordinates, &mut Transform, Option<&Origin>),
        Or<(Changed<Coordinates>, Added<Transform>)>,
    >,
) {
    for (coordinates, mut transform, origin) in query.iter_mut() {
        let target = pointy_hex_to_pixel(coordinates.q, coordinates.r);
        transform.translation = Vec3::new(target.x, target.y, transform.translation.z);

        if let Some(origin) = origin {
            transform.translation += origin.0;
        }
    }
}
