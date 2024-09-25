use cgmath::{InnerSpace, MetricSpace, Vector2, Vector4};

use crate::renderer::Rendering2D;

use super::{particalexplosion, Bullet, Enemy, Partical, Player};


pub fn update_bullets(player: &mut Player, bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>,particals: &mut Vec<Partical>,dt:f32) {
    for bullet in bullets {
        bullet.pos += bullet.vel * dt;
        bullet.time += dt;
        if bullet.friendly {
            for enemy in enemies.iter_mut() {
                if bullet.pos.distance(enemy.pos) < bullet.size * 2.0 + enemy.size {
                    enemy.health -=
                        bullet.damage - bullet.time / bullet.duration * bullet.damage;
                    particalexplosion(
                        particals,
                        bullet.pos,
                        player.vel,
                        0.0,
                        600.0,
                        50,
                        Vector4 {
                            x: 255.0 / 255.0,
                            y: 0.0 / 255.0,
                            z: 0.0 / 255.0,
                            w: 255.0 / 255.0,
                        },
                        Vector4 {
                            x: 255.0 / 255.0,
                            y: 255.0 / 255.0,
                            z: 50.0 / 255.0,
                            w: 0.0 / 255.0,
                        },
                        0.1,
                    );
                }
            }
        }
        if !bullet.friendly {
            for part in &mut player.parts {
                if bullet.pos.distance(part.pos) < bullet.size * 2.0 + part.size {
                    part.health -=
                        bullet.damage - bullet.time / bullet.duration * bullet.damage;
                    particalexplosion(
                        particals,
                        bullet.pos,
                        player.vel,
                        0.0,
                        600.0,
                        50,
                        Vector4 {
                            x: 140.0 / 255.0,
                            y: 255.0 / 255.0,
                            z: 251.0 / 255.0,
                            w: 255.0 / 255.0,
                        },
                        Vector4 {
                            x: 255.0 / 255.0,
                            y: 0.0 / 255.0,
                            z: 50.0 / 255.0,
                            w: 0.0 / 255.0,
                        },
                        0.1,
                    )
                }
            }
        }
    }
}

pub fn draw_bullets(drawing:&mut Rendering2D<'_,'_>,player: &Player,bullets: &mut Vec<Bullet>,screenwidth:i32,screenheight:i32) {
    for bullet in bullets {
        let bullet_scale = 1.0 - bullet.time / bullet.duration;
        let bullet_width = bullet.size * bullet_scale;
        let bullet_length = bullet.size * 2.0 * bullet_scale;
        let mut color = Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
        if !bullet.friendly {
            color = Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0 }
        }
        drawing.draw_quad(
            bullet.pos,
            Vector2 { x: bullet_width, y: bullet_length },
            color,
            bullet.vel.angle(Vector2::unit_y()).0.to_degrees(),
            None,
        );
        //d.draw_rectangle_pro(
        //    Rectangle::new(
        //        bullet.pos.x - player.pos.x + screenwidth as f32 / 2.0,
        //        bullet.pos.y - player.pos.y + screenheight as f32 / 2.0,
        //        bullet_width,
        //        bullet_length,
        //    ),
        //    Vector2::new(bullet_width, bullet_length),
        //    vectortoangle(bullet.vel).to_degrees() + 90.0,
        //    color,
        //)
    }
}