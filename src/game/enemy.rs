use cgmath::{InnerSpace, MetricSpace, Vector2, Vector4};
use rand::Rng;
use slotmap::SlotMap;

use crate::renderer::{
    texture::{Texture, TextureId},
    Rendering2D,
};

use super::{
    angletovector, enemy_dies, get_2_mut, particalexplosion, rotatevector, vectortoangle, Bullet,
    Enemy, Partical, Player,
};

pub fn update_enemies(
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
    particals: &mut Vec<Partical>,
    bullets: &mut Vec<Bullet>,
    dt: f32,
) {
    for enemy_index in 0..enemies.len() {
        let enemy = &mut enemies[enemy_index];
        let right = rotatevector(enemy.dir, std::f32::consts::PI / 2.0);
        let sign = if right.dot(player.pos - enemy.pos) > 0.0 {
            1.0
        } else {
            -1.0
        };
        if enemy.predictive {
            let mut time_to_reach = 0.0;
            for _ in 0..10 {
                enemy.targetpos = player.pos + player.dir * player.vel.magnitude() * time_to_reach;
                time_to_reach = enemy.targetpos.distance(enemy.pos) / enemy.vel.magnitude();
            }
        } else {
            if enemy.name == "Basic".to_string() {
                enemy.targetpos = player.pos
            }

            if enemy.name == "Turret".to_string() {
                enemy.targetpos = player.pos
                    + rotatevector(
                        (enemy.pos - player.pos).normalize(),
                        std::f32::consts::TAU / 8.0 * sign,
                    ) * 200.0
            }
        }
        if right.dot(enemy.targetpos - enemy.pos) > 0.0 {
            enemy.dir =
                angletovector(vectortoangle(enemy.dir) + (enemy.turningspeed.to_radians() * dt))
        } else {
            enemy.dir =
                angletovector(vectortoangle(enemy.dir) - (enemy.turningspeed.to_radians() * dt))
        }
        enemy.vel += enemy.dir.normalize()
            * (enemy.speed
                - (enemy.vel.magnitude() * (2.0 + (enemy.vel.normalize().dot(enemy.dir) - 1.0))
                    / 2.0))
            * dt;

        enemy.vel -= right * (right.dot(enemy.vel)) * enemy.friction * dt;

        enemy.pos += enemy.vel * dt;

        for partical_emmiter in &mut enemy.partical_emmiters {
            partical_emmiter.pos = enemy.pos
                + rotatevector(
                    partical_emmiter.location,
                    vectortoangle(enemy.dir) - std::f32::consts::PI / 2.0,
                );
            partical_emmiter.vel = enemy.vel
                + -enemy.dir * partical_emmiter.speed
                + angletovector(
                    rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                ) * rand::thread_rng().gen_range(20.0..40.0);
            partical_emmiter.time += dt;
            while partical_emmiter.time > partical_emmiter.partical_interval {
                particals.push(Partical {
                    pos: partical_emmiter.pos,
                    vel: partical_emmiter.vel,
                    size: partical_emmiter.size,
                    shape: partical_emmiter.shape.clone(),
                    starting_color: partical_emmiter.starting_color,
                    ending_color: partical_emmiter.ending_color,
                    duration: partical_emmiter.duration,
                    time: 0.0,
                });
                partical_emmiter.time -= partical_emmiter.partical_interval;
            }
        }
        for part in &mut player.parts {
            if enemy.pos.distance(part.pos) < part.size + enemy.size {
                enemy.health = -1.0;
                part.health -= 1.0;
                particalexplosion(
                    particals,
                    part.pos,
                    player.vel,
                    0.0,
                    300.0,
                    500,
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
                    1.0,
                );
            }
        }
        for bullet_emmiter in &mut enemy.bullet_emmiters {
            bullet_emmiter.pos = enemy.pos
                + rotatevector(
                    bullet_emmiter.location,
                    vectortoangle(player.pos - enemy.pos) - std::f32::consts::PI / 2.0,
                );
            let vel =
                (enemy.vel + player.vel) / 2.0 + (player.pos - enemy.pos).normalize() * 1000.0;
            bullet_emmiter.time += dt;
            while bullet_emmiter.time > bullet_emmiter.bullet_interval {
                if true {
                    bullets.push(Bullet {
                        pos: bullet_emmiter.pos,
                        vel: vel,
                        size: bullet_emmiter.size,
                        damage: bullet_emmiter.damage,
                        friendly: bullet_emmiter.friendly,
                        duration: bullet_emmiter.duration,
                        time: 0.0,
                    });
                }
                bullet_emmiter.time -= bullet_emmiter.bullet_interval;
            }
        }
        bullets.retain(|bullet| {
            bullet.pos.distance(enemy.pos) > bullet.size * 2.0 + enemy.size || !bullet.friendly
        });
        for part in &player.parts {
            bullets.retain(|bullet| {
                bullet.pos.distance(part.pos) > bullet.size * 2.0 + part.size || bullet.friendly
            });
        }
        for other_enemy_index in 0..enemies.len() {
            let Some((enemy, other_enemy)) = get_2_mut(enemies, enemy_index, other_enemy_index)
            else {
                continue;
            };
            if enemy.pos.distance(other_enemy.pos) < other_enemy.size + enemy.size {
                enemy.health = -1.0;
                other_enemy.health = -1.0;
            };
        }
    }
    for enemy in enemies.iter_mut() {
        if enemy.health <= 0.0 {
            enemy_dies(enemy.pos, enemy.vel, particals);
        }
    }
    enemies.retain(|enemy| enemy.health > 0.0);
}

pub fn draw_enemies(
    drawing: &mut Rendering2D<'_, '_>,
    player: &Player,
    enemies: &Vec<Enemy>,
    enemy_warning_image: &TextureId,
) {
    for enemy in enemies {

        drawing.draw_quad(
            enemy.pos,
            Vector2 { x: 32.0, y: 32.0 },
            Vector4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            vectortoangle(enemy.dir).to_degrees() - 90.0,
            Some(enemy.texture_id),
        );
        if enemy.name == "Turret" {
            drawing.draw_quad(
                enemy.pos,
                Vector2 { x: 32.0, y: 32.0 },
                Vector4 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: 1.0,
                },
                (player.pos - enemy.pos).normalize().angle(Vector2::unit_y()).0.to_degrees() - 90.0,
                Some(enemy.extra_texture_ids[0]),
            );
        }
        if player.pos.distance(enemy.pos) > 170.0 {
            drawing.draw_quad(
                (enemy.pos - player.pos).normalize() * 170.0 + player.pos,
                Vector2 { x: 32.0, y: 32.0 },
                Vector4 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: 1.0,
                },
                0.0,
                Some(*enemy_warning_image),
            );
        }
    }
}
