use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::render::color::Color;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct MapCoordinates {
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
