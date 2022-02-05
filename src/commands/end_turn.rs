use crate::commands::CommandLike;
use crate::commands::TurnCommandResult;
use crate::components::Team;
use crate::systems::CurrentTurn;
use crate::systems::StartTurn;
use bevy::ecs::event::Events;
use bevy::prelude::*;
use utils::wrap;

pub struct EndTurn;

impl CommandLike for EndTurn {
    fn execute(&self, world: &mut World) -> TurnCommandResult {
        let mut result = TurnCommandResult::Failure;

        world.resource_scope(|world, current_turn: Mut<CurrentTurn>| {
            let current_team = current_turn.into_inner().0.expect("No current team found");
            let mut teams = world.query::<(Entity, With<Team>)>();

            let index = teams
                .iter(world)
                .into_iter()
                .position(|(team, _)| team == current_team);

            let next_index = wrap(
                (index.expect("current_team not found in teams") + 1) as f32,
                0.,
                teams.iter(world).count() as f32,
            ) as usize;

            let teams = teams
                .iter(world)
                .into_iter()
                .map(|(entity, _)| entity)
                .collect::<Vec<Entity>>();
            let next_team = teams.get(next_index).expect("No next team found");

            let events = world.get_resource_mut::<Events<StartTurn>>();
            events
                .expect("No events res found")
                .send(StartTurn { team: *next_team });

            result = TurnCommandResult::Success;
        });

        result
    }
}
