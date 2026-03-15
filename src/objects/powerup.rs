use std::time::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::prelude::*;

use lib::{POWER_UP_SIZE, POWERUP_MAX_COUNT, POWERUP_SPAWN_TIME, MAGNET_STRENGTH};
use crate::{
    common::common_components::{RotationAngle, Velocity, Position, HitBoxSize, BoundsDespawnable},
    player::player::PlayerComponent,
    resources::{GameSprites, Life, PlayerUpgrades, WindowSize},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Component ─────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct PowerUpComponent {
    rotation_speed: f32,
}

impl PowerUpComponent {
    pub fn apply(&self, life: &mut Life) {
        // Restore 1 HP, capped at max.
        life.current_life = (life.current_life + 1).min(life.max_life);
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, powerup_rotation_system)
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

/// When the player has Asteroid Magnet, nudge powerup velocity toward the player.
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
    let y = if from_top {
        half_h + 50.0
    } else {
        -half_h - 50.0
    };

    let speed_y = if from_top { rng.gen_range(-1.5..-0.8) } else { rng.gen_range(0.8..1.5) };
    let speed_x = rng.gen_range(-1.0..1.0);
    let rotation = rng.gen_range(-0.1_f32..0.1);
    let rot_speed = rng.gen_range(-0.08_f32..0.08);

    commands
        .spawn(SpriteBundle {
            texture: game_sprites.powerup_hp.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 1.0),
                rotation:    Quat::from_rotation_z(rotation),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("HP Pack"))
        .insert(PowerUpComponent { rotation_speed: rot_speed })
        .insert(HitBoxSize(POWER_UP_SIZE))
        .insert(Velocity(Vec2::new(speed_x, speed_y)))
        .insert(Position(Vec2::new(x, y)))
        .insert(RotationAngle(rotation))
        .insert(BoundsDespawnable(Vec2::new(50.0, 50.0)))
        .insert(CleanUpOnLevelEnd);
}
