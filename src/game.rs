use crate::renderer::{texture::TextureId, FrameRendering, Renderer, Rendering2D};
use bullets::*;
use cgmath::{Vector2, Vector4, Zero};
use debug::*;
use enemy::*;
use particals::*;
use player::*;
//use powerups::*;
use rand::Rng;
use std::io::Write;
use waves::*;

mod bullets;
mod debug;
mod enemy;
mod particals;
mod player;
//mod powerups;
mod waves;

pub struct Game {
    camera_pos: Vector2<f32>,
    texture: TextureId,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    particals: Vec<Partical>,
    waves: Vec<Wave>,
    enemy_warning_image: TextureId,
}

#[derive(Clone)]
struct Player {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    dir: Vector2<f32>,
    speed_original: f32,
    left_turn_original: f32,
    right_turn_original: f32,
    parts: Vec<Part>,
    damage: Vec<Damage>,
    partical_emmiters: Vec<ParticalEmitter>,
    bullet_emmiters: Vec<BulletEmitter>,
    speed: f32,
    left_turn: f32,
    right_turn: f32,
    texture_id: TextureId,
}
#[derive(Clone)]
struct Part {
    pos: Vector2<f32>,
    location: Vector2<f32>,
    health: f32,
    starting_health: f32,
    size: f32,
    name: String,
}
#[derive(Clone)]
struct Enemy {
    name: String,
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    dir: Vector2<f32>,
    targetpos: Vector2<f32>,
    speed: f32,
    turningspeed: f32,
    predictive: bool,
    texture_scale: f32,
    friction: f32,
    size: f32,
    health: f32,
    partical_emmiters: Vec<ParticalEmitter>,
    bullet_emmiters: Vec<BulletEmitter>,
    texture_id: TextureId,
    extra_texture_ids: Vec<TextureId>,
}
#[derive(Clone)]
struct Bullet {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    size: f32,
    damage: f32,
    friendly: bool,
    duration: f32,
    time: f32,
}
#[derive(Clone)]
struct Partical {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    size: f32,
    shape: ParticalShape,
    starting_color: Vector4<f32>,
    ending_color: Vector4<f32>,
    duration: f32,
    time: f32,
}
#[derive(Clone)]
struct ParticalEmitter {
    pos: Vector2<f32>,
    location: Vector2<f32>,
    speed_orginal: f32,
    vel: Vector2<f32>,
    size: f32,
    shape: ParticalShape,
    starting_color: Vector4<f32>,
    ending_color: Vector4<f32>,
    duration: f32,
    partical_interval: f32,
    time: f32,
    speed: f32,
}

#[derive(Clone)]
struct BulletEmitter {
    pos: Vector2<f32>,
    location: Vector2<f32>,
    size: f32,
    damage: f32,
    friendly: bool,
    duration: f32,
    bullet_interval: f32,
    time: f32,
}

#[derive(Clone)]
struct Damage {
    src: Vec<usize>,
    des: PartMod,
    index: usize,
    damage_type: DamageType,
    scale: f32,
}

struct PowerUp {
    pos: Vector2<f32>,
    power_type: PowerUpType,
    texture: TextureId,
}

struct Wave {
    interval: f32,
    min_interval: f32,
    interval_delta: f32,
    double_spawn_chance: f32,
    max_double_spawn_chance: f32,
    time: f32,
    enemy: Enemy,
}

enum PowerUpType {
    Shield,
    Repair,
}

#[derive(Clone)]
enum DamageType {
    Mult,
    Div,
}

#[derive(Clone)]
enum PartMod {
    Partical,
    Gun,
    TurnLeft,
    TurnRight,
    Speed,
}

#[derive(Clone)]
enum ParticalShape {
    Square,
    Circle,
    RotSquare,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Self {
        Self {
            camera_pos: Vector2 { x: 0.0, y: 0.0 },
            texture: renderer.create_texture("Yellow", 1, 1, &[255, 255, 0, 255]),
            player: init_player(renderer),
            enemies: vec![],
            bullets: vec![],
            particals: vec![],
            waves: init_waves(renderer),
            enemy_warning_image: renderer.create_texture("Yellow", 1, 1, &[255, 255, 255, 255]),
        }
    }

