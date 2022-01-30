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

extern crate lazy_static;

#[derive(Bundle)]
pub struct Unit {
    pub coordinates: Coordinates,
    pub layer: Layer,
    pub color_fade: ColorFade,
    pub unit: components::Unit,
    pub origin: Origin,
    pub add_handle: AddHandle,
    pub z_order: ZOrder,
}

pub enum UnitType {
    Skelly,
    Stabby,
}

impl Unit {
    pub fn new(coordinates: Coordinates, unit_type: UnitType) -> Self {
        Unit {
            unit: components::Unit,
            // TODO: Make the origin coordinates normalized
            origin: Origin(Vec3::new(0.0, 23.0, 0.0)),
            coordinates,
            layer: Layer(5),
            color_fade: ColorFade(Color::WHITE),
            add_handle: AddHandle {
                bundle_type: match unit_type {
                    UnitType::Skelly => BundleType::Skelly,
                    UnitType::Stabby => BundleType::Stabby,
                },
            },
            z_order: ZOrder,
        }
    }
}
