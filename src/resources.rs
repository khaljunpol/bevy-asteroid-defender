use bevy::prelude::*;
use lib::{
    PLAYER_START_HP, PLAYER_SHOOT_COOLDOWN, PLAYER_MAX_SPEED, PLAYER_TURN_SPEED,
    BASE_LEVEL_ASTEROID_COUNT, ASTEROID_COUNT_PER_LEVEL, MAX_ASTEROIDS_PER_LEVEL,
    BASE_ASTEROID_HP, ASTEROID_HP_SCALING_INTERVAL,
    RAPID_FIRE_COOLDOWN_MULT, AFTERBURNER_SPEED_BONUS_PER_LEVEL,
    QUICK_REFLEXES_TURN_BONUS_PER_LEVEL, CHAIN_REACTION_COOLDOWN_MULT,
    COUNTDOWN_TICK_SECS, COUNTDOWN_GO_SECS,
};

// ── Asset path constants ──────────────────────────────────────────────────────
pub const SHIP_NORMAL_SPRITE:  &str = "sprites/ships/playerShip1_blue.png";
pub const SHIP_ATTACK_SPRITE:  &str = "sprites/ships/playerShip1_red.png";
pub const SHIP_SHIELD_SPRITE:  &str = "sprites/ships/playerShip1_green.png";

pub const POWERUP_HP_SPRITE:   &str = "sprites/powerup/powerupBlue_star.png";

pub const PROJECTILE_NORMAL_SPRITE: &str = "sprites/laser/laserBlue01.png";
pub const PROJECTILE_ATTACK_SPRITE: &str = "sprites/laser/laserRed01.png";
pub const PROJECTILE_SHIELD_SPRITE: &str = "sprites/laser/laserGreen01.png";

pub const LIFE_NORMAL_SPRITE: &str = "sprites/ui/playerLife1_blue.png";
pub const LIFE_ATTACK_SPRITE: &str = "sprites/ui/playerLife1_red.png";
pub const LIFE_SHIELD_SPRITE: &str = "sprites/ui/playerLife1_green.png";

pub const METEOR_BIG_SPRITE: &str = "sprites/meteor/meteorGrey_big1.png";
pub const METEOR_MED_SPRITE: &str = "sprites/meteor/meteorGrey_med1.png";
pub const METEOR_SML_SPRITE: &str = "sprites/meteor/meteorGrey_small1.png";

// ── Preloaded sprite handles ──────────────────────────────────────────────────
#[derive(Resource)]
pub struct GameSprites {
    pub ship_type_normal: Handle<Image>,
    pub ship_type_attack:  Handle<Image>,
    pub ship_type_shield:  Handle<Image>,
    pub powerup_hp:        Handle<Image>,
    pub projectile_normal: Handle<Image>,
    pub projectile_attack: Handle<Image>,
    pub projectile_shield: Handle<Image>,
    pub life_normal:       Handle<Image>,
    pub life_attack:       Handle<Image>,
    pub life_shield:       Handle<Image>,
    pub meteor_big:        Handle<Image>,
    pub meteor_med:        Handle<Image>,
    pub meteor_sml:        Handle<Image>,
}

// ── Window helpers ────────────────────────────────────────────────────────────
#[derive(Resource)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct WindowDespawnBorder {
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
}

// ── Core game stats ───────────────────────────────────────────────────────────
#[derive(Resource)]
pub struct Score {
    pub current:    i32,
    pub high_score: i32,
}

impl Score {
    pub fn new(score: i32) -> Self {
        Score { current: score, high_score: 0 }
    }

    pub fn reset(&mut self) {
        self.high_score = self.high_score.max(self.current);
        self.current = 0;
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}

#[derive(Resource)]
pub struct Life {
    pub max_life:     i32,
    pub current_life: i32,
}

impl Life {
    pub fn new(life: i32) -> Self {
        Life { max_life: life, current_life: life }
    }

    pub fn reset(&mut self) {
        self.max_life = PLAYER_START_HP;
        self.current_life = PLAYER_START_HP;
    }
}

pub fn reset_life(mut life: ResMut<Life>) {
    life.reset();
}

// ── Level ─────────────────────────────────────────────────────────────────────
#[derive(Resource)]
pub struct LevelResource {
    /// Current level number (starts at 1).
    pub current: u32,
    /// Set to the number of asteroids spawned when a level begins.
    /// Used to prevent an instant level-complete on the first frame.
    pub total_asteroids_spawned: u32,
}

impl LevelResource {
    pub fn new() -> Self {
        LevelResource { current: 1, total_asteroids_spawned: 0 }
    }

