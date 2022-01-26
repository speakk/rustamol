use crate::components::Layer;
use crate::components::ZOrder;
use bevy::prelude::*;

pub fn z_order(mut sprites: Query<(&mut Transform, Option<&Layer>, With<ZOrder>)>) {
    for (mut transform, layer, _) in sprites.iter_mut() {
        // TODO: Fix magic number
        transform.translation.z = 10.0 + -transform.translation.y * 0.01;

        if let Some(layer) = layer {
            transform.translation.z += layer.0 as f32;
        }
    }
}
