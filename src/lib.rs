use std::f32::consts::PI;
use rand::{
    prelude::*, distributions::Standard
};

pub const PLAYER_SIZE: (f32, f32) = (112., 75.);

pub const PLAYER_TURN_SPEED: f32 = 5.0 * 2.0 * PI / 360.0;
pub const PLAYER_ACCELERATION: f32 = 0.2;
pub const PLAYER_DECELERATION: f32 = 0.01;
pub const PLAYER_MAX_SPEED: f32 = 7.0;
pub const SPRITE_SCALE: f32 = 0.5;

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

