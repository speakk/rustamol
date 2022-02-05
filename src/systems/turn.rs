use crate::commands::EndTurn;
use crate::components::Team;
use bevy::prelude::*;
use utils::math::wrap;

pub struct StartTurn {
    pub team: Entity,
}

pub struct TurnStarted {
    pub team: Entity,
}

#[derive(Default, Debug)]
pub struct CurrentTurn(pub Option<Entity>);

pub fn start_turn(
    mut events: EventReader<StartTurn>,
    mut current_turn: ResMut<CurrentTurn>,
    mut turn_started: EventWriter<TurnStarted>,
) {
    for event in events.iter() {
        *current_turn = CurrentTurn(Some(event.team));
        println!("Got a turn start... ?, {:?}", current_turn);
        turn_started.send(TurnStarted {
            team: current_turn
                .0
                .expect("No current team when trying to send turn_started"),
        });
    }
}

pub fn end_turn(
    mut events: EventReader<EndTurn>,
    current_turn: ResMut<CurrentTurn>,
    teams: Query<Entity, With<Team>>,
    mut start_turn: EventWriter<StartTurn>,
) {
    for _ in events.iter() {
        let current_team = current_turn
            .0
            .expect("Tried to end current turn, but there was no current turn");

        let index = teams
            .iter()
            .into_iter()
            .position(|team| team == current_team);

        let next_index = wrap(
            (index.expect("current_team not found in teams") + 1) as f32,
            0.,
            teams.iter().count() as f32 - 1.,
        ) as usize;

        let teams = teams.iter().into_iter().collect::<Vec<Entity>>();
        let next_team = teams.get(next_index).expect("No next team found");

        start_turn.send(StartTurn { team: *next_team });
    }
}
// pub fn turn() {}
