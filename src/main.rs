use bevy::prelude::*;

#[macro_use]

mod bundles;
mod components;
mod models;

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut spawn_hex: EventWriter<bundles::SpawnHex>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    spawn_hex.send(bundles::SpawnHex { q: 3, r: 4 });
    spawn_hex.send(bundles::SpawnHex { q: 0, r: 4 });
    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("sprites/hexagon.png"),
    //     ..Default::default()
    // });

    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(1.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<bundles::SpawnHex>()
        .add_system(bundles::create_hex_system)
        .add_startup_system(setup)
        .run();
}
