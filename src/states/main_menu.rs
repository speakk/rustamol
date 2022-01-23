use crate::bundles;
use crate::components::Hex;
use crate::models;
use crate::AppState;
use bevy::prelude::*;

pub struct StatePlugin;

const NORMAL_BUTTON_COLOR: Color = Color::hsla(0.2, 0.4, 0.2, 1.);
const HOVERED_BUTTON_COLOR: Color = Color::hsla(0.0, 0.5, 0.24, 1.);
const PRESSED_BUTTON_COLOR: Color = Color::hsla(0.0, 0.7, 0.2, 1.);
const BUTTON_TEXT_COLOR: Color = Color::hsla(0.5, 0.5, 0.4, 1.);

type MenuButton = Entity;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::MainMenu;
        app.add_system_set(
            SystemSet::on_enter(STATE)
                .with_system(setup_background_hexes)
                .with_system(setup_ui),
        );
        app.add_system_set(SystemSet::on_update(STATE).with_system(menu_interact));
        app.add_system_set(SystemSet::on_exit(STATE).with_system(cleanup_menu));
    }
}

//pub fn setup_background_hexes(mut spawn_hex: EventWriter<bundles::SpawnHex>) {
pub fn setup_background_hexes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hexes = models::map::create_grid(35, models::MapShape::Square);
    let mut hex_entities: Vec<Entity> = vec![];
    for hex in hexes {
        //hex_entities.push(commands.add(bundles::SpawnHex { q: hex.q, r: hex.r }));
        hex_entities.push(bundles::spawn_hex(&hex, &mut commands, &asset_server));
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/ThaleahFat.ttf"),
                        font_size: 40.0,
                        color: BUTTON_TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(button_entity as MenuButton);
}

fn menu_interact(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands,
    menu_data: Res<MenuButton>,
    mut hexes: Query<Entity, With<Hex>>,
) {
    commands.entity(*menu_data.into_inner()).despawn_recursive();
    for entity in hexes.iter_mut() {
        commands.entity(entity).despawn();
    }
}
