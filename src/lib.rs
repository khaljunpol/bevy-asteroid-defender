use std::f32::consts::PI;
use bevy::prelude::*;
use rand::{
    prelude::*, distributions::Standard
};

pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const POWER_UP_SIZE: Vec2 = Vec2::new(33.0, 33.0);
pub const PROJECTILE_SIZE: Vec2 = Vec2::new(9., 54.);

pub const PLAYER_TURN_SPEED: f32 = 8.0 * PI / 360.0;
pub const PLAYER_ACCELERATION: f32 = 0.1;
pub const PLAYER_DECELERATION: f32 = 0.01;
pub const PLAYER_MAX_SPEED: f32 = 4.0;
pub const PLAYER_SHOOT_COOLDOWN: f32 = 0.5;

pub const BORDER_EXTRA_SPACE: f32 = 100.0;

pub const SPRITE_SCALE: f32 = 0.5;

pub const POWERUP_SPAWN_TIME: f32 = 2.0;
pub const POWERUP_MAX_COUNT: i32 = 3;

pub const PROJECTILE_DESPAWN_TIME: f32 = 3.0;
pub const PROJECTILE_SPEED: f32 = 10.0;

// Ship Settings
pub const DEFAULT_STATS: [(ShipType, Stats); 3] = [
    (ShipType::Normal, Stats {
        hp: 75.0,
        shield: 75.0,
        power: 50.0,
        attack_cooldown: 1.0,
    }),
    (ShipType::Shield, Stats {
        hp: 100.0,
        shield: 100.0,
        power: 25.0,
        attack_cooldown: 1.0,
    }),
    (ShipType::Attack, Stats {
        hp: 50.0,
        shield: 50.0,
        power: 100.0,
        attack_cooldown: 0.5,
    }),
];

#[derive(Copy, Clone)]
pub enum ShipType {
    Normal,
    Shield,
    Attack
}

#[derive(Copy, Clone)]
pub struct Stats {
    pub hp: f32,
    pub shield: f32,
    pub power: f32,
    pub attack_cooldown: f32
}

impl Distribution<ShipType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShipType {
        match rng.gen_range(0..3) {
            0 => ShipType::Attack,
            1 => ShipType::Shield,
            _ => ShipType::Normal,
        }
    }
}

// Power Settings
#[derive(Copy, Clone)]
pub enum PowerUpType {
    ChangeShipType
}

impl Distribution<PowerUpType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpType {
        match rng.gen_range(0..1) {
            0 => PowerUpType::ChangeShipType,
            _ => PowerUpType::ChangeShipType,
        }
    }
}