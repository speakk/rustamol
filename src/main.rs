use crate::models::ShaderMaterialPlugin;
use crate::models::ShaderMesh2dBundle;
//use crate::states;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

#[macro_use]

mod bundles;
mod components;
mod models;
mod states;
mod systems;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
}

fn main() {
    static AFTER: &str = "after_update";
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.15, 0.23)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShaderMaterialPlugin)
        .add_state(AppState::InGame)
        .init_resource::<systems::hex_map::CoordinatesToHex>()
        .init_resource::<systems::hex_map::HexOccupants>()
        .init_resource::<systems::path_hilight::LastHoveredCoordinates>()
        .init_resource::<systems::path_hilight::HilightedPath>()
        .init_resource::<systems::mouse_world_coordinates::MouseWorldCoordinates>()
        .add_stage_after(CoreStage::Update, AFTER, SystemStage::parallel())
        .add_event::<bundles::SpawnHex>()
        .add_event::<bundles::SpawnUnit>()
        .add_system_set(states::in_game::get_system_sets())
        .add_system(systems::mouse_world_coordinates)
        .add_system(bundles::create_hex_system)
        .add_startup_system(states::in_game::setup)
        .add_system(systems::z_order)
        .add_system(systems::hex_map)
        .add_system(systems::last_hovered_coordinates)
        .add_system(systems::hex_hilight)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(systems::color_fade),
        )
        .add_system_to_stage(AFTER, systems::selected_removal)
        .run();
}
