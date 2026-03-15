use bevy::prelude::*;
use rand::prelude::*;

use lib::{MeteorSizeType, meteor_damage, meteor_size};
use crate::{
    common::common_components::{
        RotationAngle, Velocity, Position, HitBoxSize,
        CollisionDespawnableWithDamage, BoundsDespawnable,
    },
    resources::{GameSprites, WindowSize, LevelResource, PlayerUpgrades},
    state::states::GameStates,
    utils::{
        cleanup::CleanUpOnLevelEnd,
        utils::{get_angle_to_target, calculate_max_spawn_distance},
    },
};

// ── Component ─────────────────────────────────────────────────────────────────

#[derive(Component, Default)]
pub struct MeteorComponent {
    pub size:           MeteorSizeType,
    pub rotation_speed: f32,
    pub health: i32,
}

/// Flashes white when a meteor is hit but not yet destroyed.
#[derive(Component)]
pub struct MeteorHitFlash(pub Timer);

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, meteor_rotation_system)
            .add_systems(Update, meteor_hit_flash_system)
            .add_systems(
                Update,
                check_level_complete_system.run_if(in_state(GameStates::InGame)),
            );
    }
}

// ── Level-start spawn ─────────────────────────────────────────────────────────

/// Called by `InGameStatePlugin::OnEnter(InGame)` to pre-spawn this level's asteroids.
pub fn spawn_level_asteroids(
    mut commands:    Commands,
    game_sprites:    Res<GameSprites>,
    wdw_size:        Res<WindowSize>,
    mut level:       ResMut<LevelResource>,
    upgrades:        Res<PlayerUpgrades>,
) {
    let count  = level.asteroids_for_level();
    let hp     = level.asteroid_hp_for_level();
    let center = Vec2::ZERO;

    let mut rng = thread_rng();
    let max_dist = calculate_max_spawn_distance(Vec2::new(wdw_size.w, wdw_size.h));

    for _ in 0..count {
        let angle   = rng.gen_range(0.0_f32..360.0).to_radians();
        let (sy, sx) = angle.sin_cos();
        let position = Vec2::new(sx * max_dist, sy * max_dist);

        let rotation       = rng.gen_range(-0.05_f32..0.05);
        let rotation_speed = rng.gen_range(-0.03_f32..0.03);
        let base_velocity  = get_angle_to_target(center, position);

        // Slow down asteroids if the player has Overclock.
        let velocity = if upgrades.overclock {
            base_velocity * lib::OVERCLOCK_SPEED_MULT
        } else {
            base_velocity
        };

        spawn_meteor(
            &mut commands,
            &game_sprites,
            MeteorSizeType::Large,
            Vec3::new(position.x, position.y, 1.0),
            position,
            rotation,
            rotation_speed,
            velocity,
            hp,
        );
    }

    level.total_asteroids_spawned = count;
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Spawns a single meteor entity.
/// `hp` is only meaningful for Large meteors at level start; fragment children
/// always receive `hp = 1` from the collision system.
pub fn spawn_meteor(
    commands:       &mut Commands,
    game_sprites:   &GameSprites,
    size:           MeteorSizeType,
    spawn_position: Vec3,
    position:       Vec2,
    rotation:       f32,
    rotation_speed: f32,
    velocity:       Vec2,
    hp:             i32,
) {
    let (name, sprite) = match size {
        MeteorSizeType::Large  => ("Meteor Large",  game_sprites.meteor_big.clone()),
        MeteorSizeType::Medium => ("Meteor Medium", game_sprites.meteor_med.clone()),
        MeteorSizeType::Small  => ("Meteor Small",  game_sprites.meteor_sml.clone()),
    };

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            transform: Transform {
                translation: spawn_position,
                rotation:    Quat::from_rotation_z(rotation),
                scale:       Vec3::ONE,
                ..default()
            },
            ..default()
        })
        .insert(Name::new(name))
        .insert(MeteorComponent {
            size,
            rotation_speed,
            health: hp,
        })
        .insert(HitBoxSize(meteor_size(size)))
        .insert(Velocity(velocity))
        .insert(Position(position))
        .insert(RotationAngle(rotation))
        .insert(BoundsDespawnable(Vec2::new(200.0, 200.0)))
        .insert(CollisionDespawnableWithDamage::new(true, meteor_damage(size)))
        .insert(CleanUpOnLevelEnd);
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn meteor_rotation_system(mut query: Query<(&MeteorComponent, &mut RotationAngle)>) {
    for (meteor, mut angle) in &mut query {
        angle.0 += meteor.rotation_speed;
    }
}

fn meteor_hit_flash_system(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut Sprite, &mut MeteorHitFlash)>,
) {
    for (entity, mut sprite, mut flash) in &mut query {
        flash.0.tick(time.delta());
        // Lerp back to full white (normal) as the timer expires.
        let t = flash.0.percent();
        sprite.color = Color::rgb(1.0, 1.0 - t * 0.6, 1.0 - t * 0.6);
        if flash.0.just_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<MeteorHitFlash>();
        }
    }
}

/// Transitions to LevelComplete when no meteors or UFOs remain.
/// The guard on `total_asteroids_spawned` prevents a false trigger on the
/// very first frame before asteroids have been fully spawned.
fn check_level_complete_system(
    level:          Res<LevelResource>,
    meteor_query:   Query<&MeteorComponent>,
    ufo_query:      Query<&crate::objects::ufo::UfoComponent>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if level.total_asteroids_spawned > 0 && meteor_query.is_empty() && ufo_query.is_empty() {
        next_state.set(GameStates::LevelComplete);
    }
}
