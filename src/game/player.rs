use std::path::Path;

use crate::renderer::{self, texture::TextureId, Renderer, Rendering2D};

use super::{
    angletovector, load_texture, rotatevector, vectortoangle, Bullet, BulletEmitter, Damage, DamageType, Enemy, Part, PartMod, Partical, ParticalEmitter, ParticalShape, Player
};
use cgmath::{InnerSpace, Vector2, Vector4, Zero};
use rand::Rng;
use inputbot::KeybdKey::*;

pub fn init_player(renderer:&mut Renderer) -> Player {
    Player {
        pos: Vector2 { x: 50.0, y: 50.0 },
        vel: Vector2 { x: 1.0, y: 0.0 },
        dir: Vector2 { x: 0.0, y: 1.0 },
        speed_original: 250.0,
        left_turn_original: 100.0,
        right_turn_original: 100.0,
        parts: vec![
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 12.0, y: -13.0 },
                health: 4.0,
                starting_health: 4.0,
                size: 17.0,
                name: "Left Engine".to_string(),
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: -12.0, y: -13.0 },
                health: 4.0,
                starting_health: 4.0,
                size: 17.0,
                name: "Right Engine".to_string(),
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 0.0, y: 15.0 },
                health: 3.0,
                starting_health: 3.0,
                size: 20.0,
                name: "Main Body".to_string(),
            },
        ],
        damage: vec![
            Damage {
                src: vec![1],
                des: PartMod::TurnLeft,
                index: 0,
                damage_type: DamageType::Mult,
                scale: 1.0,
            },
            Damage {
                src: vec![0],
                des: PartMod::TurnRight,
                index: 0,
                damage_type: DamageType::Mult,
                scale: 1.0,
            },
            Damage {
                src: vec![0, 1, 2],
                des: PartMod::Speed,
                index: 0,
                damage_type: DamageType::Mult,
                scale: 1.0,
            },
            Damage {
                src: vec![0],
                des: PartMod::Partical,
                index: 0,
                damage_type: DamageType::Mult,
                scale: 1.0,
            },
            Damage {
                src: vec![1],
                des: PartMod::Partical,
                index: 1,
                damage_type: DamageType::Mult,
                scale: 1.0,
            },
        ],
        partical_emmiters: vec![
            ParticalEmitter {
                pos: Vector2::zero(),
                location: Vector2::new(21.0, -26.0),
                vel: Vector2::zero(),
                speed_orginal: 200.0,
                size: 5.0,
                shape: ParticalShape::Square,
                starting_color: Vector4 {
                    x: 140.0 / 255.0,
                    y: 255.0 / 255.0,
                    z: 251.0 / 255.0,
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
            },
            ParticalEmitter {
                pos: Vector2::zero(),
                location: Vector2::new(-21.0, -26.0),
                vel: Vector2::zero(),
                speed_orginal: 200.0,
                size: 5.0,
                shape: ParticalShape::Square,
                starting_color: Vector4 {
                    x: 140.0 / 255.0,
                    y: 255.0 / 255.0,
                    z: 251.0 / 255.0,
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
            },
        ],
        bullet_emmiters: vec![
            BulletEmitter {
                pos: Vector2::zero(),
                location: Vector2 { x: 17.0, y: 13.0 },
                size: 5.0,
                damage: 2.0,
                friendly: true,
                duration: 2.0,
                bullet_interval: 1.0 / 7.5,
                time: 0.0,
            },
            BulletEmitter {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: 13.0 },
                size: 5.0,
                damage: 2.0,
                friendly: true,
                duration: 2.0,
                bullet_interval: 1.0 / 5.0,
                time: 1.0 / 7.5 / 2.0,
            },
        ],
        speed: 0.0,
        left_turn: 0.0,
        right_turn: 0.0,
        texture_id: load_texture(renderer, "Player Texture", Path::new("images/V1Ship.png")),
    }
}

