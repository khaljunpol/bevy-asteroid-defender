use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use bevy_tweening::{
    EaseFunction,
    lens::{TransformPositionLens, TransformRotationLens},
    Tween, Animator, Tracks,
};
use rand::{thread_rng, Rng};

use lib::{PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_SIZE, ShipType};
use crate::{
    common::common_components::{Velocity, RotationAngle, HitBoxSize, Position, BoundsWarpable},
    effects::particle::ParticleComponent,
    objects::projectile::projectile_shoot_system,
    resources::{
        SHIP_NORMAL_SPRITE, SHIP_SHIELD_SPRITE, SHIP_ATTACK_SPRITE,
        GameSprites, WindowSize, PlayerUpgrades, Life,
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

/// Flashes red when the player takes damage.
#[derive(Component)]
pub struct PlayerDamageFlash {
    pub timer: Timer,
}

impl PlayerDamageFlash {
    pub fn new() -> Self {
        Self { timer: Timer::from_seconds(0.3, TimerMode::Once) }
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
            .add_systems(
                Update,
                player_engine_trail_system.run_if(in_state(GameStates::InGame)),
            )
            .add_systems(Update, player_damage_flash_system)
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

fn player_damage_flash_system(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut Sprite, &mut PlayerDamageFlash), With<PlayerComponent>>,
) {
    for (entity, mut sprite, mut flash) in &mut query {
        flash.timer.tick(time.delta());
        let t = flash.timer.percent();
        // Lerp from red (1,0,0) back toward white (1,1,1)
        sprite.color = Color::rgb(1.0, t, t);
        if flash.timer.just_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<PlayerDamageFlash>();
        }
    }
}

fn player_engine_trail_system(
    mut commands: Commands,
    keyboard:     Res<Input<KeyCode>>,
    game_sprites: Res<GameSprites>,
    query:        Query<(&Position, &RotationAngle), With<PlayerComponent>>,
) {
    if !keyboard.pressed(KeyCode::Up) {
        return;
    }

    let Ok((pos, angle)) = query.get_single() else { return };

    let mut rng = thread_rng();

    // Direction the ship faces (forward)
    let forward = Vec2::new(
        (angle.0 + PI / 2.0).cos(),
        (angle.0 + PI / 2.0).sin(),
    );
    let backward = -forward;

    for _ in 0..2 {
        let offset_dist = rng.gen_range(18.0_f32..28.0);
        let jitter_x    = rng.gen_range(-4.0_f32..4.0);
        let jitter_y    = rng.gen_range(-4.0_f32..4.0);

        let spawn_pos = Vec3::new(
            pos.0.x + backward.x * offset_dist + jitter_x,
            pos.0.y + backward.y * offset_dist + jitter_y,
            3.0,
        );

        let trail_speed = rng.gen_range(30.0_f32..70.0);
        let trail_vel   = backward * trail_speed + Vec2::new(jitter_x * 2.0, jitter_y * 2.0);
        let lifetime    = rng.gen_range(0.10_f32..0.22);
        let scale       = rng.gen_range(0.12_f32..0.30);

        commands.spawn((
            SpriteBundle {
                texture: game_sprites.speed.clone(),
                transform: Transform {
                    translation: spawn_pos,
                    scale: Vec3::splat(scale),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.5, 0.8, 1.0, 0.85),
                    ..default()
                },
                ..default()
            },
            ParticleComponent {
                lifetime,
                max_lifetime: lifetime,
                velocity: trail_vel,
            },
            Name::new("Engine Trail"),
        ));
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
    mut upgrades:        ResMut<PlayerUpgrades>,
    mut life:            ResMut<Life>,
    mut ev_player_spawn: EventWriter<crate::events::events::PlayerSpawnEvent>,
) {
    ev_player_spawn.send(crate::events::events::PlayerSpawnEvent);

    let ship = ShipComponent::new();

    // Apply ship-type starting bonuses/penalties
    match ship.ship_type {
        ShipType::Attack => {
            // Red ship: starts with Heavy Rounds level 1, but -1 max HP
            upgrades.heavy_rounds = upgrades.heavy_rounds.max(1);
            life.max_life     = (life.max_life - 1).max(1);
            life.current_life = life.max_life;
        }
        ShipType::Shield => {
            // Green ship: +2 max HP, but slightly slower
            life.max_life     += 2;
            life.current_life  = life.max_life;
            upgrades.shield_speed_penalty = true;
        }
        ShipType::Normal => {}
    }

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
