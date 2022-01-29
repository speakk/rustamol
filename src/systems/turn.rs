use bevy::prelude::*;

pub struct StartTurn {
    pub team: Entity,
}

#[derive(Default)]
pub struct CurrentTurn(pub Option<Entity>);

pub fn start_turn(mut events: EventReader<StartTurn>, mut current_turn: ResMut<CurrentTurn>) {
    for event in events.iter() {
        *current_turn = CurrentTurn(Some(event.team))
    }
}
// pub fn turn() {}
