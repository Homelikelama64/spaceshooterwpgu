
use crate::{colorlerp, Partical, ParticalShape, Player};
use raylib::prelude::*;

pub fn update_particals(particals: &mut Vec<Partical>, dt: f32) {
    for partical in &mut *particals {
        partical.pos += partical.vel * dt;
        partical.time += dt;
    }
    particals.retain(|partical| partical.time < partical.duration);
}

pub fn draw_particals(
    d: &mut RaylibDrawHandle,
    player: &Player,
    particals: &mut Vec<Partical>,
    screenwidth: i32,
    screenheight: i32,
) {
    for partical in particals {
        let lerped_color = colorlerp(
            partical.starting_color,
            partical.ending_color,
            partical.time / partical.duration,
        );
        match partical.shape {
            ParticalShape::Square => d.draw_rectangle_v(
                Vector2::new(
                    partical.pos.x - player.pos.x + screenwidth as f32 / 2.0 - partical.size / 2.0,
                    partical.pos.y - player.pos.y + screenheight as f32 / 2.0 - partical.size / 2.0,
                ),
                Vector2::new(partical.size, partical.size),
                lerped_color,
            ),
            ParticalShape::Circle => d.draw_circle_v(
                Vector2::new(
                    partical.pos.x - player.pos.x + screenwidth as f32 / 2.0,
                    partical.pos.y - player.pos.y + screenheight as f32 / 2.0,
                ),
                partical.size / 2.0,
                lerped_color,
            ),
            ParticalShape::RotSquare => {}
        }
    }
}
