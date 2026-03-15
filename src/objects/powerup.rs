use std::time::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::prelude::*;

use lib::{POWER_UP_SIZE, POWERUP_MAX_COUNT, POWERUP_SPAWN_TIME, MAGNET_STRENGTH};
use crate::{
    common::common_components::{RotationAngle, Velocity, Position, HitBoxSize, BoundsDespawnable},
    player::player::PlayerComponent,
    resources::{GameSprites, PlayerBuff, Life, PlayerUpgrades, WindowSize},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Tier ──────────────────────────────────────────────────────────────────────

/// Powerup quality tier, shown by colour.
/// Blue = standard, Green = enhanced, Red = rare.
#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpTier {
    Standard, // blue
    Enhanced, // green
    Rare,     // red
}

// ── Kind ──────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpKind {
    Hp,
    Bolt,
    Shield,
}

// ── Component ─────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct PowerUpComponent {
    pub kind:           PowerUpKind,
    pub tier:           PowerUpTier,
    pub rotation_speed: f32,
}

impl PowerUpComponent {
    pub fn apply(&self, life: &mut Life, buff: &mut PlayerBuff) {
        match (self.kind, self.tier) {
            // ── HP restore ───────────────────────────────────────────────────
            (PowerUpKind::Hp, PowerUpTier::Standard) => {
                life.current_life = (life.current_life + 1).min(life.max_life);
            }
            (PowerUpKind::Hp, PowerUpTier::Enhanced) => {
                life.current_life = (life.current_life + 2).min(life.max_life);
            }
            (PowerUpKind::Hp, PowerUpTier::Rare) => {
                life.current_life = life.max_life; // full restore
            }

            // ── Speed bolt ───────────────────────────────────────────────────
            (PowerUpKind::Bolt, PowerUpTier::Standard) => {
                buff.bolt_timer = (buff.bolt_timer + 5.0).min(20.0);
            }
            (PowerUpKind::Bolt, PowerUpTier::Enhanced) => {
                buff.bolt_timer = (buff.bolt_timer + 9.0).min(20.0);
            }
            (PowerUpKind::Bolt, PowerUpTier::Rare) => {
                buff.bolt_timer = (buff.bolt_timer + 14.0).min(20.0);
            }

            // ── Shield ───────────────────────────────────────────────────────
            (PowerUpKind::Shield, PowerUpTier::Standard) => {
                buff.shield_timer = (buff.shield_timer + 4.0).min(20.0);
            }
            (PowerUpKind::Shield, PowerUpTier::Enhanced) => {
                buff.shield_timer = (buff.shield_timer + 7.0).min(20.0);
            }
            (PowerUpKind::Shield, PowerUpTier::Rare) => {
                buff.shield_timer = (buff.shield_timer + 11.0).min(20.0);
            }
        }
    }

    /// Display name shown in the HUD notification.
    pub fn display_name(&self) -> &'static str {
        let prefix = match self.tier {
            PowerUpTier::Standard => "",
            PowerUpTier::Enhanced => "Enhanced ",
            PowerUpTier::Rare     => "Rare ",
        };
        // We can't return a formatted String as &'static str easily,
        // so we encode both parts — callers should concatenate them.
        let _ = prefix; // silence warning; use tier_label() + kind_label() instead
        match (self.kind, self.tier) {
            (PowerUpKind::Hp,     PowerUpTier::Standard) => "HP Pack",
            (PowerUpKind::Hp,     PowerUpTier::Enhanced) => "HP Pack+",
            (PowerUpKind::Hp,     PowerUpTier::Rare)     => "Full Heal!",
            (PowerUpKind::Bolt,   PowerUpTier::Standard) => "Speed Bolt",
            (PowerUpKind::Bolt,   PowerUpTier::Enhanced) => "Speed Bolt+",
            (PowerUpKind::Bolt,   PowerUpTier::Rare)     => "Mega Bolt!",
            (PowerUpKind::Shield, PowerUpTier::Standard) => "Shield",
            (PowerUpKind::Shield, PowerUpTier::Enhanced) => "Shield+",
            (PowerUpKind::Shield, PowerUpTier::Rare)     => "Mega Shield!",
        }
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, powerup_rotation_system)
            .add_systems(Update, player_buff_tick_system)
            .add_systems(
                Update,
                powerup_magnet_system.run_if(in_state(GameStates::InGame)),
            )
            .add_systems(
                Update,
                spawn_powerup_system
                    .run_if(in_state(GameStates::InGame))
                    .run_if(on_timer(Duration::from_secs_f32(POWERUP_SPAWN_TIME))),
            );
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn powerup_rotation_system(mut query: Query<(&PowerUpComponent, &mut RotationAngle)>) {
    for (pu, mut angle) in &mut query {
        angle.0 += pu.rotation_speed;
    }
}

