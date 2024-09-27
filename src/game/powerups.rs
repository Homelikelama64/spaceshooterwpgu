use super::{angletovector, get_2_mut, Player, PowerUp, PowerUpType};
use crate::renderer::Rendering2D;
use cgmath::{InnerSpace, MetricSpace, Vector2, Vector4};
use rand::prelude::*;

pub fn power_ups_update(
    drawing: &mut Rendering2D<'_, '_>,
    player: &mut Player,
    power_ups: &mut Vec<PowerUp>,
) {
    for power_up in power_ups {
        for part_index in 0..player.parts.len() {
            let part = &mut player.parts[part_index];
            if part.pos.distance(power_up.pos) < part.size + 16.0 {
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
        //d.draw_texture_v(
        //    &power_up.texture,
        //    power_up.pos - player.pos
        //        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
        //        - Vector2::new(
        //            *&power_up.texture.width as f32 / 2.0,
        //            *&power_up.texture.height as f32 / 2.0,
        //        ),
        //    Color::WHITE,
        //);
        drawing.draw_quad(
            power_up.pos,
            Vector2 { x: 32.0, y: 32.0 },
            Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            0.0,
            Some(power_up.texture),
        );
        if player.pos.distance(power_up.pos) > 210.0 {
            //d.draw_texture_v(
            //    &&power_up.texture,
            //    (power_up.pos - player.pos).normalized() * 210.0
            //        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
            //        - Vector2::new(
            //            *&power_up.texture.width as f32 / 2.0,
            //            *&power_up.texture.height as f32 / 2.0,
            //        ),
            //    Color::WHITE,
            //)
            drawing.draw_quad(
                (power_up.pos - player.pos).normalize() * 210.0 + player.pos,
                Vector2 { x: 32.0, y: 32.0 },
                Vector4 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: 1.0,
                },
                0.0,
                Some(power_up.texture),
            );
        }
    }
}
