use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use lib::PROJECTILE_SIZE;
use crate::{
    common::common_components::{HitBoxSize, Position, Velocity, BoundsDespawnable},
    objects::projectile::ProjectileDespawnComponent,
    player::player::PlayerComponent,
    resources::{GameSprites, LevelResource, WindowSize},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct UfoComponent {
    pub hp:             i32,
    pub shoot_timer:    Timer,
    pub phase_offset:   f32,
    pub phase_speed:    f32,
    pub base_y:         f32,
    pub horizontal_vel: f32,
}

/// Flashes when a UFO is hit but not yet destroyed.
#[derive(Component)]
pub struct UfoHitFlash(pub Timer);

/// Marks projectiles fired by UFOs — distinguishes them from player bullets.
#[derive(Component)]
pub struct UfoProjectileComponent;

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct UfoPlugin;

impl Plugin for UfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameStates::InGame), spawn_ufo_for_level)
            .add_systems(
                Update,
                (ufo_movement_system, ufo_shoot_system, ufo_hit_flash_system)
                    .run_if(in_state(GameStates::InGame)),
            );
    }
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

fn spawn_ufo_for_level(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size:     Res<WindowSize>,
    level:        Res<LevelResource>,
) {
    if level.current < 5 {
        return;
    }

    let ufo_count = ((level.current - 4) as usize).min(3);
    for _ in 0..ufo_count {
        spawn_ufo(&mut commands, &game_sprites, &wdw_size);
    }
}

pub fn spawn_ufo(
    commands:     &mut Commands,
    game_sprites: &GameSprites,
    wdw_size:     &WindowSize,
) {
    let mut rng = thread_rng();
    let from_left     = rng.gen_bool(0.5);
    let start_x       = if from_left { -wdw_size.w / 2.0 - 60.0 } else { wdw_size.w / 2.0 + 60.0 };
    let horiz_vel     = if from_left { 70.0_f32 } else { -70.0_f32 };
    let base_y        = rng.gen_range(-wdw_size.h * 0.3..wdw_size.h * 0.3);
    let shoot_secs    = rng.gen_range(2.5_f32..4.5);

    commands.spawn((
        SpriteBundle {
            texture: game_sprites.ufo.clone(),
            transform: Transform {
                translation: Vec3::new(start_x, base_y, 2.0),
                scale: Vec3::splat(0.6),
                ..default()
            },
            ..default()
        },
        Name::new("UFO"),
        UfoComponent {
            hp:             3,
            shoot_timer:    Timer::from_seconds(shoot_secs, TimerMode::Repeating),
            phase_offset:   rng.gen_range(0.0_f32..std::f32::consts::TAU),
            phase_speed:    rng.gen_range(1.5_f32..2.5),
            base_y,
            horizontal_vel: horiz_vel,
        },
        HitBoxSize(Vec2::new(80.0, 55.0)),
        CleanUpOnLevelEnd,
    ));
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn ufo_movement_system(
    mut commands: Commands,
    time:         Res<Time>,
    wdw_size:     Res<WindowSize>,
    mut query:    Query<(Entity, &mut UfoComponent, &mut Transform)>,
) {
    let dt   = time.delta_seconds();
    let edge = wdw_size.w / 2.0 + 110.0;

    for (entity, mut ufo, mut tf) in &mut query {
        ufo.phase_offset    += ufo.phase_speed * dt;
        tf.translation.x    += ufo.horizontal_vel * dt;
        tf.translation.y     = ufo.base_y + ufo.phase_offset.sin() * 80.0;

        if tf.translation.x.abs() > edge {
            commands.entity(entity).despawn();
        }
    }
}

fn ufo_shoot_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    time:         Res<Time>,
    player_q:     Query<&Transform, With<PlayerComponent>>,
    mut ufo_q:    Query<(&Transform, &mut UfoComponent)>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };

    for (ufo_tf, mut ufo) in ufo_q.iter_mut() {
        ufo.shoot_timer.tick(time.delta());
        if !ufo.shoot_timer.just_finished() {
            continue;
        }

        let dir = (player_tf.translation.truncate()
                   - ufo_tf.translation.truncate())
                   .normalize_or_zero();
        if dir == Vec2::ZERO {
            continue;
        }

        let angle = dir.y.atan2(dir.x) - PI / 2.0;
        let speed = 5.0_f32;

        commands.spawn((
            SpriteBundle {
                texture: game_sprites.projectile_attack.clone(),
                transform: Transform {
                    translation: Vec3::new(ufo_tf.translation.x, ufo_tf.translation.y, 4.0),
                    scale:       Vec3::splat(0.5),
                    rotation:    Quat::from_rotation_z(angle),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.4, 0.4),
                    ..default()
                },
                ..default()
            },
            Name::new("UFO Projectile"),
            UfoProjectileComponent,
            ProjectileDespawnComponent::default(),
            HitBoxSize(PROJECTILE_SIZE),
            Velocity(dir * speed),
            Position(Vec2::new(ufo_tf.translation.x, ufo_tf.translation.y)),
            BoundsDespawnable(Vec2::new(20.0, 20.0)),
            CleanUpOnLevelEnd,
        ));
    }
}

fn ufo_hit_flash_system(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut Sprite, &mut UfoHitFlash)>,
) {
    for (entity, mut sprite, mut flash) in &mut query {
        flash.0.tick(time.delta());
        let t = flash.0.percent();
        sprite.color = Color::rgb(1.0, 1.0 - t * 0.6, 1.0 - t * 0.6);
        if flash.0.just_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<UfoHitFlash>();
        }
    }
}
