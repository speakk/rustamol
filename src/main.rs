use crate::components::MainCamera;
use bevy::core::FixedTimestep;
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
    let hexes = models::map::create_grid(8, models::MapShape::Hexagonal);
    for hex in hexes {
        spawn_hex.send(bundles::SpawnHex { q: hex.q, r: hex.r });
    }

    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(2.0);
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.15, 0.23)))
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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(systems::color_fade),
        )
        .run();
}
