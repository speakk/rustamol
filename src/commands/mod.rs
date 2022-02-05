use crate::components::*;
use bevy::ecs::entity::Entity;
use bevy::ecs::world::World;

pub struct TurnCommandEvent {
    pub command: Box<dyn CommandLike>,
    pub team: Option<Entity>,
}

pub enum TurnCommandResult {
    Success,
    Failure,
}

pub trait CommandLike: Send + Sync {
    fn execute(&self, world: &mut World) -> TurnCommandResult;
}

#[derive(Debug, Copy, Clone)]
pub struct Attack {
    pub from: Coordinates,
    pub to: Coordinates,
}

#[derive(Debug, Copy, Clone)]
pub struct MoveAndAttack {
    pub from: Coordinates,
    pub to: Coordinates,
}

pub mod move_entity;
pub use move_entity::*;

pub mod end_turn;
pub use end_turn::*;
