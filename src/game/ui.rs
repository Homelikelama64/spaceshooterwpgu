use raylib::prelude::*;

use crate::Player;

pub fn draw_timer(d: &mut RaylibDrawHandle, time: f32, screenwidth: i32) {
    d.draw_text(
        format!("Time: {:.1}", time).as_str(),
        screenwidth / 2 - 50,
        10,
        36,
        Color::WHITE,
    );
}

pub fn draw_part_health(
    d: &mut RaylibDrawHandle,
    player: &Player,
    default_font: &WeakFont,
    screenwidth: i32,
) {
    let longest_name_len = player
        .parts
        .iter()
        .map(|part| part.name.len())
        .max()
        .unwrap_or(0);

    for part_index in 0..player.parts.len() {
        let part = &player.parts[part_index];
        let font_size = 18.0;
        let spacing = 5.0;
        let name_length = longest_name_len as f32 * spacing;
        d.draw_text_pro(
            &default_font,
            part.name.as_str(),
            Vector2 {
                x: screenwidth as f32 - name_length as f32 * 3.0,
                y: 10.0 + (part_index as f32 * (font_size + 15.0)),
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            font_size,
            spacing,
            Color::WHITE,
        );
        d.draw_text_pro(
            &default_font,
            format!("Health: {:.1}%", part.health / part.starting_health * 100.0).as_str(),
            Vector2 {
                x: screenwidth as f32 - name_length as f32 * 3.0 + 10.0,
                y: 28.0 + (part_index as f32 * (font_size + 15.0)),
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            font_size / 1.1,
            spacing / 1.1,
            Color::WHITE,
        );
    }
}
