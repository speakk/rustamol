use crate::components::Selected;
use bevy::prelude::*;

pub fn selected(mut selected_query: Query<&mut Sprite, Changed<Selected>>) {
    for mut sprite in selected_query.iter_mut() {
        sprite.color.set_b(0.5);
    }
}

pub fn selected_removal(
    removed_selected: RemovedComponents<Selected>,
    mut query: Query<&mut Sprite>,
) {
    for entity in removed_selected.iter() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            sprite.color.set_b(1.0);
        }
    }
}
