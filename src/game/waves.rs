use std::path::Path;

use cgmath::{Vector2, Vector4, Zero};
use rand::Rng;
use slotmap::SlotMap;

use crate::{game::{load_texture, BulletEmitter, Enemy, ParticalEmitter, ParticalShape}, renderer::{texture::{Texture, TextureId}, Renderer, Rendering2D}};

use super::{angletovector, Player, Wave};


pub fn init_waves(
    renderer:&mut Renderer,
) -> Vec<Wave> {
    vec![
        Wave {
            interval: 6.0,
            min_interval: 1.0,
            interval_delta: 0.3,
            double_spawn_chance: 0.5,
            max_double_spawn_chance: 0.7,
            time: 0.0,
            enemy: Enemy {
                name: format!("Basic"),
                pos: Vector2::zero(),
                vel: Vector2 { x: 0.0, y: 1.0 },
                dir: Vector2::zero(),
                targetpos: Vector2 { x: 200.0, y: 200.0 },
                speed: 600.0,
                turningspeed: 100.0,
                predictive: false,
                texture_scale: 1.0,
                friction: 1.0,
                size: 16.0,
                health: 1.0,
                partical_emmiters: vec![ParticalEmitter {
                    pos: Vector2::zero(),
                    location: Vector2 { x: 0.0, y: -13.0 },
                    vel: Vector2::zero(),
                    speed_orginal: 400.0,
                    size: 5.0,
                    shape: ParticalShape::Square,
                    starting_color: Vector4 {
                        x: 255.0 / 255.0,
                        y: 255.0 / 255.0,
                        z: 0.0 / 255.0,
                        w: 255.0 / 255.0,
                    },
                    ending_color: Vector4 {
                        x: 255.0 / 255.0,
                        y: 0.0 / 255.0,
                        z: 50.0 / 255.0,
                        w: 0.0 / 255.0,
                    },
                    duration: 1.0,
                    partical_interval: 1.0 / 400.0,
                    time: 0.0,
                    speed: 0.0,
                }],
                bullet_emmiters: vec![],
                texture_id: load_texture(renderer, "Basic Enemy Texture", Path::new("images/V1Enemy.png")),
                extra_texture_ids: vec![],
            },
        },
        Wave {
            interval: 16.0,
            min_interval: 7.0,
            interval_delta: 0.3,
            double_spawn_chance: 0.1,
            max_double_spawn_chance: 0.5,
            time: 0.0,
            enemy: Enemy {
                name: format!("Turret"),
                pos: Vector2::zero(),
                vel: Vector2 { x: 0.0, y: 1.0 },
                dir: Vector2::zero(),
                targetpos: Vector2 { x: 200.0, y: 200.0 },
                speed: 500.0,
                turningspeed: 100.0,
                predictive: false,
                texture_scale: 1.5,
                friction: 1.0,
                size: 24.0,
                health: 7.0,
                partical_emmiters: vec![ParticalEmitter {
                    pos: Vector2::zero(),
                    location: Vector2 { x: 0.0, y: -15.0 },
                    vel: Vector2::zero(),
                    speed_orginal: 800.0,
                    size: 10.0,
                    shape: ParticalShape::Square,
                    starting_color: Vector4 {
                        x: 255.0 / 255.0,
                        y: 255.0 / 255.0,
                        z: 0.0 / 255.0,
                        w: 255.0 / 255.0,
                    },
                    ending_color: Vector4 {
                        x: 255.0 / 255.0,
                        y: 0.0 / 255.0,
                        z: 50.0 / 255.0,
                        w: 0.0 / 255.0,
                    },
                    duration: 1.0,
                    partical_interval: 1.0 / 400.0,
                    time: 0.0,
                    speed: 0.0,
                }],
                bullet_emmiters: vec![BulletEmitter {
                    pos: Vector2::zero(),
                    location: Vector2 { x: 0.0, y: 10.0 },
                    size: 5.0,
                    damage: 0.3,
                    friendly: false,
                    duration: 2.0,
                    bullet_interval: 1.0 / 2.0,
                    time: 0.0,
                }],
                texture_id: load_texture(renderer, "Turret Base Enemy Texture", Path::new("images/V2EnemyBase.png")),
                extra_texture_ids: vec![load_texture(renderer, "Turret Top Enemy Texture", Path::new("images/V2EnemyCannon.png"))],
            },
        },
    ]
}

pub fn update_waves(waves: &mut Vec<Wave>, player: &Player, enemies: &mut Vec<Enemy>, dt: f32) {
    for wave in waves.iter_mut() {
        while wave.time > wave.interval {
            let mut amount = 1;
            while rand::thread_rng().gen_range(0.0..1.0)
                < wave.double_spawn_chance / (amount * amount) as f32
            {
                amount += 1;
            }
            wave.double_spawn_chance = f32::min(
                wave.double_spawn_chance + 0.05,
                wave.max_double_spawn_chance,
            );
            wave.interval = f32::max(wave.interval + wave.interval_delta, wave.min_interval);

            for _ in 0..amount {
                let mut enemy = wave.enemy.clone();
                enemy.pos = player.pos
                    + angletovector(
                        rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                    ) * 2000.0;
                enemy.dir = angletovector(
                    rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                );
                enemies.push(enemy);
            }
            wave.time -= wave.interval;
        }
        wave.time += dt;
    }
}
