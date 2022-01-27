use crate::components::AddHandle;
use crate::states::Images;
use bevy::prelude::*;

pub fn add_sprite(
    mut commands: Commands,
    query: Query<(Entity, &AddHandle), Changed<AddHandle>>,
    images: Res<Images>,
) {
    let images = images.into_inner();
    for (entity, add_handle) in query.iter() {
        commands.entity(entity).insert_bundle(SpriteBundle {
            texture: images.get(&add_handle.bundle_type).unwrap().clone(),
            ..Default::default()
        });
    }
}
