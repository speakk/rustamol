use crate::components::MainCamera;
use crate::models::ShaderMaterialPlugin;
use crate::systems;
use crate::TIME_STEP;
use crate::{AFTER, BEFORE};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.15, 0.23)))
            .add_plugins(DefaultPlugins)
            .add_plugin(ShaderMaterialPlugin)
            .init_resource::<systems::hex_map::CoordinatesToHex>()
            .init_resource::<systems::hex_map::HexOccupants>()
            .init_resource::<systems::path_hilight::LastHoveredCoordinates>()
            .init_resource::<systems::path_hilight::HilightedPath>()
            .init_resource::<systems::mouse_world_coordinates::MouseWorldCoordinates>()
            .add_stage_after(CoreStage::Update, AFTER, SystemStage::parallel())
            .add_stage_before(CoreStage::Update, BEFORE, SystemStage::parallel())
            .add_system_set(
                SystemSet::new()
                    .label("input")
                    .with_system(systems::mouse_world_coordinates)
                    .with_system(systems::last_hovered_coordinates),
            )
            .add_system(systems::z_order)
            .add_system(systems::hex_map)
            .add_system(systems::move_entity_to_coordinates)
            .add_system(systems::hex_hilight.after("input"))
            .add_system_set_to_stage(
                BEFORE,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(systems::color_fade),
            )
            .add_system_to_stage(AFTER, systems::selected_removal)
            .add_startup_system(setup);
    }
}

pub fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(500., 500.);
    window.update_scale_factor_from_backend(2.0);
}
