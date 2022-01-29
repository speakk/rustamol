use crate::components::*;
use bevy::ecs::entity::Entity;

pub trait CommandLike {}

pub struct TurnCommand<C: CommandLike> {
    pub command: Box<C>,
    pub team: Option<Entity>,
}

#[derive(Debug)]
pub struct MoveEntity {
    pub from: Coordinates,
    pub to: Coordinates,
}

impl CommandLike for MoveEntity {}
