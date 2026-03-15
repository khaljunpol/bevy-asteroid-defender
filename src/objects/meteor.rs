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

/// Stores the base tint color of the meteor so the flash can restore it.
#[derive(Component)]
pub struct MeteorBaseColor(pub Color);

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

        // Speed scales with level (+8% per level, capped at 2.0x)
        let speed_scale = (1.0 + (level.current as f32 - 1.0) * 0.08).min(2.0);
        let velocity = if upgrades.overclock {
            base_velocity * lib::OVERCLOCK_SPEED_MULT
        } else {
            base_velocity * speed_scale
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

fn hp_color(hp: i32) -> Color {
    match hp {
        1 => Color::WHITE,
        2 => Color::rgb(1.0, 0.88, 0.4),   // gold
        3 => Color::rgb(1.0, 0.55, 0.2),   // orange
        _ => Color::rgb(1.0, 0.25, 0.25),  // red (4+ HP)
    }
}

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

    let base_color = hp_color(hp);

    commands
        .spawn(SpriteBundle {
            texture: sprite,
            transform: Transform {
                translation: spawn_position,
                rotation:    Quat::from_rotation_z(rotation),
                scale:       Vec3::ONE,
                ..default()
            },
            sprite: Sprite {
                color: base_color,
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
        .insert(MeteorBaseColor(base_color))
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
    mut query:    Query<(Entity, &mut Sprite, &mut MeteorHitFlash, &MeteorBaseColor)>,
) {
    for (entity, mut sprite, mut flash, base_color) in &mut query {
        flash.0.tick(time.delta());
        let t = flash.0.percent();
        // Lerp from white back to base color as t goes 0→1
        let [br, bg, bb, _] = base_color.0.as_rgba_f32();
        sprite.color = Color::rgb(
            1.0 - t * (1.0 - br),
            1.0 - t * (1.0 - bg),
            1.0 - t * (1.0 - bb),
        );
        if flash.0.just_finished() {
            sprite.color = base_color.0;
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
