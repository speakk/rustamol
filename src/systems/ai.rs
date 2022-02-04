use crate::commands::EndTurn;
//use crate::commands::TurnCommand;
use crate::commands::TurnCommandEvent;
use crate::components::*;
use crate::systems::TurnStarted;
use bevy::prelude::*;

pub fn ai(
    mut turn_starts: EventReader<TurnStarted>,
    ai_teams: Query<Entity, (With<Team>, With<AiControlled>)>,
    mut turn_commands: EventWriter<TurnCommandEvent>,
) {
    for turn_start in turn_starts.iter() {
        let ai_team = ai_teams.get(turn_start.team);
        if let Ok(ai_team) = ai_team {
            // TODO: Add back in
            // turn_commands.send(TurnCommandEvent {
            //     command: TurnCommand::EndTurn(EndTurn),
            //     team: Some(ai_team),
            // });
        }
    }
}