pub fn update_player(
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    particals: &mut Vec<Partical>,
   // rl: &RaylibHandle,
    dt: f32,
) {
    player.left_turn = player.left_turn_original;
    player.right_turn = player.right_turn_original;
    player.speed = player.speed_original;

    for partical_emmiter in &mut player.partical_emmiters {
        partical_emmiter.speed = partical_emmiter.speed_orginal
    }

    let mut fire: bool = false;
    for enemy in enemies {
        for partical_emmiter in &mut enemy.partical_emmiters {
            partical_emmiter.speed = partical_emmiter.speed_orginal
        }
        if ((enemy.pos - player.pos).normalize().dot(player.dir) - 1.0).abs() < 0.25 {
            fire = true
        }
    }

    for damage in &player.damage {
        let mut health = 0.0;
        let mut total_health = 0.0;
        for src in &damage.src {
            health += player.parts[*src].health;
            total_health += player.parts[*src].starting_health;
        }
        let value = health / total_health;
        match damage.damage_type {
            DamageType::Mult => match damage.des {
                PartMod::Partical => {
                    player.partical_emmiters[damage.index].speed *= value * damage.scale
                }
                PartMod::Gun => {
                    player.bullet_emmiters[damage.index].bullet_interval *= value * damage.scale
                }
                PartMod::TurnLeft => player.left_turn *= value * damage.scale,
                PartMod::TurnRight => player.right_turn *= value * damage.scale,
                PartMod::Speed => player.speed *= value * damage.scale,
            },
            DamageType::Div => match damage.des {
                PartMod::Partical => {
                    player.partical_emmiters[damage.index].speed /= value * damage.scale
                }
                PartMod::Gun => {
                    player.bullet_emmiters[damage.index].bullet_interval /= value * damage.scale
                }
                PartMod::TurnLeft => player.left_turn /= value * damage.scale,
                PartMod::TurnRight => player.right_turn /= value * damage.scale,
                PartMod::Speed => player.speed /= value * damage.scale,
            },
        }
    }
    if AKey.is_pressed() {
        player.dir =
            angletovector(vectortoangle(player.dir) + (player.left_turn.to_radians() * dt));
    };
    if DKey.is_pressed() {
           player.dir =
               angletovector(vectortoangle(player.dir) - (player.right_turn.to_radians() * dt));
    };
    player.vel += player.dir.normalize()
        * (player.speed
            - (player.vel.magnitude() * (2.0 + (player.vel.normalize().dot(player.dir) - 1.0))
                / 2.0))
        * dt;
    let right = rotatevector(player.dir, std::f32::consts::PI / 2.0);
    player.vel -= right * (right.dot(player.vel)) * 1.0 * dt;
    player.pos += player.vel * dt;
    for part in &mut player.parts {
        part.pos = player.pos
            + rotatevector(
                part.location,
                vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
            )
    }
    for partical_emmiter in &mut player.partical_emmiters {
        partical_emmiter.pos = player.pos
            + rotatevector(
                partical_emmiter.location,
                vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
            );
        partical_emmiter.vel = player.vel
            + -player.dir * partical_emmiter.speed * (player.parts[1].health / 2.0)
            + angletovector(
                rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
            ) * rand::thread_rng().gen_range(20.0..40.0);
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
        partical_emmiter.time += dt;
    }

    for bullet_emmiter in &mut player.bullet_emmiters {
        bullet_emmiter.pos = player.pos
            + rotatevector(
                bullet_emmiter.location,
                vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
            );
        let vel = player.vel + player.dir * 500.0;
        while bullet_emmiter.time > bullet_emmiter.bullet_interval {
            if fire {
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
        bullet_emmiter.time += dt;
    }
}

pub fn draw_player(drawing: &mut Rendering2D<'_, '_>, player: &Player, ship_image: TextureId) {
    let ship_scale = 1.0;
    drawing.draw_quad(
        player.pos,
        Vector2 { x: 64.0 * ship_scale, y: 64.0 * ship_scale},
        Vector4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        },
        vectortoangle(player.dir).to_degrees() - 90.0,
        Some(ship_image),
    );
    //drawing.draw_texture_pro(
    //    &ship_image,
    //    Rectangle::new(0.0, 0.0, ship_image.width as f32, ship_image.height as f32),
    //    Rectangle::new(
    //        screenwidth as f32 / 2.0,
    //        screenheight as f32 / 2.0,
    //        ship_image.width as f32 * ship_scale,
    //        ship_image.height as f32 * ship_scale,
    //    ),
    //    Vector2::new(
    //        ship_image.width as f32 / 2.0 * ship_scale,
    //        ship_image.height as f32 / 2.0 * ship_scale,
    //    ),
    //    vectortoangle(player.dir).to_degrees() + 90.0,
    //    Color::WHITE,
    //);
}
