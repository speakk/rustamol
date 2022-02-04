use crate::commands::*;
use bevy::prelude::*;

pub fn attack(mut events: EventReader<Attack>) {
    for event in events.iter() {
        println!("Attack!");
    }
}

pub fn move_and_attack(mut events: EventReader<MoveAndAttack>) {
    for event in events.iter() {
        println!("Attack!");
    }
}