/// Ticks active buff timers every frame.
pub fn player_buff_tick_system(time: Res<Time>, mut buff: ResMut<PlayerBuff>) {
    let dt = time.delta_seconds();
    buff.bolt_timer   = (buff.bolt_timer   - dt).max(0.0);
    buff.shield_timer = (buff.shield_timer - dt).max(0.0);
}

fn powerup_magnet_system(
    upgrades:  Res<PlayerUpgrades>,
    player_q:  Query<&Position, With<PlayerComponent>>,
    mut pu_q:  Query<(&Position, &mut Velocity), With<PowerUpComponent>>,
) {
    if !upgrades.asteroid_magnet {
        return;
    }
    let Ok(player_pos) = player_q.get_single() else { return };

    for (pu_pos, mut vel) in pu_q.iter_mut() {
        let dir = (player_pos.0 - pu_pos.0).normalize_or_zero();
        vel.0 += dir * MAGNET_STRENGTH;
    }
}

fn spawn_powerup_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size:     Res<WindowSize>,
    query:        Query<With<PowerUpComponent>>,
) {
    let count = query.iter().count() as i32;
    if count >= POWERUP_MAX_COUNT {
        return;
    }

    let half_w = wdw_size.w / 2.0;
    let half_h = wdw_size.h / 2.0;
    let mut rng = thread_rng();

    let x = rng.gen_range(-half_w..half_w);
    let from_top = rng.gen_bool(0.5);
    let y = if from_top { half_h + 50.0 } else { -half_h - 50.0 };

    let speed_y   = if from_top { rng.gen_range(-1.5..-0.8) } else { rng.gen_range(0.8..1.5) };
    let speed_x   = rng.gen_range(-1.0..1.0);
    let rotation  = rng.gen_range(-0.1_f32..0.1);
    let rot_speed = rng.gen_range(-0.08_f32..0.08);

    // Tier: Standard 55%, Enhanced 30%, Rare 15%
    let tier = match rng.gen_range(0u8..20) {
        0..=2 => PowerUpTier::Rare,
        3..=8 => PowerUpTier::Enhanced,
        _     => PowerUpTier::Standard,
    };

    // Kind: HP 50%, Bolt 25%, Shield 25%
    let kind = match rng.gen_range(0u8..4) {
        0 => PowerUpKind::Bolt,
        1 => PowerUpKind::Shield,
        _ => PowerUpKind::Hp,
    };

    let texture = powerup_sprite(&game_sprites, kind, tier);

    commands
        .spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(x, y, 1.0),
                rotation:    Quat::from_rotation_z(rotation),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("PowerUp"))
        .insert(PowerUpComponent { kind, tier, rotation_speed: rot_speed })
        .insert(HitBoxSize(POWER_UP_SIZE))
        .insert(Velocity(Vec2::new(speed_x, speed_y)))
        .insert(Position(Vec2::new(x, y)))
        .insert(RotationAngle(rotation))
        .insert(BoundsDespawnable(Vec2::new(50.0, 50.0)))
        .insert(CleanUpOnLevelEnd);
}

/// Returns the correct sprite handle for a given kind + tier combination.
pub fn powerup_sprite(gs: &GameSprites, kind: PowerUpKind, tier: PowerUpTier) -> Handle<Image> {
    match (kind, tier) {
        (PowerUpKind::Hp,     PowerUpTier::Standard) => gs.powerup_hp.clone(),
        (PowerUpKind::Hp,     PowerUpTier::Enhanced) => gs.powerup_hp_green.clone(),
        (PowerUpKind::Hp,     PowerUpTier::Rare)     => gs.powerup_hp_red.clone(),
        (PowerUpKind::Bolt,   PowerUpTier::Standard) => gs.powerup_bolt.clone(),
        (PowerUpKind::Bolt,   PowerUpTier::Enhanced) => gs.powerup_bolt_green.clone(),
        (PowerUpKind::Bolt,   PowerUpTier::Rare)     => gs.powerup_bolt_red.clone(),
        (PowerUpKind::Shield, PowerUpTier::Standard) => gs.powerup_shield.clone(),
        (PowerUpKind::Shield, PowerUpTier::Enhanced) => gs.powerup_shield_green.clone(),
        (PowerUpKind::Shield, PowerUpTier::Rare)     => gs.powerup_shield_red.clone(),
    }
}
