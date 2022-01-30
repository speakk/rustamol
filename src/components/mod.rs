use crate::states::BundleType;
use bevy::core::Timer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::render::color::Color;

#[derive(Component, Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coordinates {
    pub q: i32,
    pub r: i32,
}

#[derive(Component)]
pub struct Hex;

#[derive(Component)]
pub struct Layer(pub i32);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ColorFade(pub Color);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Origin(pub Vec3);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct ZOrder;

#[derive(Component)]
pub struct AddHandle {
    pub bundle_type: BundleType,
}

#[derive(Component, Default)]
pub struct Path(pub Vec<Coordinates>);

#[derive(Component)]
pub struct PathTimer(pub Timer);

#[derive(Component)]
pub struct PathCurrentIndex(pub usize);

#[derive(Bundle)]
pub struct TimedPath {
    pub path: Path,
    pub timer: PathTimer,
    pub current_index: PathCurrentIndex,
}

impl TimedPath {
    pub fn new(path: Path, timer: Option<PathTimer>) -> Self {
        TimedPath {
            path,
            timer: timer.unwrap_or_else(|| PathTimer(Timer::from_seconds(0.05, true))),
            current_index: PathCurrentIndex(0),
        }
    }
}

#[derive(Component)]
pub struct Team {
    pub name: String,
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AiControlled;
