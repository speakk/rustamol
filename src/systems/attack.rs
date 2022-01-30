use crate::commands::*;
use bevy::prelude::*;

pub fn attack(mut events: EventReader<Attack>) {
    for event in events.iter() {
        println!("Attack!");
    }
}
