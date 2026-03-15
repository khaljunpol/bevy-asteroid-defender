use bevy::prelude::*;
use lib::{
    PLAYER_START_HP, PLAYER_SHOOT_COOLDOWN, PLAYER_MAX_SPEED, PLAYER_TURN_SPEED,
    BASE_LEVEL_ASTEROID_COUNT, ASTEROID_COUNT_PER_LEVEL, MAX_ASTEROIDS_PER_LEVEL,
    BASE_ASTEROID_HP, ASTEROID_HP_SCALING_INTERVAL,
    RAPID_FIRE_COOLDOWN_MULT, AFTERBURNER_SPEED_BONUS_PER_LEVEL,
    QUICK_REFLEXES_TURN_BONUS_PER_LEVEL, CHAIN_REACTION_COOLDOWN_MULT,
    COUNTDOWN_TICK_SECS, COUNTDOWN_GO_SECS,
    ACCELERATOR_SPEED_BONUS, ACCELERATOR_RANGE_PENALTY,
    LONG_SHOT_RANGE_BONUS, LONG_SHOT_SPEED_PENALTY,
    PIERCING_ROUNDS_SPEED_PENALTY, GLASS_CANNON_COOLDOWN_MULT, DETONATOR_RANGE_MULT,
    ATTACK_SHIP_PROJ_SPEED_MULT, ATTACK_SHIP_COOLDOWN_MULT, ATTACK_SHIP_RANGE_MULT,
    SHIELD_SHIP_PROJ_SPEED_MULT, SHIELD_SHIP_COOLDOWN_MULT, SHIELD_SHIP_RANGE_MULT,
    PROJECTILE_BASE_RANGE, BOLT_PROJ_SPEED_MULT,
    PROJECTILE_SPEED, ShipType,
};

// ── Asset path constants ──────────────────────────────────────────────────────
pub const SHIP_NORMAL_SPRITE:  &str = "sprites/ships/playerShip1_blue.png";
pub const SHIP_ATTACK_SPRITE:  &str = "sprites/ships/playerShip1_red.png";
pub const SHIP_SHIELD_SPRITE:  &str = "sprites/ships/playerShip1_green.png";

// HP restore (tiered: Blue=standard, Green=enhanced, Red=rare)
pub const POWERUP_HP_SPRITE:          &str = "sprites/powerup/powerupBlue_star.png";
pub const POWERUP_HP_SPRITE_GREEN:    &str = "sprites/powerup/powerupGreen_star.png";
pub const POWERUP_HP_SPRITE_RED:      &str = "sprites/powerup/powerupRed_star.png";
// Speed bolt
pub const POWERUP_BOLT_SPRITE:        &str = "sprites/powerup/powerupBlue_bolt.png";
pub const POWERUP_BOLT_SPRITE_GREEN:  &str = "sprites/powerup/powerupGreen_bolt.png";
pub const POWERUP_BOLT_SPRITE_RED:    &str = "sprites/powerup/powerupRed_bolt.png";
// Shield
pub const POWERUP_SHIELD_SPRITE:      &str = "sprites/powerup/powerupBlue_shield.png";
pub const POWERUP_SHIELD_SPRITE_GREEN:&str = "sprites/powerup/powerupGreen_shield.png";
pub const POWERUP_SHIELD_SPRITE_RED:  &str = "sprites/powerup/powerupRed_shield.png";
// Shield visual effect
pub const SHIELD_EFFECT_SPRITE:       &str = "sprites/effects/shield1.png";

pub const PROJECTILE_NORMAL_SPRITE: &str = "sprites/laser/laserBlue01.png";
pub const PROJECTILE_ATTACK_SPRITE: &str = "sprites/laser/laserRed01.png";
pub const PROJECTILE_SHIELD_SPRITE: &str = "sprites/laser/laserGreen01.png";

pub const LIFE_NORMAL_SPRITE: &str = "sprites/ui/playerLife1_blue.png";
pub const LIFE_ATTACK_SPRITE: &str = "sprites/ui/playerLife1_red.png";
pub const LIFE_SHIELD_SPRITE: &str = "sprites/ui/playerLife1_green.png";

pub const METEOR_BIG_SPRITE: &str = "sprites/meteor/meteorGrey_big1.png";
pub const METEOR_MED_SPRITE: &str = "sprites/meteor/meteorGrey_med1.png";
pub const METEOR_SML_SPRITE: &str = "sprites/meteor/meteorGrey_small1.png";

pub const STAR1_SPRITE: &str = "sprites/effects/star1.png";
pub const STAR2_SPRITE: &str = "sprites/effects/star2.png";
pub const STAR3_SPRITE: &str = "sprites/effects/star3.png";
pub const SPEED_SPRITE: &str = "sprites/effects/speed.png";
pub const UFO_SPRITE:        &str = "sprites/ufo/ufoRed.png";
pub const UFO_BLUE_SPRITE:   &str = "sprites/ufo/ufoBlue.png";
pub const UFO_GREEN_SPRITE:  &str = "sprites/ufo/ufoGreen.png";
pub const UFO_YELLOW_SPRITE: &str = "sprites/ufo/ufoYellow.png";

