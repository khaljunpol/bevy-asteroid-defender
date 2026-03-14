use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use bevy_tweening::{
    EaseFunction,
    lens::{TransformPositionLens, TransformRotationLens},
    Tween, Animator, Tracks,
};

use lib::{PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_SIZE, ShipType};
use crate::{
    common::common_components::{Velocity, RotationAngle, HitBoxSize, Position, BoundsWarpable},
    objects::projectile::projectile_shoot_system,
    resources::{
        SHIP_NORMAL_SPRITE, SHIP_SHIELD_SPRITE, SHIP_ATTACK_SPRITE,
        WindowSize, PlayerUpgrades,
    },
    state::states::GameStates,
    utils::cleanup::CleanUpOnGameOver,
};
use super::ship::ShipComponent;

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn direction(&self, rotation_angle: f32) -> Vec2 {
        let (y, x) = (rotation_angle + PI / 2.0).sin_cos();
        Vec2::new(x, y)
    }
}

#[derive(Component)]
pub struct PlayerShootCooldownComponent(pub Timer);

impl Default for PlayerShootCooldownComponent {
    fn default() -> Self {
        Self(Timer::from_seconds(lib::PLAYER_SHOOT_COOLDOWN, TimerMode::Once))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // InGame – movement and shooting
            .add_systems(
                Update,
                (player_movement_system, projectile_shoot_system)
                    .run_if(in_state(GameStates::InGame)),
            )
            // GameOver – freeze the player by removing physics components
            .add_systems(OnEnter(GameStates::GameOver), player_on_death_system)
            // EndGame – fly the ship off the top of the screen
            .add_systems(OnEnter(GameStates::GameOver), player_fly_out_system);
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn player_movement_system(
    keyboard:  Res<Input<KeyCode>>,
    upgrades:  Res<PlayerUpgrades>,
    mut query: Query<(&PlayerComponent, &mut Velocity, &mut RotationAngle)>,
) {
    if let Ok((player, mut velocity, mut angle)) = query.get_single_mut() {
        let turn_speed = upgrades.effective_turn_speed();
        let max_speed  = upgrades.effective_max_speed();

        if keyboard.pressed(KeyCode::Left) {
            angle.0 += turn_speed;
        } else if keyboard.pressed(KeyCode::Right) {
            angle.0 -= turn_speed;
        }

        if keyboard.pressed(KeyCode::Up) {
            velocity.0 += player.direction(angle.0) * PLAYER_ACCELERATION;
            if velocity.0.length() > max_speed {
                velocity.0 = velocity.0.normalize_or_zero() * max_speed;
            }
        } else {
            velocity.0 *= 1.0 - PLAYER_DECELERATION;
        }
    }
}

fn player_on_death_system(
    mut commands: Commands,
    query:        Query<Entity, With<PlayerComponent>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity)
            .remove::<Velocity>()
            .remove::<RotationAngle>()
            .remove::<Position>();
    }
}

fn player_fly_out_system(
    mut commands: Commands,
    wdw_size:     Res<WindowSize>,
    query:        Query<(Entity, &Transform), With<PlayerComponent>>,
) {
    if let Ok((entity, transform)) = query.get_single() {
        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformPositionLens {
                start: transform.translation,
                end:   Vec3::new(0.0, wdw_size.h, transform.translation.z),
            },
        );
        commands.entity(entity).insert(Animator::<Transform>::new(tween));
    }
}

// ── Spawn / cleanup (called by state plugins) ─────────────────────────────────

pub fn player_spawn_system(
    mut commands:        Commands,
    asset_server:        Res<AssetServer>,
    wdw_size:            Res<WindowSize>,
    mut ev_player_spawn: EventWriter<crate::events::events::PlayerSpawnEvent>,
) {
    ev_player_spawn.send(crate::events::events::PlayerSpawnEvent);

    let ship = ShipComponent::new();

    let sprite = match ship.ship_type {
        ShipType::Attack => asset_server.load(SHIP_ATTACK_SPRITE),
        ShipType::Normal => asset_server.load(SHIP_NORMAL_SPRITE),
        ShipType::Shield => asset_server.load(SHIP_SHIELD_SPRITE),
    };

    let start_pos = Vec3::new(0.0, -wdw_size.h, 0.0);

    let pos_tween = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs(2),
        TransformPositionLens {
            start: start_pos,
            end:   Vec3::new(0.0, 1.0, 0.0),
        },
    );
    let rot_tween = Tween::new(
        EaseFunction::ElasticIn,
        Duration::from_secs(2),
        TransformRotationLens {
            start: Quat::IDENTITY,
            end:   Quat::IDENTITY,
        },
    );

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            transform: Transform {
                translation: start_pos,
                scale:       Vec3::new(0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(PlayerComponent)
        .insert(ship)
        .insert(PlayerShootCooldownComponent::default())
        .insert(HitBoxSize(PLAYER_SIZE))
        .insert(Velocity(Vec2::ZERO))
        .insert(Position(Vec2::ZERO))
        .insert(RotationAngle(0.0))
        .insert(BoundsWarpable)
        .insert(CleanUpOnGameOver)
        .insert(Animator::<Transform>::new(Tracks::new([pos_tween, rot_tween])));
}

pub fn clean_up_player_tween(
    mut commands: Commands,
    query:        Query<Entity, With<PlayerComponent>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).remove::<Animator::<Transform>>();
    }
}
