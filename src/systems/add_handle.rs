use crate::components::AddHandle;
use crate::models::ShaderMaterial;
use crate::models::ShaderMesh2dBundle;
use crate::states::Images;
use crate::states::Quads;
use bevy::prelude::*;

pub fn add_shadermesh_bundle(
    mut commands: Commands,
    query: Query<(Entity, &AddHandle), Changed<AddHandle>>,
    quads: Res<Quads>,
    images: Res<Images>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
) {
    let images = images.into_inner();
    let quads = quads.into_inner();
    for (entity, add_handle) in query.iter() {
        let material = materials.add(ShaderMaterial::from(
            images.get(&add_handle.bundle_type).unwrap().clone(),
        ));
        commands.entity(entity).insert_bundle(ShaderMesh2dBundle {
            mesh: quads.get(&add_handle.bundle_type).unwrap().clone(),
            material: material.clone(),
            ..Default::default()
        });
    }
}
