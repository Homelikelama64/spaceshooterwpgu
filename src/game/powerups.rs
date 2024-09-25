use rand::prelude::*;
use raylib::prelude::*;

use crate::{angletovector, get_2_mut, Player, PowerUp, PowerUpType};

pub fn power_ups_update(
    d: &mut RaylibDrawHandle,
    player: &mut Player,
    power_ups: &mut Vec<PowerUp>,
    screenwidth: i32,
    screenheight: i32,
) {
    for power_up in power_ups {
        for part_index in 0..player.parts.len() {
            let part = &mut player.parts[part_index];
            if part.pos.distance_to(power_up.pos) < part.size + 16.0 {
                match power_up.power_type {
                    PowerUpType::Shield => {}
                    PowerUpType::Repair => {
                        for other_part_index in 0..player.parts.len() {
                            let Some((part, other_part)) =
                                get_2_mut(&mut player.parts, part_index, other_part_index)
                            else {
                                continue;
                            };
                            part.health = part.starting_health;
                            other_part.health = other_part.starting_health;
                        }
                        power_up.pos = player.pos
                            + angletovector(
                                rand::thread_rng()
                                    .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                            ) * rand::thread_rng().gen_range(2000.0..2500.0)
                    }
                }
            }
        }
        d.draw_texture_v(
            &power_up.texture,
            power_up.pos - player.pos
                + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                - Vector2::new(
                    *&power_up.texture.width as f32 / 2.0,
                    *&power_up.texture.height as f32 / 2.0,
                ),
            Color::WHITE,
        );
        if player.pos.distance_to(power_up.pos) > 210.0 {
            d.draw_texture_v(
                &&power_up.texture,
                (power_up.pos - player.pos).normalized() * 210.0
                    + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                    - Vector2::new(
                        *&power_up.texture.width as f32 / 2.0,
                        *&power_up.texture.height as f32 / 2.0,
                    ),
                Color::WHITE,
            )
        }
    }
}
