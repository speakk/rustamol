use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::models::ShaderMaterial;
use crate::models::ShaderMesh2dBundle;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Command;
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

impl Command for SpawnUnit {
    fn write(self, world: &mut World) {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let asset = asset_server.load("sprites/skelly.png");
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32., 48.)))));
        let mut materials = world.get_resource_mut::<Assets<ShaderMaterial>>().unwrap();
        let material = materials.add(ShaderMaterial::from(asset));
        world.spawn().insert_bundle(Unit {
            unit: components::Unit,
            // TODO: Make the origin coordinates normalized
            origin: Origin(Vec3::new(0.0, 23.0, 0.0)),
            coordinates: Coordinates {
                q: self.q,
                r: self.r,
            },
            layer: Layer(5),
            sprite: ShaderMesh2dBundle {
                mesh,
                transform: Transform::default()
                    .with_translation(Vec3::new(0., 0., 20.))
                    .with_scale(Vec3::splat(1.)),
                material,
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
        });
    }
}
