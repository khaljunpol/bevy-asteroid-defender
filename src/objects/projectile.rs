use bevy::prelude::*;
use lib::{PROJECTILE_SIZE, SPRITE_SCALE, ShipType};

use crate::{
    common::common_components::{RotationAngle, Position, HitBoxSize, Velocity},
    player::{
        player::{PlayerComponent, PlayerShootCooldownComponent},
        ship::ShipComponent,
    },
    resources::{GameSprites, IsPaused, PlayerBuff, PlayerUpgrades},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct ProjectileComponent {
    pub damage:          i32,
    /// World-unit position where this projectile was fired.
    pub origin:          Vec2,
    /// Maximum distance before auto-despawn (range-based).
    pub max_range:       f32,
    /// How many additional hits this projectile can pierce through (0 = normal).
    pub pierce_remaining: i32,
    /// Entities already hit by this projectile — prevents re-collision across frames.
    pub hit_meteors:     Vec<Entity>,
}

/// Tiny shrapnel spawned by Explosive Rounds on asteroid kill.
/// Shrapnel does not trigger further explosions and is not affected by upgrades.
#[derive(Component)]
pub struct ShrapnelComponent;

/// Marks bullets that explode at max range (Detonator Rounds upgrade).
#[derive(Component)]
pub struct DetonatorComponent;

/// Counts down until the projectile self-destructs (safety fallback).
#[derive(Component)]
pub struct ProjectileDespawnComponent(pub Timer);

impl Default for ProjectileDespawnComponent {
    fn default() -> Self {
        Self(Timer::from_seconds(8.0, TimerMode::Once))
    }
}

/// Ricochet flag – the bullet reflects off screen edges once.
#[derive(Component)]
pub struct ProjectileRicochet {
    pub bounced: bool,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, projectile_range_despawn_system)
            .add_systems(Update, projectile_despawn_system)
            .add_systems(
                Update,
                projectile_ricochet_system.run_if(in_state(GameStates::InGame)),
            )
            .add_systems(
                Update,
                chain_reaction_tick_system.run_if(in_state(GameStates::InGame)),
            );
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

/// Despawn projectiles that have exceeded their max range.
fn projectile_range_despawn_system(
    mut commands: Commands,
    game_sprites:  Res<GameSprites>,
    ship_q:        Query<&crate::player::ship::ShipComponent, With<crate::player::player::PlayerComponent>>,
    query:         Query<(Entity, &ProjectileComponent, &Position, Option<&DetonatorComponent>)>,
) {
    let ship_type = ship_q.get_single().map(|s| s.ship_type).unwrap_or(lib::ShipType::Normal);
    for (entity, proj, pos, detonator) in &query {
        if (pos.0 - proj.origin).length() >= proj.max_range {
            if detonator.is_some() {
                spawn_shrapnel(&mut commands, &game_sprites, pos.0, ship_type);
            }
            commands.entity(entity).despawn();
        }
    }
}

/// Safety timer fallback despawn.
fn projectile_despawn_system(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut ProjectileDespawnComponent)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_ricochet_system(
    mut commands: Commands,
    wdw_size:     Res<crate::resources::WindowSize>,
    mut query:    Query<(Entity, &mut Velocity, &Position, &mut Transform, &mut ProjectileRicochet)>,
) {
    let hw = wdw_size.w / 2.0;
    let hh = wdw_size.h / 2.0;
    let despawn_margin = 60.0;

    for (entity, mut vel, pos, mut tf, mut ricochet) in &mut query {
        if ricochet.bounced {
            if pos.0.x.abs() > hw + despawn_margin || pos.0.y.abs() > hh + despawn_margin {
                commands.entity(entity).despawn();
            }
            continue;
        }

        let mut did_bounce = false;
        if pos.0.x.abs() > hw { vel.0.x = -vel.0.x; did_bounce = true; }
        if pos.0.y.abs() > hh { vel.0.y = -vel.0.y; did_bounce = true; }

        if did_bounce {
            ricochet.bounced = true;
            tf.rotation = Quat::from_rotation_z(
                vel.0.y.atan2(vel.0.x) - std::f32::consts::PI / 2.0,
            );
        }
    }
}

fn chain_reaction_tick_system(
    time:         Res<Time>,
    mut upgrades: ResMut<PlayerUpgrades>,
) {
    if !upgrades.chain_active { return; }
    upgrades.chain_timer -= time.delta_seconds();
    if upgrades.chain_timer <= 0.0 {
        upgrades.chain_active = false;
        upgrades.chain_timer  = 0.0;
    }
}

// ── Shoot system ─────────────────────────────────────────────────────────────

pub fn projectile_shoot_system(
    mut commands:  Commands,
    kb:            Res<Input<KeyCode>>,
    game_sprites:  Res<GameSprites>,
    time:          Res<Time>,
    buff:          Res<PlayerBuff>,
    paused:        Res<IsPaused>,
    mut upgrades:  ResMut<PlayerUpgrades>,
    ship_q:        Query<&ShipComponent, With<PlayerComponent>>,
    mut player_q:  Query<(&PlayerComponent, &RotationAngle, &Position, &mut PlayerShootCooldownComponent)>,
) {
    if paused.0 { return; }
    let Ok(ship) = ship_q.get_single() else { return };
    let ship_type = ship.ship_type;

    let texture: Handle<Image> = match ship_type {
        ShipType::Attack => game_sprites.projectile_attack.clone(),
        ShipType::Normal => game_sprites.projectile_normal.clone(),
        ShipType::Shield => game_sprites.projectile_shield.clone(),
    };

    let proj_speed = upgrades.effective_projectile_speed(ship_type, buff.bolt_timer > 0.0);
    let proj_range = upgrades.effective_projectile_range(ship_type);
    let damage     = upgrades.bullet_damage();
    let pierce     = upgrades.pierce_count();

    for (player, angle, position, mut cooldown) in player_q.iter_mut() {
        cooldown.0.tick(time.delta());

        if !cooldown.0.finished() || !kb.pressed(KeyCode::Space) {
            continue;
        }

        let offsets = upgrades.shot_offsets();
        let ricochet_enabled = upgrades.ricochet;

        for &offset in &offsets {
            let shot_angle = angle.0 + offset;
            let direction  = {
                let (y, x) = (shot_angle + std::f32::consts::PI / 2.0).sin_cos();
                Vec2::new(x, y).normalize()
            };

            let mut entity_cmds = commands.spawn(SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(position.0.x, position.0.y, 5.0),
                    scale:       Vec3::splat(SPRITE_SCALE),
                    rotation:    Quat::from_rotation_z(shot_angle),
                    ..default()
                },
                ..default()
            });

            entity_cmds
                .insert(Name::new("Projectile"))
                .insert(ProjectileComponent {
                    damage,
                    origin: position.0,
                    max_range: proj_range,
                    pierce_remaining: pierce,
                    hit_meteors: Vec::new(),
                })
                .insert(ProjectileDespawnComponent::default())
                .insert(HitBoxSize(PROJECTILE_SIZE))
                .insert(Velocity(direction * proj_speed))
                .insert(Position(position.0))
                .insert(CleanUpOnLevelEnd);

            if ricochet_enabled {
                entity_cmds.insert(ProjectileRicochet { bounced: false });
            }
            if upgrades.detonator_rounds {
                entity_cmds.insert(DetonatorComponent);
            }
            // No BoundsDespawnable — range system handles despawn so upgrades visibly affect bullet reach.
        }

        let cd = upgrades.effective_shoot_cooldown(ship_type);
        cooldown.0 = Timer::from_seconds(cd, TimerMode::Once);
    }
}

