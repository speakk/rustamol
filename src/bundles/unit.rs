use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::models::ShaderMaterial;
use crate::ShaderMesh2dBundle;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::components;

extern crate lazy_static;

#[derive(Bundle)]
pub struct Unit {
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub unit: components::Unit,
    pub origin: Origin,

    #[bundle]
    pub sprite: ShaderMesh2dBundle,
}

pub struct SpawnUnit {
    pub q: i32,
    pub r: i32,
}

pub fn create_unit_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<SpawnUnit>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for ev in spawn_event.iter() {
        commands.spawn_bundle(Unit {
            unit: components::Unit,
            // TODO: Make the origin coordinates normalized
            origin: Origin(Vec3::new(0.0, 23.0, 0.0)),
            coordinates: Coordinates { q: ev.q, r: ev.r },
            layer: Layer(5),
            sprite: ShaderMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32., 48.))))),
                transform: Transform::default()
                    .with_translation(Vec3::new(0., 0., 20.))
                    .with_scale(Vec3::splat(1.)),
                material: materials.add(ShaderMaterial::from(
                    asset_server.load("sprites/skelly.png"),
                )),
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
        });
    }
}
