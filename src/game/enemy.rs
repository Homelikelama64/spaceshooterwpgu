use rand::Rng;
use slotmap::SlotMap;

use crate::{
    angletovector, enemy_dies, get_2_mut, particalexplosion, rotatevector, vectortoangle, Bullet,
    Enemy, Partical, Player, TextureID,
};
use raylib::prelude::*;

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
                enemy.targetpos = player.pos + player.dir * player.vel.length() * time_to_reach;
                time_to_reach = enemy.targetpos.distance_to(enemy.pos) / enemy.vel.length();
            }
        } else {
            if enemy.name == "Basic".to_string() {
                enemy.targetpos = player.pos
            }

            if enemy.name == "Turret".to_string() {
                enemy.targetpos = player.pos
                    + rotatevector(
                        (enemy.pos - player.pos).normalized(),
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
        enemy.vel += enemy.dir.normalized()
            * (enemy.speed
                - (enemy.vel.length() * (2.0 + (enemy.vel.normalized().dot(enemy.dir) - 1.0))
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
            if enemy.pos.distance_to(part.pos) < part.size + enemy.size {
                enemy.health = -1.0;
                part.health -= 1.0;
                particalexplosion(
                    particals,
                    part.pos,
                    player.vel,
                    0.0,
                    300.0,
                    500,
                    Color {
                        r: 140,
                        g: 255,
                        b: 251,
                        a: 255,
                    },
                    Color {
                        r: 255,
                        g: 0,
                        b: 50,
                        a: 0,
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
                (enemy.vel + player.vel) / 2.0 + (player.pos - enemy.pos).normalized() * 1000.0;
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
            bullet.pos.distance_to(enemy.pos) > bullet.size * 2.0 + enemy.size || !bullet.friendly
        });
        for part in &player.parts {
            bullets.retain(|bullet| {
                bullet.pos.distance_to(part.pos) > bullet.size * 2.0 + part.size || bullet.friendly
            });
        }
        for other_enemy_index in 0..enemies.len() {
            let Some((enemy, other_enemy)) = get_2_mut(enemies, enemy_index, other_enemy_index)
            else {
                continue;
            };
            if enemy.pos.distance_to(other_enemy.pos) < other_enemy.size + enemy.size {
                enemy.health = -1.0;
                other_enemy.health = -1.0;
            };
        }
    }
    for enemy in enemies {
        if enemy.health <= 0.0 {
            enemy_dies(enemy.pos, enemy.vel, particals);
        }
    }
}

pub fn draw_enemies(
    d: &mut RaylibDrawHandle,
    player: &Player,
    enemies: &Vec<Enemy>,
    textures: &SlotMap<TextureID, Texture2D>,
    enemy_warning_image: &Texture2D,
    screenwidth: i32,
    screenheight: i32,
) {
    for enemy in enemies {
        let pos = Vector2::new(
            enemy.pos.x - player.pos.x + screenwidth as f32 / 2.0,
            enemy.pos.y - player.pos.y + screenheight as f32 / 2.0,
        );
        let image: &Texture2D = &textures[enemy.texture_id];

        d.draw_texture_pro(
            image,
            Rectangle::new(0.0, 0.0, image.width as f32, image.height as f32),
            Rectangle::new(
                pos.x,
                pos.y,
                image.width as f32 * enemy.texture_scale,
                image.height as f32 * enemy.texture_scale,
            ),
            Vector2::new(
                image.width as f32 / 2.0 * enemy.texture_scale,
                image.height as f32 / 2.0 * enemy.texture_scale,
            ),
            vectortoangle(enemy.dir).to_degrees() + 90.0,
            Color::WHITE,
        );
        if enemy.name == "Turret".to_string() {
            d.draw_texture_pro(
                &textures[enemy.extra_texture_ids[0]],
                Rectangle::new(
                    0.0,
                    0.0,
                    textures[enemy.extra_texture_ids[0]].width as f32,
                    textures[enemy.extra_texture_ids[0]].height as f32,
                ),
                Rectangle::new(
                    pos.x,
                    pos.y + 1.0,
                    textures[enemy.extra_texture_ids[0]].width as f32 * enemy.texture_scale,
                    textures[enemy.extra_texture_ids[0]].height as f32 * enemy.texture_scale,
                ),
                Vector2::new(
                    textures[enemy.extra_texture_ids[0]].width as f32 / 2.0 * enemy.texture_scale,
                    textures[enemy.extra_texture_ids[0]].height as f32 / 2.0 * enemy.texture_scale,
                ),
                vectortoangle((player.pos - enemy.pos).normalized()).to_degrees() + 90.0,
                Color::WHITE,
            );
        }
        if player.pos.distance_to(enemy.pos) > 170.0 {
            d.draw_texture_v(
                &enemy_warning_image,
                (enemy.pos - player.pos).normalized() * 170.0
                    + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                    - Vector2::new(
                        enemy_warning_image.width as f32 / 2.0,
                        enemy_warning_image.height as f32 / 2.0,
                    ),
                Color::WHITE,
            )
        }
    }
}
