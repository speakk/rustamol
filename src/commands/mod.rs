use crate::components::*;
use bevy::ecs::entity::Entity;

pub struct TurnCommandEvent {
    pub command: TurnCommand,
    pub team: Option<Entity>,
}

pub enum TurnCommand {
    MoveEntity(MoveEntity),
    EndTurn(EndTurn),
    Attack(Attack),
}

#[derive(Debug, Copy, Clone)]
pub struct MoveEntity {
    pub from: Coordinates,
    pub to: Coordinates,
}

#[derive(Debug, Copy, Clone)]
pub struct Attack {
    pub from: Coordinates,
    pub to: Coordinates,
}

#[derive(Debug, Copy, Clone)]
pub struct EndTurn;
