use crate::states::BundleType;
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
