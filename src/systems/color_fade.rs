use crate::components::ColorFade;
use bevy::prelude::*;

pub fn color_fade(mut sprites: Query<(&mut Sprite, &ColorFade), With<Sprite>>) {
    for (mut sprite, color_fade) in sprites.iter_mut() {
        let current_color = sprite.color.as_rgba_f32();
        let target_color = color_fade.0.as_rgba_f32();
        let final_color =
            Vec4::from_slice(&current_color).lerp(Vec4::from_slice(&target_color), 0.05);
        sprite.color = Color::from(final_color);
    }
}
