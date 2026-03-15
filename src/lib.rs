use std::f32::consts::PI;
use bevy::prelude::*;
use rand::{prelude::*, distributions::Standard};

// ── Frame / window ──────────────────────────────────────────────────────────
pub const MAX_FRAMERATE: f64 = 60.0;
pub const BORDER_EXTRA_SPACE: f32 = 100.0;
pub const SPRITE_SCALE: f32 = 0.5;

// ── Player ───────────────────────────────────────────────────────────────────
pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const PLAYER_TURN_SPEED: f32 = 8.0 * PI / 360.0;
pub const PLAYER_ACCELERATION: f32 = 0.15;
pub const PLAYER_DECELERATION: f32 = 0.01;
pub const PLAYER_MAX_SPEED: f32 = 5.0;
pub const PLAYER_SHOOT_COOLDOWN: f32 = 0.18;
pub const PLAYER_START_HP: i32 = 3;
pub const PLAYER_START_SCORE: i32 = 0;
pub const PLAYER_HP_ADD: i32 = 1;

// ── Projectile ───────────────────────────────────────────────────────────────
pub const PROJECTILE_SIZE: Vec2 = Vec2::new(9., 54.);
pub const PROJECTILE_DESPAWN_TIME: f32 = 3.5;
pub const PROJECTILE_SPEED: f32 = 10.0;

// ── Meteor / asteroid ────────────────────────────────────────────────────────
pub const METEOR_SIZE: [(MeteorSizeType, Vec2); 3] = [
    (MeteorSizeType::Large,  Vec2::new(120.0, 98.0)),
    (MeteorSizeType::Medium, Vec2::new(43.0,  43.0)),
    (MeteorSizeType::Small,  Vec2::new(28.0,  28.0)),
];

pub const METEOR_DMG: [(MeteorSizeType, i32); 3] = [
    (MeteorSizeType::Small,  1),
    (MeteorSizeType::Medium, 2),
    (MeteorSizeType::Large,  3),
];

pub const METEOR_SCORE: [(MeteorSizeType, i32); 3] = [
    (MeteorSizeType::Small,  10),
    (MeteorSizeType::Medium, 25),
    (MeteorSizeType::Large,  50),
];

// ── Level scaling ────────────────────────────────────────────────────────────
/// Asteroids spawned on level 1.
pub const BASE_LEVEL_ASTEROID_COUNT: u32 = 4;
/// Extra large asteroid added per level (on top of base).
pub const ASTEROID_COUNT_PER_LEVEL: u32 = 1;
/// Hard cap on asteroids per level so the screen doesn't overflow.
pub const MAX_ASTEROIDS_PER_LEVEL: u32 = 12;
/// Base HP for large asteroids spawned at the start of a level.
pub const BASE_ASTEROID_HP: i32 = 1;
/// Every this many levels, large asteroids gain +1 HP.
pub const ASTEROID_HP_SCALING_INTERVAL: u32 = 2;

// ── Countdown ────────────────────────────────────────────────────────────────
/// Duration of each numeric tick (3 → 2 → 1).
pub const COUNTDOWN_TICK_SECS: f32 = 1.0;
/// How long "GO!" is shown before the level begins.
pub const COUNTDOWN_GO_SECS: f32 = 0.75;

// ── Power-ups ────────────────────────────────────────────────────────────────
pub const POWER_UP_SIZE: Vec2 = Vec2::new(33.0, 33.0);
pub const POWERUP_SPAWN_TIME: f32 = 6.0;
pub const POWERUP_MAX_COUNT: i32 = 2;

// ── Upgrades ─────────────────────────────────────────────────────────────────
/// Cards shown in the upgrade selection screen.
pub const UPGRADE_CHOICES: usize = 3;
/// Cooldown multiplier applied per Rapid Fire level (0.78^level).
pub const RAPID_FIRE_COOLDOWN_MULT: f32 = 0.75;
/// Max-speed multiplier added per Afterburner level.
pub const AFTERBURNER_SPEED_BONUS_PER_LEVEL: f32 = 0.30;
/// Turn-speed multiplier added per Quick Reflexes level.
pub const QUICK_REFLEXES_TURN_BONUS_PER_LEVEL: f32 = 0.40;
/// Asteroid speed fraction when Overclock is active.
pub const OVERCLOCK_SPEED_MULT: f32 = 0.60;
/// Seconds of rapid-fire granted by Chain Reaction.
pub const CHAIN_REACTION_DURATION: f32 = 3.0;
/// Fire-cooldown multiplier while Chain Reaction is active (faster = smaller).
pub const CHAIN_REACTION_COOLDOWN_MULT: f32 = 0.35;
/// Speed at which Asteroid Magnet drifts powerups toward the player.
pub const MAGNET_STRENGTH: f32 = 0.012;