// ── Preloaded sprite handles ──────────────────────────────────────────────────
#[derive(Resource)]
pub struct GameSprites {
    pub ship_type_normal:  Handle<Image>,
    pub ship_type_attack:  Handle<Image>,
    pub ship_type_shield:  Handle<Image>,
    // HP powerup tiers
    pub powerup_hp:            Handle<Image>,
    pub powerup_hp_green:      Handle<Image>,
    pub powerup_hp_red:        Handle<Image>,
    // Bolt powerup tiers
    pub powerup_bolt:          Handle<Image>,
    pub powerup_bolt_green:    Handle<Image>,
    pub powerup_bolt_red:      Handle<Image>,
    // Shield powerup tiers
    pub powerup_shield:        Handle<Image>,
    pub powerup_shield_green:  Handle<Image>,
    pub powerup_shield_red:    Handle<Image>,
    // Shield visual effect
    pub shield_effect:         Handle<Image>,
    pub projectile_normal: Handle<Image>,
    pub projectile_attack: Handle<Image>,
    pub projectile_shield: Handle<Image>,
    pub life_normal:       Handle<Image>,
    pub life_attack:       Handle<Image>,
    pub life_shield:       Handle<Image>,
    pub meteor_big:        Handle<Image>,
    pub meteor_med:        Handle<Image>,
    pub meteor_sml:        Handle<Image>,
    // Effects
    pub star1:             Handle<Image>,
    pub star2:             Handle<Image>,
    pub star3:             Handle<Image>,
    pub fire_frames:       Vec<Handle<Image>>,
    pub speed:             Handle<Image>,
    // Enemies
    pub ufo:               Handle<Image>,
    pub ufo_blue:          Handle<Image>,
    pub ufo_green:         Handle<Image>,
    pub ufo_yellow:        Handle<Image>,
    // Font
    pub font:              Handle<Font>,
}

// ── Camera shake ─────────────────────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct CameraShake {
    pub intensity: f32,
}

impl CameraShake {
    pub fn trigger(&mut self, intensity: f32) {
        self.intensity = (self.intensity + intensity).min(20.0);
    }
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
    pub split_shot:       u32,  // 0-3
    pub rear_guard:       u32,  // 0-2  backward/omni shot
    pub rapid_fire:       u32,  // 0-3
    pub heavy_rounds:     u32,  // 0-2
    pub ricochet:         bool,
    pub accelerator:      u32,  // 0-2  faster bullets, shorter range
    pub piercing_rounds:  u32,  // 0-2  bullets pierce through asteroids
    pub explosive_rounds: bool, //       on kill: spawn 4 shrapnel fragments
    // Defense
    pub extra_armor:      u32,  // 0-2
    pub afterburner:      u32,  // 0-2
    pub quick_reflexes:   u32,  // 0-2
    pub long_shot:        u32,  // 0-2  longer range, slower bullet
    pub bulwark:          bool, //       35% chance to heal 1 HP on large kill
    // Special
    pub overclock:        bool,
    pub chain_reaction:   bool,
    pub asteroid_magnet:  bool,
    pub glass_cannon:     bool, //       double damage+speed, -1 max HP, slower fire
    pub detonator_rounds: bool, //       bullets explode at max range
    // Runtime (managed by systems, not upgrades)
    pub chain_active:     bool,
    pub chain_timer:      f32,
    // Ship-type bonuses applied at spawn
    pub shield_speed_penalty: bool,
}

impl PlayerUpgrades {
    pub fn reset(&mut self) {
        *self = PlayerUpgrades::default();
    }

    pub fn effective_shoot_cooldown(&self, ship_type: ShipType) -> f32 {
        let ship_mult = match ship_type {
            ShipType::Attack => ATTACK_SHIP_COOLDOWN_MULT,
            ShipType::Shield => SHIELD_SHIP_COOLDOWN_MULT,
            ShipType::Normal => 1.0,
        };
        let rapid_mult = RAPID_FIRE_COOLDOWN_MULT.powi(self.rapid_fire as i32);
        let chain_mult = if self.chain_active { CHAIN_REACTION_COOLDOWN_MULT } else { 1.0 };
        let cannon_mult = if self.glass_cannon { GLASS_CANNON_COOLDOWN_MULT } else { 1.0 };
        PLAYER_SHOOT_COOLDOWN * ship_mult * rapid_mult * chain_mult * cannon_mult
    }

