use bevy::ecs::component::Component;

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