// ── New upgrade scaling ───────────────────────────────────────────────────────
pub const ACCELERATOR_SPEED_BONUS:     f32 = 0.28;
pub const ACCELERATOR_RANGE_PENALTY:   f32 = 0.18;
pub const LONG_SHOT_RANGE_BONUS:       f32 = 0.35;
pub const LONG_SHOT_SPEED_PENALTY:     f32 = 0.10;
pub const PIERCING_ROUNDS_SPEED_PENALTY: f32 = 0.12;
pub const GLASS_CANNON_COOLDOWN_MULT:  f32 = 1.50;
pub const BULWARK_HEAL_CHANCE:         f32 = 0.35;
/// Detonator Rounds cuts bullet range to this fraction of normal so explosions
/// reliably happen within the visible play area.
pub const DETONATOR_RANGE_MULT:        f32 = 0.45;

// ── Per-ship projectile stats ─────────────────────────────────────────────────
pub const ATTACK_SHIP_PROJ_SPEED_MULT: f32 = 1.45;
pub const ATTACK_SHIP_COOLDOWN_MULT:   f32 = 0.80;
pub const ATTACK_SHIP_RANGE_MULT:      f32 = 0.65;
pub const SHIELD_SHIP_PROJ_SPEED_MULT: f32 = 0.80;
pub const SHIELD_SHIP_COOLDOWN_MULT:   f32 = 1.30;
pub const SHIELD_SHIP_RANGE_MULT:      f32 = 1.60;
/// Base projectile travel range in world units before despawn.
/// Screen is 720px tall; 850 ≈ just over one full screen-height.
/// Attack ship (×0.65) → ~550px   Normal → ~850px   Shield (×1.60) → ~1360px
pub const PROJECTILE_BASE_RANGE:       f32 = 850.0;
/// Bolt powerup: multiplies projectile speed (not movement speed).
pub const BOLT_PROJ_SPEED_MULT:        f32 = 1.65;

// ── Ship types ───────────────────────────────────────────────────────────────
pub const DEFAULT_STATS: [(ShipType, Stats); 3] = [
    (ShipType::Normal, Stats { shield: 75.0, power: 50.0, attack_cooldown: 1.0 }),
    (ShipType::Shield, Stats { shield: 100.0, power: 25.0, attack_cooldown: 1.0 }),
    (ShipType::Attack, Stats { shield: 50.0, power: 100.0, attack_cooldown: 0.5 }),
];

// ── Types ─────────────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default, Reflect, PartialEq)]
pub enum MeteorSizeType {
    Small  = 1,
    Medium = 2,
    #[default]
    Large  = 3,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ShipType {
    Normal,
    Shield,
    Attack,
}

#[derive(Copy, Clone)]
pub struct Stats {
    pub shield: f32,
    pub power: f32,
    pub attack_cooldown: f32,
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

/// Returns the hit-box size for a given meteor size.
pub fn meteor_size(size: MeteorSizeType) -> Vec2 {
    for (st, s) in METEOR_SIZE {
        if st == size {
            return s;
        }
    }
    METEOR_SIZE[0].1
}

/// Returns contact damage for a given meteor size.
pub fn meteor_damage(size: MeteorSizeType) -> i32 {
    for (st, d) in METEOR_DMG {
        if st == size {
            return d;
        }
    }
    METEOR_DMG[0].1
}

/// Returns score awarded for destroying a given meteor size.
pub fn meteor_score(size: MeteorSizeType) -> i32 {
    for (st, s) in METEOR_SCORE {
        if st == size {
            return s;
        }
    }
    METEOR_SCORE[0].1
}
