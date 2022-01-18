use crate::components::Layer;
use crate::components::MapCoordinates;
use crate::components::Position;
use crate::models::pointy_hex_to_pixel;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::components;

extern crate lazy_static;

#[derive(Bundle)]
pub struct Hex {
    pub position: Position,
    pub coordinates: MapCoordinates,
    pub layer: Layer,
    pub hex: components::Hex,

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
            hex: components::Hex(),
            position: Position { x: 0.0, y: 0.0 },
            coordinates: MapCoordinates { q: ev.q, r: ev.r },
            layer: Layer(4),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/hexagon.png"),
                transform: pointy_hex_to_pixel(ev.q, ev.r),
                ..Default::default()
            },
        });
    }
}
