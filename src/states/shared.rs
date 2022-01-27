use crate::components::MainCamera;
//use crate::components::Unit;
use crate::models::ShaderMaterialPlugin;
use crate::systems;
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
            .add_plugin(ShaderMaterialPlugin)
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
            .add_system(systems::move_entity_to_coordinates)
            .add_system(systems::hex_hilight.after("input"))
            .add_system_set_to_stage(
                BEFORE,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(systems::color_fade),
            )
            .add_system_to_stage(AFTER, systems::selected_removal)
            .add_system(systems::add_shadermesh_bundle)
            .add_system(debug_system)
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

#[derive(Eq, PartialEq, Hash)]
pub enum BundleType {
    SKELLY,
    HEX,
}

pub type Quads = HashMap<BundleType, Mesh2dHandle>;
pub type Images = HashMap<BundleType, Handle<Image>>;
//pub skelly: Mesh2dHandle,
//}

pub fn setup_handles(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut images: ResMut<Assets<Image>>,
    images: ResMut<Images>,
    quads: ResMut<Quads>,
) {
    let quads = quads.into_inner();
    let mesh = Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32., 48.)))));
    quads.insert(BundleType::SKELLY, mesh);
    let mesh = Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32., 44.)))));
    quads.insert(BundleType::HEX, mesh);

    let images = images.into_inner();
    let image = asset_server.load(HEX_SPRITE_PATH) as Handle<Image>;
    images.insert(BundleType::HEX, image);

    let image = asset_server.load(SKELLY_SPRITE_PATH) as Handle<Image>;
    images.insert(BundleType::SKELLY, image);
}

//pub fn debug_system(query: Query<&Mesh2dHandle, With<Unit>>) {
pub fn debug_system(res: Res<Images>) {
    //println!("Images {}", res.into_inner().len());
    // for transform in query.iter() {
    //     println!("UNIT {:?}", transform.0);
    // }
}