/// Called by collision to spawn 4 shrapnel fragments at the given position.
pub fn spawn_shrapnel(
    commands:     &mut Commands,
    game_sprites: &GameSprites,
    origin:       Vec2,
    ship_type:    ShipType,
) {
    use rand::{thread_rng, Rng};
    use std::f32::consts::TAU;

    let texture = match ship_type {
        ShipType::Attack => game_sprites.projectile_attack.clone(),
        ShipType::Normal => game_sprites.projectile_normal.clone(),
        ShipType::Shield => game_sprites.projectile_shield.clone(),
    };

    let mut rng = thread_rng();
    for i in 0..3 {
        let angle = rng.gen_range(0.0..TAU) + (i as f32) * (TAU / 3.0);
        let speed = rng.gen_range(2.5_f32..4.5);
        let dir   = Vec2::new(angle.cos(), angle.sin());

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(origin.x, origin.y, 5.0),
                    scale:       Vec3::splat(0.3),
                    rotation:    Quat::from_rotation_z(angle - std::f32::consts::PI / 2.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.85, 0.4, 0.9),
                    ..default()
                },
                ..default()
            },
            ProjectileComponent {
                damage:          1,
                origin,
                max_range:       140.0,
                pierce_remaining: 0,
                hit_meteors:     Vec::new(),
            },
            ProjectileDespawnComponent::default(),
            HitBoxSize(Vec2::new(6.0, 20.0)),
            Velocity(dir * speed),
            Position(origin),
            ShrapnelComponent,
            CleanUpOnLevelEnd,
            Name::new("Shrapnel"),
        ));
    }
}

/// Called by the collision system to activate a Chain Reaction burst.
pub fn trigger_chain_reaction(upgrades: &mut PlayerUpgrades) {
    if upgrades.chain_reaction {
        upgrades.chain_active = true;
        upgrades.chain_timer  = lib::CHAIN_REACTION_DURATION;
    }
}
