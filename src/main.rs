#![feature(box_patterns)]

use bevy::prelude::*;

#[macro_use]

mod bundles;
mod commands;
mod components;
mod models;
mod states;
mod systems;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub static AFTER: &str = "after_update";
pub static BEFORE: &str = "before_update";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    MainMenu,
}

fn main() {
    App::new()
        .add_state(AppState::MainMenu)
        .add_plugin(states::shared::StatePlugin)
        .add_plugin(states::main_menu::StatePlugin)
        .add_plugin(states::in_game::StatePlugin)
        .run();
}
