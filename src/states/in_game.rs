use crate::bundles;
use crate::components::Coordinates;
use crate::models;
use crate::systems;
use crate::AppState;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::InGame;
        app.add_system_set(SystemSet::on_enter(STATE).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(STATE)
                .with_system(systems::click_handler)
                .with_system(systems::selected)
                .with_system(systems::find_path_hilight)
                .with_system(systems::path_hilight)
                .with_system(systems::handle_hex_occupants),
        );
    }
}

pub fn setup(mut commands: Commands) {
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
}