    pub fn update(&mut self, dt: f32) {
        print!("\r{}", 1.0 / dt);
        std::io::stdout().flush().unwrap();
        self.camera_pos = self.player.pos;
        update_player(
            &mut self.player,
            &mut self.enemies,
            &mut self.bullets,
            &mut self.particals,
            dt,
        );
        //update_waves(&mut self.waves, &self.player, &mut self.enemies, dt);
        //update_enemies(
        //    &mut self.player,
        //    &mut self.enemies,
        //    &mut self.particals,
        //    &mut self.bullets,
        //    dt,
        //);
        //update_bullets(
        //    &mut self.player,
        //    &mut self.bullets,
        //    &mut self.enemies,
        //    &mut self.particals,
        //    dt,
        //);
        //update_particals(&mut self.particals, dt);
    }

    pub fn render(&mut self, frame: &mut FrameRendering<'_>) {
        let mut drawing = Rendering2D::new(frame, self.camera_pos, 1000.0);
        //draw_player(&mut drawing, &self.player, self.player.texture_id);
        //draw_enemies(&mut drawing, &self.player, &self.enemies, &self.enemy_warning_image);
        //draw_particals(&mut drawing, &mut self.particals);
    }
}
fn get_2_mut<T>(xs: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b || a >= xs.len() || b >= xs.len() {
        return None;
    }
    let ptr = xs.as_mut_ptr();
    unsafe { Some((&mut *ptr.add(a), &mut *ptr.add(b))) }
}

fn colorlerp(starting_color: Vector4<f32>, ending_color: Vector4<f32>, t: f32) -> Vector4<f32> {
    Vector4::new(
        starting_color.x as f32 + (ending_color.x as f32 - starting_color.x as f32) * t,
        starting_color.y as f32 + (ending_color.y as f32 - starting_color.y as f32) * t,
        starting_color.z as f32 + (ending_color.z as f32 - starting_color.z as f32) * t,
        starting_color.w as f32 + (ending_color.w as f32 - starting_color.w as f32) * t,
    )
}

fn vectortoangle(vector: Vector2<f32>) -> f32 {
    f32::atan2(vector.y, vector.x)
}

fn angletovector(angle: f32) -> Vector2<f32> {
    Vector2::new(f32::cos(angle), f32::sin(angle))
}

fn rotatevector(vector: Vector2<f32>, angle: f32) -> Vector2<f32> {
    Vector2 {
        x: vector.x * f32::cos(angle) - vector.y * f32::sin(angle),
        y: vector.x * f32::sin(angle) + vector.y * f32::cos(angle),
    }
}

fn enemy_dies(pos: Vector2<f32>, vel: Vector2<f32>, particals: &mut Vec<Partical>) {
    particalexplosion(
        particals,
        pos,
        vel,
        0.0,
        300.0,
        500,
        Vector4 {
            x: 200.0 / 255.0,
            y: 200.0 / 255.0,
            z: 50.0 / 255.0,
            w: 255.0 / 255.0,
        },
        Vector4 {
            x: 255.0 / 255.0,
            y: 0.0 / 255.0,
            z: 0.0 / 255.0,
            w: 100.0 / 255.0,
        },
        0.3,
    );
}

fn particalexplosion(
    particals: &mut Vec<Partical>,
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    force_min: f32,
    force_max: f32,
    amount: usize,
    start_color: Vector4<f32>,
    ending_color: Vector4<f32>,
    duration: f32,
) {
    for _ in 0..amount {
        particals.push(Partical {
            pos: pos,
            vel: vel
                + angletovector(
                    rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                ) * rand::thread_rng().gen_range(force_min..force_max),
            size: 5.0,
            shape: ParticalShape::Square,
            starting_color: start_color,
            ending_color: ending_color,
            duration: duration,
            time: 0.0,
        });
    }
}
