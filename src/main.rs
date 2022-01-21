use crate::components::MainCamera;
use crate::components::*;
use crate::models::ShaderMaterial;
use crate::models::ShaderMaterialPlugin;
use crate::models::ShaderMesh2dBundle;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

#[macro_use]

mod bundles;
mod components;
mod models;
mod systems;

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut spawn_hex: EventWriter<bundles::SpawnHex>,
    mut spawn_unit: EventWriter<bundles::SpawnUnit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
    asset_server: Res<AssetServer>,
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
    spawn_unit.send(bundles::SpawnUnit { q: 0, r: -4 });

    let mut material = ShaderMaterial::from(asset_server.load("sprites/skelly.png"));
    material.outline = false;

    commands
        .spawn_bundle(ShaderMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32., 48.))))),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 0., 20.))
                .with_scale(Vec3::splat(1.)),
            material: materials.add(material),
            ..Default::default()
        })
        .insert(Unit)
        .insert(Layer(20))
        .insert(Coordinates { q: 0, r: 0 })
        .insert(Origin(Vec3::new(0.0, 23.0, 0.0)));

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
        .add_plugin(ShaderMaterialPlugin)
        .init_resource::<systems::hex_map::CoordinatesToHex>()
        .init_resource::<systems::hex_map::HexOccupants>()
        .init_resource::<systems::path_hilight::HilightedPath>()
        .init_resource::<systems::mouse_world_coordinates::MouseWorldCoordinates>()
        .add_event::<bundles::SpawnHex>()
        .add_event::<bundles::SpawnUnit>()
        .add_system(systems::click_handler)
        .add_system(systems::move_entity_to_coordinates)
        .add_system(systems::mouse_world_coordinates)
        .add_system(bundles::create_unit_system)
        .add_system(bundles::create_hex_system)
        .add_startup_system(setup)
        .add_system(systems::z_order)
        .add_system(systems::hex_map)
        .add_system(systems::selected)
        .add_system(systems::find_path_hilight)
        .add_system(systems::path_hilight)
        .add_system(systems::handle_hex_occupants)
        .add_system(systems::hex_hilight)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(systems::color_fade),
        )
        .run();
}
