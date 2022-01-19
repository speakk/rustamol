use crate::components::MainCamera;
use bevy::prelude::*;

#[macro_use]

mod bundles;
mod components;
mod models;
mod systems;

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut spawn_hex: EventWriter<bundles::SpawnHex>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    // spawn_hex.send(bundles::SpawnHex { q: 3, r: 4 });
    // spawn_hex.send(bundles::SpawnHex { q: 0, r: 4 });
    let hexes = models::map::create_grid(8, models::MapShape::Hexagonal);
    for hex in hexes {
        spawn_hex.send(bundles::SpawnHex { q: hex.q, r: hex.r });
    }
    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("sprites/hexagon.png"),
    //     ..Default::default()
    // });

    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(2.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<systems::hex_map::CoordinatesToHex>()
        .init_resource::<systems::mouse_world_coordinates::MouseWorldCoordinates>()
        .add_event::<bundles::SpawnHex>()
        .add_system(systems::mouse_world_coordinates)
        .add_system(bundles::create_hex_system)
        .add_startup_system(setup)
        .add_system(systems::z_order)
        .add_system(systems::hex_map)
        .add_system(systems::hex_hilight)
        .run();
}
