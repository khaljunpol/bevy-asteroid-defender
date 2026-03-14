use bevy::prelude::*;
use lib::{PROJECTILE_DESPAWN_TIME, PROJECTILE_SIZE, PROJECTILE_SPEED, SPRITE_SCALE, ShipType};

use crate::{
    common::common_components::{RotationAngle, Position, HitBoxSize, Velocity, BoundsDespawnable},
    player::{
        player::{PlayerComponent, PlayerShootCooldownComponent},
        ship::ShipComponent,
    },
    resources::{GameSprites, PlayerUpgrades},
    state::states::GameStates,
    utils::cleanup::CleanUpOnLevelEnd,
};

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct ProjectileComponent {
    pub damage: i32,
}

/// Counts down until the projectile self-destructs.
#[derive(Component)]
pub struct ProjectileDespawnComponent(pub Timer);

impl Default for ProjectileDespawnComponent {
    fn default() -> Self {
        Self(Timer::from_seconds(PROJECTILE_DESPAWN_TIME, TimerMode::Once))
    }
}

/// Ricochet flag – the bullet will reflect off screen edges once.
#[derive(Component)]
pub struct ProjectileRicochet {
    pub bounced: bool,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
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
    mut query:    Query<(Entity, &mut Velocity, &Position, &mut ProjectileRicochet)>,
) {
    let hw = wdw_size.w / 2.0;
    let hh = wdw_size.h / 2.0;

    for (entity, mut vel, pos, mut ricochet) in &mut query {
        if ricochet.bounced {
            continue;
        }

        let mut did_bounce = false;

        if pos.0.x.abs() > hw {
            vel.0.x = -vel.0.x;
            did_bounce = true;
        }
        if pos.0.y.abs() > hh {
            vel.0.y = -vel.0.y;
            did_bounce = true;
        }

        if did_bounce {
            ricochet.bounced = true;
            // After the one bounce, behave like a normal projectile at the edges.
            commands.entity(entity).insert(BoundsDespawnable(Vec2::new(50.0, 50.0)));
        }
    }
}

/// Ticks the chain-reaction timer and deactivates it when expired.
fn chain_reaction_tick_system(
    time:          Res<Time>,
    mut upgrades:  ResMut<PlayerUpgrades>,
) {
    if !upgrades.chain_active {
        return;
    }
    upgrades.chain_timer -= time.delta_seconds();
    if upgrades.chain_timer <= 0.0 {
        upgrades.chain_active = false;
        upgrades.chain_timer  = 0.0;
    }
}

// ── Shoot system (called by PlayerPlugin) ────────────────────────────────────

pub fn projectile_shoot_system(
    mut commands:  Commands,
    kb:            Res<Input<KeyCode>>,
    game_sprites:  Res<GameSprites>,
    time:          Res<Time>,
    mut upgrades:  ResMut<PlayerUpgrades>,
    ship_q:        Query<&ShipComponent, With<PlayerComponent>>,
    mut player_q:  Query<(&PlayerComponent, &RotationAngle, &Position, &mut PlayerShootCooldownComponent)>,
) {
    let Ok(ship) = ship_q.get_single() else { return };

    let texture: Handle<Image> = match ship.ship_type {
        ShipType::Attack => game_sprites.projectile_attack.clone(),
        ShipType::Normal => game_sprites.projectile_normal.clone(),
        ShipType::Shield => game_sprites.projectile_shield.clone(),
    };

    for (player, angle, position, mut cooldown) in player_q.iter_mut() {
        cooldown.0.tick(time.delta());

        if !cooldown.0.finished() || !kb.pressed(KeyCode::Space) {
            continue;
        }

        let offsets = upgrades.shot_offsets();
        let damage  = upgrades.bullet_damage();
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
                .insert(ProjectileComponent { damage })
                .insert(ProjectileDespawnComponent::default())
                .insert(HitBoxSize(PROJECTILE_SIZE))
                .insert(Velocity(direction * PROJECTILE_SPEED))
                .insert(Position(position.0))
                .insert(CleanUpOnLevelEnd);

            if ricochet_enabled {
                entity_cmds.insert(ProjectileRicochet { bounced: false });
            } else {
                entity_cmds.insert(BoundsDespawnable(Vec2::new(10.0, 10.0)));
            }
        }

        // Reset cooldown using the effective rate (factors in Rapid Fire + Chain Reaction).
        let cd = upgrades.effective_shoot_cooldown();
        cooldown.0 = Timer::from_seconds(cd, TimerMode::Once);

        // Trigger chain-reaction activation is handled by the collision system
        // (see MeteorKilledEvent). The timer tick here keeps it alive.
    }
}

/// Called by the collision system to activate a Chain Reaction burst.
pub fn trigger_chain_reaction(upgrades: &mut PlayerUpgrades) {
    if upgrades.chain_reaction {
        upgrades.chain_active = true;
        upgrades.chain_timer  = lib::CHAIN_REACTION_DURATION;
    }
}
