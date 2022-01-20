use crate::components::Selected;
use bevy::prelude::*;

pub fn selected(mut selected_query: Query<&mut Sprite, With<Selected>>) {
    for mut sprite in selected_query.iter_mut() {
        sprite.color.set_b(0.0);
    }
}
