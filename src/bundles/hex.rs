use crate::components::AddHandle;
use crate::components::ColorFade;
use crate::components::Coordinates;
use crate::components::Layer;
use crate::components::Origin;
use crate::components::ZOrder;
use crate::states::BundleType;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components;

#[derive(Bundle)]
pub struct Hex {
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub hex: components::Hex,
    pub origin: Origin,
    pub z_order: ZOrder,
    pub add_handle: AddHandle,
}

impl Hex {
    pub fn new(coordinates: Coordinates) -> Self {
        Hex {
            hex: components::Hex,
            coordinates,
            origin: Origin(Vec3::new(0.0, -6.0, 0.0)),
            layer: Layer(4),
            color_fade: ColorFade(Color::WHITE),
            z_order: ZOrder,
            add_handle: AddHandle {
                bundle_type: BundleType::Hex,
            },
        }
    }
}
