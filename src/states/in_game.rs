use crate::bundles;
use crate::components::MainCamera;
use crate::models;
use crate::systems;
use crate::AppState;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut spawn_hex: EventWriter<bundles::SpawnHex>,
    mut spawn_unit: EventWriter<bundles::SpawnUnit>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    let hexes = models::map::create_grid(8, models::MapShape::Hexagonal);
    for hex in hexes {
        spawn_hex.send(bundles::SpawnHex { q: hex.q, r: hex.r });
    }

    spawn_unit.send(bundles::SpawnUnit { q: 0, r: -2 });
    spawn_unit.send(bundles::SpawnUnit { q: 2, r: 0 });
    spawn_unit.send(bundles::SpawnUnit { q: 2, r: 1 });
    spawn_unit.send(bundles::SpawnUnit { q: 2, r: 2 });
    spawn_unit.send(bundles::SpawnUnit { q: 3, r: 2 });
    spawn_unit.send(bundles::SpawnUnit { q: 0, r: -4 });

    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(500., 500.);
    window.update_scale_factor_from_backend(2.0);
}

pub fn add_state_systems(app: &mut bevy::prelude::App) -> &mut bevy::prelude::App {
    app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup));
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(systems::click_handler)
            .with_system(systems::move_entity_to_coordinates)
            .with_system(bundles::create_unit_system)
            .with_system(systems::selected)
            .with_system(systems::find_path_hilight)
            .with_system(systems::path_hilight)
            .with_system(systems::handle_hex_occupants),
    );

    app
}
