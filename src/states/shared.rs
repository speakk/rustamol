use crate::components::MainCamera;
use crate::systems;
use crate::systems::CoordinatesChanged;
use crate::systems::DespawnHex;
use crate::systems::PlaceEntity;
use crate::TIME_STEP;
use crate::{AFTER, BEFORE};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.15, 0.23)))
            .add_plugins(DefaultPlugins)
            .init_resource::<systems::hex_map::CoordinatesToHex>()
            .init_resource::<systems::hex_map::HexOccupants>()
            .init_resource::<systems::path_hilight::LastHoveredCoordinates>()
            .init_resource::<systems::path_hilight::HilightedPath>()
            .init_resource::<systems::mouse_world_coordinates::MouseWorldCoordinates>()
            .init_resource::<Quads>()
            .init_resource::<Images>()
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
            .add_event::<DespawnHex>()
            .add_system(systems::despawn_hex)
            .add_system(systems::follow_path)
            .add_system(systems::place_entity_in_coordinate)
            .add_system(systems::detect_coordinates_added)
            .add_system(systems::update_translation_from_coordinates)
            .add_system(systems::hex_hilight.after("input"))
            .add_system_set_to_stage(
                BEFORE,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(systems::color_fade),
            )
            .add_system_to_stage(AFTER, systems::selected_removal)
            .add_system(systems::add_sprite)
            .add_system(debug_system)
            .add_event::<CoordinatesChanged>()
            .add_event::<PlaceEntity>()
            .add_startup_system(setup)
            .add_startup_system(setup_handles);
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

pub static HEX_SPRITE_PATH: &str = "sprites/hexagon.png";
pub static SKELLY_SPRITE_PATH: &str = "sprites/skelly.png";
pub static STABBY_SPRITE_PATH: &str = "sprites/stabby.png";

#[derive(Eq, PartialEq, Hash)]
pub enum BundleType {
    Skelly,
    Hex,
    Stabby,
}

pub type Quads = HashMap<BundleType, Mesh2dHandle>;
pub type Images = HashMap<BundleType, Handle<Image>>;
//pub skelly: Mesh2dHandle,
//}

pub fn setup_handles(asset_server: Res<AssetServer>, images: ResMut<Images>) {
    let images = images.into_inner();
    let image = asset_server.load(HEX_SPRITE_PATH) as Handle<Image>;
    images.insert(BundleType::Hex, image);

    let image = asset_server.load(SKELLY_SPRITE_PATH) as Handle<Image>;
    images.insert(BundleType::Skelly, image);

    let image = asset_server.load(STABBY_SPRITE_PATH) as Handle<Image>;
    images.insert(BundleType::Stabby, image);
}

//pub fn debug_system(query: Query<&Mesh2dHandle, With<Unit>>) {
pub fn debug_system() {
    //println!("Images {}", res.into_inner().len());
    // for transform in query.iter() {
    //     println!("UNIT {:?}", transform.0);
    // }
}
