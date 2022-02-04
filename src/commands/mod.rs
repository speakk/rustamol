use crate::components::*;
use bevy::ecs::entity::Entity;
use bevy::ecs::world::World;

pub struct TurnCommandEvent {
    pub command: Box<dyn CommandLike>,
    pub team: Option<Entity>,
}

// unsafe impl Sync for TurnCommandEvent {}
// unsafe impl Send for TurnCommandEvent {}

// pub struct TurnCommandEvent {
//     pub command: TurnCommand,
//     pub team: Option<Entity>,
// }
//
// pub struct TurnCommandFinishedEvent {
//     pub command: TurnCommand,
// }

// pub enum TurnCommand {
//     MoveEntity(MoveEntity),
//     // EndTurn(EndTurn),
//     // Attack(Attack),
//     // MoveAndAttack(MoveAndAttack),
// }

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

#[derive(Debug, Copy, Clone)]
pub struct EndTurn;

pub mod move_entity;
pub use move_entity::*;