    pub fn effective_max_speed(&self) -> f32 {
        let base = if self.shield_speed_penalty { PLAYER_MAX_SPEED * 0.85 } else { PLAYER_MAX_SPEED };
        base * (1.0 + self.afterburner as f32 * AFTERBURNER_SPEED_BONUS_PER_LEVEL)
    }

    pub fn effective_turn_speed(&self) -> f32 {
        PLAYER_TURN_SPEED * (1.0 + self.quick_reflexes as f32 * QUICK_REFLEXES_TURN_BONUS_PER_LEVEL)
    }

    pub fn bullet_damage(&self) -> i32 {
        let base = 1 + self.heavy_rounds as i32;
        if self.glass_cannon { base * 2 } else { base }
    }

    /// How many times a projectile can pierce before being consumed.
    pub fn pierce_count(&self) -> i32 {
        self.piercing_rounds as i32
    }

    /// Effective projectile speed considering ship type, upgrades, and bolt buff.
    pub fn effective_projectile_speed(&self, ship_type: ShipType, bolt_active: bool) -> f32 {
        let ship_mult = match ship_type {
            ShipType::Attack => ATTACK_SHIP_PROJ_SPEED_MULT,
            ShipType::Shield => SHIELD_SHIP_PROJ_SPEED_MULT,
            ShipType::Normal => 1.0,
        };
        let upg_mult = 1.0
            + self.accelerator as f32 * ACCELERATOR_SPEED_BONUS
            - self.long_shot   as f32 * LONG_SHOT_SPEED_PENALTY
            - self.piercing_rounds as f32 * PIERCING_ROUNDS_SPEED_PENALTY
            + if self.glass_cannon { 0.50 } else { 0.0 };
        let bolt_mult = if bolt_active { BOLT_PROJ_SPEED_MULT } else { 1.0 };
        (PROJECTILE_SPEED * ship_mult * upg_mult * bolt_mult).max(3.0)
    }

    /// Effective projectile range in world units.
    pub fn effective_projectile_range(&self, ship_type: ShipType) -> f32 {
        let ship_mult = match ship_type {
            ShipType::Attack => ATTACK_SHIP_RANGE_MULT,
            ShipType::Shield => SHIELD_SHIP_RANGE_MULT,
            ShipType::Normal => 1.0,
        };
        let upg_mult = 1.0
            + self.long_shot  as f32 * LONG_SHOT_RANGE_BONUS
            - self.accelerator as f32 * ACCELERATOR_RANGE_PENALTY;
        let detonator_mult = if self.detonator_rounds { DETONATOR_RANGE_MULT } else { 1.0 };
        (PROJECTILE_BASE_RANGE * ship_mult * upg_mult * detonator_mult).max(200.0)
    }

    /// Angle offsets (radians) for multi-shot patterns.
    pub fn shot_offsets(&self) -> Vec<f32> {
        use std::f32::consts::PI;
        let d = PI / 180.0;

        // Forward spread pattern (split shot)
        let mut offsets: Vec<f32> = match self.split_shot {
            0 => vec![0.0],
            1 => vec![-12.0 * d, 12.0 * d],
            2 => vec![-20.0 * d, 0.0, 20.0 * d],
            _ => vec![-28.0 * d, -14.0 * d, 0.0, 14.0 * d, 28.0 * d],
        };

        // Rear guard adds shots behind and around the ship
        match self.rear_guard {
            0 => {}
            1 => {
                offsets.push(PI);              // one backward shot
            }
            _ => {
                offsets.push(-PI / 2.0);       // right
                offsets.push(PI);              // backward
                offsets.push(PI / 2.0);        // left
            }
        }

        offsets
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

// ── Ship selection state ──────────────────────────────────────────────────────
/// Which ship (0=Normal, 1=Shield, 2=Attack) the player has highlighted or confirmed.
#[derive(Resource, Default)]
pub struct ShipSelectState {
    pub selected: usize, // 0=Normal, 1=Shield, 2=Attack
}

impl ShipSelectState {
    pub fn ship_type(&self) -> lib::ShipType {
        match self.selected {
            1 => lib::ShipType::Shield,
            2 => lib::ShipType::Attack,
            _ => lib::ShipType::Normal,
        }
    }
}

// ── Player buff state ─────────────────────────────────────────────────────────
/// Active timed powerup buffs.
#[derive(Resource, Default)]
pub struct PlayerBuff {
    pub bolt_timer:   f32, // speed boost seconds remaining
    pub shield_timer: f32, // invincibility seconds remaining
}

pub fn reset_player_buff(mut buff: ResMut<PlayerBuff>) {
    *buff = PlayerBuff::default();
}

// ── Pause state ───────────────────────────────────────────────────────────────
#[derive(Resource, Default)]
pub struct IsPaused(pub bool);

pub fn reset_paused(mut paused: ResMut<IsPaused>) {
    paused.0 = false;
}
