use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::components::Position;
use crate::models::pointy_hex_to_pixel;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::components;

extern crate lazy_static;

#[derive(Bundle)]
pub struct Unit {
    pub position: Position,
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub unit: components::Unit,
    pub origin: Origin,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct SpawnUnit {
    pub q: i32,
    pub r: i32,
}

pub fn create_unit_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<SpawnUnit>,
) {
    for ev in spawn_event.iter() {
        commands.spawn_bundle(Unit {
            unit: components::Unit,
            // TODO: Make the origin coordinates normalized
            origin: Origin(Vec3::new(0.0, 23.0, 0.0)),
            position: Position { x: 0.0, y: 0.0 },
            coordinates: Coordinates { q: ev.q, r: ev.r },
            layer: Layer(5),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/skelly.png"),
                transform: pointy_hex_to_pixel(ev.q, ev.r),
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
        });
    }
}
