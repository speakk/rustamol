use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::models::pointy_hex_to_pixel;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::components;

extern crate lazy_static;

#[derive(Bundle)]
pub struct Hex {
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub hex: components::Hex,
    pub origin: Origin,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct SpawnHex {
    pub q: i32,
    pub r: i32,
}

pub fn create_hex_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_hex_event: EventReader<SpawnHex>,
) {
    for ev in spawn_hex_event.iter() {
        commands.spawn_bundle(Hex {
            hex: components::Hex,
            coordinates: Coordinates { q: ev.q, r: ev.r },
            origin: Origin(Vec3::new(0.0, -6.0, 0.0)),
            layer: Layer(4),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/hexagon.png"),
                transform: Transform::from_translation(pointy_hex_to_pixel(ev.q, ev.r).extend(0.)),
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
        });
    }
}
