use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use lib::PROJECTILE_SIZE;
use crate::{
    common::common_components::{HitBoxSize, Position, Velocity, BoundsDespawnable},
    objects::projectile::ProjectileDespawnComponent,
    player::player::PlayerComponent,
    resources::{GameSprites, IsPaused, LevelResource, WindowSize},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UfoType {
    /// Red – balanced scout. 3 HP, medium speed, single aimed shot.
    Scout,
    /// Yellow – heavy gunship. 6 HP, slow, fires 3-shot spread.
    Gunship,
    /// Green – fast bomber. 2 HP, fast movement, rapid fire.
    Bomber,
    /// Blue – precision sniper. 2 HP, medium speed, fast accurate shot.
    Sniper,
}

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct UfoComponent {
    pub ufo_type:       UfoType,
    pub hp:             i32,
    pub shoot_timer:    Timer,
    pub phase_offset:   f32,
    pub phase_speed:    f32,
    pub base_y:         f32,
    pub horizontal_vel: f32,
}

#[derive(Component)]
pub struct UfoHitFlash(pub Timer);

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
                    .run_if(in_state(GameStates::InGame))
                    .run_if(|p: Res<IsPaused>| !p.0),
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
    if level.current < 3 { return; }

    let mut rng = thread_rng();
    let ufo_count = ((level.current - 2) as usize).min(4);

    for _ in 0..ufo_count {
        // Pick type based on level with weighted random
        let ufo_type = if level.current >= 9 {
            match rng.gen_range(0..4) { 0 => UfoType::Gunship, 1 => UfoType::Bomber, 2 => UfoType::Sniper, _ => UfoType::Scout }
        } else if level.current >= 7 {
            match rng.gen_range(0..3) { 0 => UfoType::Gunship, 1 => UfoType::Bomber, _ => UfoType::Scout }
        } else if level.current >= 5 {
            if rng.gen_bool(0.4) { UfoType::Gunship } else { UfoType::Scout }
        } else {
            UfoType::Scout
        };
        spawn_ufo_typed(&mut commands, &game_sprites, &wdw_size, ufo_type);
    }
}

pub fn spawn_ufo_typed(
    commands:     &mut Commands,
    game_sprites: &GameSprites,
    wdw_size:     &WindowSize,
    ufo_type:     UfoType,
) {
    let mut rng = thread_rng();

    let (texture, hp, speed, shoot_secs, phase_spd, scale) = match ufo_type {
        UfoType::Scout   => (game_sprites.ufo.clone(),        3, 70.0_f32,  3.0_f32, 2.0_f32, 0.6_f32),
        UfoType::Gunship => (game_sprites.ufo_yellow.clone(), 6, 38.0,      4.5,     1.2,     0.80),
        UfoType::Bomber  => (game_sprites.ufo_green.clone(),  2, 130.0,     1.5,     3.0,     0.55),
        UfoType::Sniper  => (game_sprites.ufo_blue.clone(),   2, 90.0,      2.0,     1.8,     0.55),
    };

    let from_left     = rng.gen_bool(0.5);
    let start_x       = if from_left { -wdw_size.w / 2.0 - 60.0 } else { wdw_size.w / 2.0 + 60.0 };
    let horiz_vel     = if from_left { speed } else { -speed };
    let base_y        = rng.gen_range(-wdw_size.h * 0.35..wdw_size.h * 0.35);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(start_x, base_y, 2.0),
                scale: Vec3::splat(scale),
                ..default()
            },
            ..default()
        },
        Name::new(format!("UFO {:?}", ufo_type)),
        UfoComponent {
            ufo_type,
            hp,
            shoot_timer:    Timer::from_seconds(shoot_secs, TimerMode::Repeating),
            phase_offset:   rng.gen_range(0.0_f32..std::f32::consts::TAU),
            phase_speed:    phase_spd,
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
    let edge = wdw_size.w / 2.0 + 120.0;

    for (entity, mut ufo, mut tf) in &mut query {
        ufo.phase_offset += ufo.phase_speed * dt;
        tf.translation.x += ufo.horizontal_vel * dt;

        let amplitude = match ufo.ufo_type {
            UfoType::Scout   => 80.0,
            UfoType::Gunship => 110.0,
            UfoType::Bomber  => 35.0,
            UfoType::Sniper  => 55.0,
        };
        tf.translation.y = ufo.base_y + ufo.phase_offset.sin() * amplitude;

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
        if !ufo.shoot_timer.just_finished() { continue; }

        let to_player = (player_tf.translation.truncate() - ufo_tf.translation.truncate()).normalize_or_zero();
        if to_player == Vec2::ZERO { continue; }

        match ufo.ufo_type {
            UfoType::Scout => {
                fire_ufo_shot(&mut commands, &game_sprites, ufo_tf.translation, to_player, 5.0, Color::rgb(1.0, 0.4, 0.4));
            }
            UfoType::Gunship => {
                // 3-shot spread
                for spread in [-0.25_f32, 0.0, 0.25] {
                    let rot = Vec2::new(
                        to_player.x * spread.cos() - to_player.y * spread.sin(),
                        to_player.x * spread.sin() + to_player.y * spread.cos(),
                    );
                    fire_ufo_shot(&mut commands, &game_sprites, ufo_tf.translation, rot, 4.5, Color::rgb(1.0, 0.85, 0.2));
                }
            }
            UfoType::Bomber => {
                fire_ufo_shot(&mut commands, &game_sprites, ufo_tf.translation, to_player, 6.0, Color::rgb(0.3, 1.0, 0.4));
            }
            UfoType::Sniper => {
                // Fast, high-speed shot
                fire_ufo_shot(&mut commands, &game_sprites, ufo_tf.translation, to_player, 11.0, Color::rgb(0.4, 0.7, 1.0));
            }
        }
    }
}

fn fire_ufo_shot(
    commands:     &mut Commands,
    game_sprites: &GameSprites,
    origin:       Vec3,
    dir:          Vec2,
    speed:        f32,
    color:        Color,
) {
    let angle = dir.y.atan2(dir.x) - PI / 2.0;
    commands.spawn((
        SpriteBundle {
            texture: game_sprites.projectile_attack.clone(),
            transform: Transform {
                translation: Vec3::new(origin.x, origin.y, 4.0),
                scale:       Vec3::splat(0.5),
                rotation:    Quat::from_rotation_z(angle),
                ..default()
            },
            sprite: Sprite { color, ..default() },
            ..default()
        },
        Name::new("UFO Projectile"),
        UfoProjectileComponent,
        ProjectileDespawnComponent::default(),
        HitBoxSize(PROJECTILE_SIZE),
        Velocity(dir * speed),
        Position(Vec2::new(origin.x, origin.y)),
        BoundsDespawnable(Vec2::new(20.0, 20.0)),
        CleanUpOnLevelEnd,
    ));
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
