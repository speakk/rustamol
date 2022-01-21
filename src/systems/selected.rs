use crate::components::Selected;
use crate::ShaderMaterial;
use bevy::prelude::*;

pub fn selected(
    selected_query: Query<&Handle<ShaderMaterial>, Changed<Selected>>,
    mut assets: ResMut<Assets<ShaderMaterial>>,
) {
    for handle in selected_query.iter() {
        let mat = assets.get_mut(handle);

        if let Some(mut mat) = mat {
            mat.outline = true;
            mat.color.set_b(0.5);
        }
    }
}

pub fn selected_removal(
    removed_selected: RemovedComponents<Selected>,
    query: Query<&Handle<ShaderMaterial>>,
    mut assets: ResMut<Assets<ShaderMaterial>>,
) {
    for entity in removed_selected.iter() {
        if let Ok(handle) = query.get(entity) {
            if let Some(mut mat) = assets.get_mut(handle) {
                mat.outline = false;
                mat.color.set_b(1.0);
            }
        }
    }
}
