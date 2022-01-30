use crate::bundles;
use crate::commands::EndTurn;
use crate::commands::MoveEntity;
use crate::commands::TurnCommandEvent;
use crate::components::*;
use crate::models;
use crate::systems;
use crate::systems::CurrentTurn;
use crate::AppState;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::InGame;
        app.add_system_set(
            SystemSet::on_enter(STATE)
                .with_system(setup)
                .with_system(setup_debug),
        )
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(systems::click_handler)
                .with_system(systems::selected)
                .with_system(systems::start_turn)
                .with_system(systems::end_turn)
                .with_system(systems::find_path_hilight)
                .with_system(systems::keyboard_handler)
                .with_system(systems::turn_command)
                .with_system(systems::move_entity)
                .with_system(systems::ai)
                .with_system(systems::path_hilight)
                .with_system(systems::handle_hex_occupants)
                .with_system(update_team_text),
        )
        .init_resource::<CurrentTurn>()
        .add_event::<TurnCommandEvent>()
        .add_event::<MoveEntity>()
        .add_event::<EndTurn>()
        .add_event::<systems::turn::StartTurn>()
        .add_event::<systems::turn::TurnStarted>();

        let world = &mut app.world;
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/m5x7.ttf");
        app.insert_resource(TeamTextStyle(TextStyle {
            font,
            font_size: 24.0,
            color: Color::WHITE,
        }));
    }
}

pub fn setup(mut commands: Commands, mut start_turn: EventWriter<systems::turn::StartTurn>) {
    let hexes = models::map::create_grid(8, models::MapShape::Hexagonal);
    for hex in hexes {
        commands.spawn_bundle(bundles::Hex::new(hex));
    }

    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 0, r: -2 }));
    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 2, r: 0 }));
    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 2, r: 1 }));
    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 1, r: 2 }));
    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 3, r: 1 }));
    commands.spawn_bundle(bundles::Unit::new(Coordinates { q: 0, r: -4 }));

    let team1 = commands
        .spawn()
        .insert(Team {
            name: "PlayerTeam".to_string(),
        })
        .insert(PlayerControlled)
        .id();
    commands
        .spawn()
        .insert(Team {
            name: "AI".to_string(),
        })
        .insert(AiControlled);

    start_turn.send(systems::turn::StartTurn { team: team1 });
}

#[derive(Component)]
struct TeamSelectorText;

pub fn setup_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(15.0),
                    top: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Teams...",
                TextStyle {
                    font: asset_server.load("fonts/m5x7.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(TeamSelectorText);
}

struct TeamTextStyle(TextStyle);

fn update_team_text(
    teams: Query<(Entity, &Team), With<Team>>,
    mut text_query: Query<&mut Text, With<TeamSelectorText>>,
    text_style: Res<TeamTextStyle>,
    current_team: Res<CurrentTurn>,
) {
    if let Some(current_team) = current_team.0 {
        // TODO: Maybe optimize this if needed....
        for mut text in text_query.iter_mut() {
            text.sections = teams
                .iter()
                .map(|(entity, team)| TextSection {
                    value: format!(
                        "{} {}\n",
                        team.name,
                        if entity == current_team { "<" } else { "" }
                    ),
                    style: text_style.0.clone(),
                })
                .collect();
        }
    }
}
