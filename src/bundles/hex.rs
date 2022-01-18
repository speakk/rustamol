use crate::components::Layer;
use crate::components::MapCoordinates;
use crate::components::Position;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

extern crate lazy_static;

const HEX_SIZE: f32 = (32.0 / 2.0) * 1.1;

#[derive(Bundle)]
pub struct Hex {
    pub position: Position,
    pub coordinates: MapCoordinates,
    pub layer: Layer,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct SpawnHex {
    pub q: i32,
    pub r: i32,
}

pub struct Matrix {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    // b0: f32,
    // b1: f32,
    // b2: f32,
    // b3: f32,
}

// const POINTY_HEX_MATRIX: Matrix = Matrix {
//     f0: 3_f32.sqrt(),
//     f1: 3_f32.sqrt() / 2.0,
//     f2: 0.0,
//     f3: 3.0 / 2.0,
//     b0: 3_f32.sqrt() / 3.0,
//     b1: -1.0 / 3.0,
//     b2: 0.0,
//     b3: 2.0 / 3.0,
// };

lazy_static::lazy_static! {
    pub static ref POINTY_HEX_MATRIX: Matrix = Matrix {
        f0: 3_f32.sqrt(),
        f1: 3_f32.sqrt() / 2.0,
        f2: 0.0,
        f3: 3.0 / 2.0,
        // b0: 3_f32.sqrt() / 3.0,
        // b1: -1.0 / 3.0,
        // b2: 0.0,
        // b3: 2.0 / 3.0,
    };
}

pub fn pointy_hex_to_pixel(q: i32, r: i32) -> Transform {
    let q = q as f32;
    let r = r as f32;
    let x = (POINTY_HEX_MATRIX.f0 * q + POINTY_HEX_MATRIX.f1 * r) * HEX_SIZE as f32;
    let y = (POINTY_HEX_MATRIX.f2 * q + POINTY_HEX_MATRIX.f3 * r) * (HEX_SIZE as f32) * 0.82;

    Transform::from_xyz(x, y, 0.0)
}

pub fn create_hex_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_hex_event: EventReader<SpawnHex>,
) {
    for ev in spawn_hex_event.iter() {
        commands.spawn_bundle(Hex {
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