    pub fn asteroids_for_level(&self) -> u32 {
        (BASE_LEVEL_ASTEROID_COUNT + (self.current - 1) * ASTEROID_COUNT_PER_LEVEL)
            .min(MAX_ASTEROIDS_PER_LEVEL)
    }

    /// HP for large asteroids that are pre-spawned at level start.
    pub fn asteroid_hp_for_level(&self) -> i32 {
        BASE_ASTEROID_HP + ((self.current - 1) / ASTEROID_HP_SCALING_INTERVAL) as i32
    }

    pub fn advance(&mut self) {
        self.current += 1;
        self.total_asteroids_spawned = 0;
    }

    pub fn reset(&mut self) {
        self.current = 1;
        self.total_asteroids_spawned = 0;
    }
}

pub fn reset_level(mut level: ResMut<LevelResource>) {
    level.reset();
}

// ── Countdown ─────────────────────────────────────────────────────────────────
#[derive(Resource)]
pub struct CountdownResource {
    /// 3 → 2 → 1 → 0 ("GO!"). Transitions to InGame when this drops below 0.
    pub count:      i32,
    pub tick_timer: Timer,
    pub go_timer:   Timer,
}

impl CountdownResource {
    pub fn new() -> Self {
        CountdownResource {
            count:      3,
            tick_timer: Timer::from_seconds(COUNTDOWN_TICK_SECS, TimerMode::Once),
            go_timer:   Timer::from_seconds(COUNTDOWN_GO_SECS,   TimerMode::Once),
        }
    }

    pub fn reset(&mut self) {
        self.count = 3;
        self.tick_timer.reset();
        self.go_timer.reset();
    }
}

// ── Player upgrades ───────────────────────────────────────────────────────────
#[derive(Resource, Default, Clone)]
pub struct PlayerUpgrades {
    // Offense
    pub split_shot:    u32,  // 0-3 levels
    pub rapid_fire:    u32,  // 0-3 levels
    pub heavy_rounds:  u32,  // 0-2 levels
    pub ricochet:      bool,
    // Defense
    pub extra_armor:      u32,  // 0-2 levels (+1 max HP each)
    pub afterburner:      u32,  // 0-2 levels
    pub quick_reflexes:   u32,  // 0-2 levels
    // Special
    pub overclock:       bool,
    pub chain_reaction:  bool,
    pub asteroid_magnet: bool,
    // Runtime (not an upgrade, managed by systems)
    pub chain_active:   bool,
    pub chain_timer:    f32,
}

impl PlayerUpgrades {
    pub fn reset(&mut self) {
        *self = PlayerUpgrades::default();
    }

    pub fn effective_shoot_cooldown(&self) -> f32 {
        let rapid_mult = RAPID_FIRE_COOLDOWN_MULT.powi(self.rapid_fire as i32);
        let chain_mult = if self.chain_active { CHAIN_REACTION_COOLDOWN_MULT } else { 1.0 };
        PLAYER_SHOOT_COOLDOWN * rapid_mult * chain_mult
    }

    pub fn effective_max_speed(&self) -> f32 {
        PLAYER_MAX_SPEED * (1.0 + self.afterburner as f32 * AFTERBURNER_SPEED_BONUS_PER_LEVEL)
    }

    pub fn effective_turn_speed(&self) -> f32 {
        PLAYER_TURN_SPEED * (1.0 + self.quick_reflexes as f32 * QUICK_REFLEXES_TURN_BONUS_PER_LEVEL)
    }

    pub fn bullet_damage(&self) -> i32 {
        1 + self.heavy_rounds as i32
    }

    /// Angle offsets (radians) for multi-shot patterns.
    pub fn shot_offsets(&self) -> Vec<f32> {
        use std::f32::consts::PI;
        let d = PI / 180.0;
        match self.split_shot {
            0 => vec![0.0],
            1 => vec![-20.0 * d, 0.0, 20.0 * d],
            2 => vec![-30.0 * d, -15.0 * d, 0.0, 15.0 * d, 30.0 * d],
            _ => vec![-36.0 * d, -24.0 * d, -12.0 * d, 0.0, 12.0 * d, 24.0 * d, 36.0 * d],
        }
    }
}

pub fn reset_upgrades(mut upgrades: ResMut<PlayerUpgrades>) {
    upgrades.reset();
}

// ── Upgrade selection state ───────────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct UpgradeSelectionState {
    pub choices:  Vec<crate::upgrades::upgrades::UpgradeType>,
    pub selected: usize,
}
