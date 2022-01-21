use crate::components::Selected;
use crate::ShaderMaterial;
use bevy::prelude::*;

pub fn selected(
    selected_query: Query<&Handle<ShaderMaterial>, With<Selected>>,
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
