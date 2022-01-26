use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::components::ZOrder;
use crate::models::pointy_hex_to_pixel;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::components;

//use extend::ext;
extern crate lazy_static;

#[derive(Bundle)]
pub struct Hex {
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub hex: components::Hex,
    pub origin: Origin,
    pub z_order: ZOrder,

    #[bundle]
    pub sprite: SpriteBundle,
}

pub struct SpawnHex {
    pub q: i32,
    pub r: i32,
}

impl Command for SpawnHex {
    fn write(self, world: &mut World) {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let asset = asset_server.load("sprites/hexagon.png");
        world.spawn().insert_bundle(Hex {
            hex: components::Hex,
            coordinates: Coordinates {
                q: self.q,
                r: self.r,
            },
            origin: Origin(Vec3::new(0.0, -6.0, 0.0)),
            layer: Layer(4),
            sprite: SpriteBundle {
                texture: asset,
                transform: Transform::from_translation(
                    pointy_hex_to_pixel(self.q, self.r).extend(0.),
                ),
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
            z_order: ZOrder,
        });
    }
}

pub fn spawn_hex(
    coordinates: &Coordinates,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn_bundle(Hex {
            hex: components::Hex,
            coordinates: *coordinates,
            origin: Origin(Vec3::new(0.0, -6.0, 0.0)),
            layer: Layer(4),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/hexagon.png"),
                transform: Transform::from_translation(
                    pointy_hex_to_pixel(coordinates.q, coordinates.r).extend(0.),
                ),
                ..Default::default()
            },
            color_fade: ColorFade(Color::WHITE),
            z_order: ZOrder,
        })
        .id()
}
