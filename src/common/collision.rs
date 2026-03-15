use bevy::{prelude::*, sprite::collide_aabb::collide, math::Vec3Swizzles};
use std::collections::HashSet;
use rand::{thread_rng, Rng};

use lib::{meteor_score, MeteorSizeType};

use crate::{
    common::common_components::{HitBoxSize, CollisionDespawnableWithDamage, DamageCollision, MeteorSplitEvent},
    effects::particle::spawn_explosion,
    events::events::PlayerDeadEvent,
    objects::{
        meteor::{MeteorComponent, spawn_meteor, MeteorHitFlash},
        projectile::ProjectileComponent,
        powerup::PowerUpComponent,
        ufo::{UfoComponent, UfoHitFlash, UfoProjectileComponent},
    },
    player::player::{PlayerComponent, PlayerDamageFlash},
    resources::{CameraShake, GameSprites, Life, PlayerUpgrades, Score},
    state::states::GameStates,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_projectile_hit_meteor_system,
                player_projectile_hit_ufo_system,
                meteor_split_system,
                player_hit_by_meteor_system,
                player_hit_by_ufo_projectile_system,
                apply_damage_system,
                player_collect_powerup_system,
            )
                .run_if(in_state(GameStates::InGame)),
        );
    }
}

// ── Projectile → Meteor ───────────────────────────────────────────────────────

fn player_projectile_hit_meteor_system(
    mut commands:  Commands,
    game_sprites:  Res<GameSprites>,
    mut shake:     ResMut<CameraShake>,
    projectile_q:  Query<(Entity, &Transform, &HitBoxSize, &ProjectileComponent), Without<UfoProjectileComponent>>,
    mut meteor_q:  Query<(Entity, &Transform, &HitBoxSize, &mut MeteorComponent)>,
    mut score:     ResMut<Score>,
    mut upgrades:  ResMut<PlayerUpgrades>,
) {
    let mut despawned_projectiles: HashSet<Entity> = HashSet::new();
    let mut despawned_meteors:     HashSet<Entity> = HashSet::new();

    for (proj_e, proj_tf, proj_hit, projectile) in &projectile_q {
        if despawned_projectiles.contains(&proj_e) {
            continue;
        }

        let proj_scale = proj_tf.scale.xy();

        for (meteor_e, meteor_tf, meteor_hit, mut meteor) in meteor_q.iter_mut() {
            if despawned_meteors.contains(&meteor_e) || despawned_projectiles.contains(&proj_e) {
                continue;
            }

            let meteor_scale = meteor_tf.scale.xy();

            let hit = collide(
                proj_tf.translation, proj_hit.0 * proj_scale,
                meteor_tf.translation, meteor_hit.0 * meteor_scale,
            );

            if hit.is_none() {
                continue;
            }

            // Deal damage.
            meteor.health -= projectile.damage;

            if meteor.health <= 0 {
                // Destroyed – despawn and schedule fragment spawn.
                let meteor_pos  = meteor_tf.translation;
                let meteor_size = meteor.size;
                commands.entity(meteor_e).despawn();
                despawned_meteors.insert(meteor_e);

                score.current += meteor_score(meteor_size);

                // Screen shake – bigger for large asteroids
                if meteor_size == MeteorSizeType::Large {
                    shake.trigger(4.0);
                } else {
                    shake.trigger(1.5);
                }

                // Explosion particles
                spawn_explosion(&mut commands, &game_sprites, meteor_pos, meteor_size);

                // Activate chain reaction burst if the player has that upgrade.
                crate::objects::projectile::trigger_chain_reaction(&mut upgrades);

                // Emit a split event so fragment spawning doesn't need GameSprites here.
                commands.spawn((
                    MeteorSplitEvent {
                        size:        meteor_size as i32,
                        translation: meteor_pos,
                    },
                    Name::new("MeteorSplitEvent"),
                ));
            } else {
                // Survived – flash white.
                commands.entity(meteor_e).insert(MeteorHitFlash(
                    Timer::from_seconds(0.15, TimerMode::Once),
                ));
            }

            // Consume the projectile.
            if !despawned_projectiles.contains(&proj_e) {
                commands.entity(proj_e).despawn();
                despawned_projectiles.insert(proj_e);
            }
        }
    }
}

// ── Projectile → UFO ─────────────────────────────────────────────────────────

fn player_projectile_hit_ufo_system(
    mut commands:  Commands,
    game_sprites:  Res<GameSprites>,
    mut shake:     ResMut<CameraShake>,
    projectile_q:  Query<(Entity, &Transform, &HitBoxSize, &ProjectileComponent), Without<UfoProjectileComponent>>,
    mut ufo_q:     Query<(Entity, &Transform, &HitBoxSize, &mut UfoComponent)>,
    mut score:     ResMut<Score>,
) {
    let mut despawned_projectiles: HashSet<Entity> = HashSet::new();
    let mut despawned_ufos:        HashSet<Entity> = HashSet::new();

    for (proj_e, proj_tf, proj_hit, projectile) in &projectile_q {
        if despawned_projectiles.contains(&proj_e) {
            continue;
        }

        let proj_scale = proj_tf.scale.xy();

        for (ufo_e, ufo_tf, ufo_hit, mut ufo) in ufo_q.iter_mut() {
            if despawned_ufos.contains(&ufo_e) || despawned_projectiles.contains(&proj_e) {
                continue;
            }

            let hit = collide(
                proj_tf.translation, proj_hit.0 * proj_scale,
                ufo_tf.translation,  ufo_hit.0 * ufo_tf.scale.xy(),
            );

            if hit.is_none() {
                continue;
            }

            ufo.hp -= projectile.damage;

            if ufo.hp <= 0 {
                let ufo_pos = ufo_tf.translation;
                commands.entity(ufo_e).despawn();
                despawned_ufos.insert(ufo_e);

                score.current += 150;
                shake.trigger(3.0);
                spawn_explosion(&mut commands, &game_sprites, ufo_pos, MeteorSizeType::Large);
            } else {
                commands.entity(ufo_e).insert(UfoHitFlash(
                    Timer::from_seconds(0.15, TimerMode::Once),
                ));
            }

            if !despawned_projectiles.contains(&proj_e) {
                commands.entity(proj_e).despawn();
                despawned_projectiles.insert(proj_e);
            }
        }
    }
}

// ── Meteor fragment spawner ───────────────────────────────────────────────────

fn meteor_split_system(
    mut commands:  Commands,
    game_sprites:  Res<GameSprites>,
    upgrades:      Res<PlayerUpgrades>,
    query:         Query<(Entity, &MeteorSplitEvent)>,
) {
    let mut rng = thread_rng();

    for (entity, event) in &query {
        commands.entity(entity).despawn();

        if event.size <= 1 {
            // Small meteors don't split further.
            continue;
        }

        let child_size = match event.size - 1 {
            1 => MeteorSizeType::Small,
            _ => MeteorSizeType::Medium,
        };

        for i in 1..=3 {
            let rot       = rng.gen_range(-1.0_f32..1.0);
            let rot_speed = rng.gen_range(-0.05_f32..0.05);
            let speed_val = i as f32 * 0.8;
            let mut vel   = Vec2::new(
                rng.gen_range(-speed_val..speed_val),
                rng.gen_range(-speed_val..speed_val),
            );

            if upgrades.overclock {
                vel *= lib::OVERCLOCK_SPEED_MULT;
            }

            spawn_meteor(
                &mut commands,
                &game_sprites,
                child_size,
                event.translation,
                Vec2::new(event.translation.x, event.translation.y),
                rot,
                rot_speed,
                vel,
                1, // fragment children always have 1 HP
            );
        }
    }
}

// ── Player ← Meteor ───────────────────────────────────────────────────────────

fn player_hit_by_meteor_system(
    mut commands:  Commands,
    mut shake:     ResMut<CameraShake>,
    player_q:      Query<(Entity, &Transform, &HitBoxSize), With<PlayerComponent>>,
    meteor_q:      Query<(Entity, &Transform, &HitBoxSize, &CollisionDespawnableWithDamage), With<MeteorComponent>>,
) {
    let mut despawned: HashSet<Entity> = HashSet::new();

    for (player_e, player_tf, player_hit) in &player_q {
        let player_scale = player_tf.scale.xy();

        for (meteor_e, meteor_tf, meteor_hit, damageable) in &meteor_q {
            if despawned.contains(&meteor_e) {
                continue;
            }

            let meteor_scale = meteor_tf.scale.xy();

            let hit = collide(
                player_tf.translation, player_hit.0 * player_scale,
                meteor_tf.translation, meteor_hit.0 * meteor_scale,
            );

            if hit.is_none() {
                continue;
            }

            commands.entity(meteor_e).despawn();
            despawned.insert(meteor_e);

            if damageable.should_damage {
                shake.trigger(8.0);
                commands.entity(player_e).insert(PlayerDamageFlash::new());
                commands.spawn((
                    DamageCollision(damageable.damage),
                    Name::new("ContactDamage"),
                ));
            }
        }
    }
}

// ── Player ← UFO projectile ───────────────────────────────────────────────────

fn player_hit_by_ufo_projectile_system(
    mut commands: Commands,
    mut shake:    ResMut<CameraShake>,
    player_q:     Query<(Entity, &Transform, &HitBoxSize), With<PlayerComponent>>,
    proj_q:       Query<(Entity, &Transform, &HitBoxSize), With<UfoProjectileComponent>>,
) {
    let mut despawned: HashSet<Entity> = HashSet::new();

    for (player_e, player_tf, player_hit) in &player_q {
        let player_scale = player_tf.scale.xy();

        for (proj_e, proj_tf, proj_hit) in &proj_q {
            if despawned.contains(&proj_e) {
                continue;
            }

            let hit = collide(
                player_tf.translation, player_hit.0 * player_scale,
                proj_tf.translation,   proj_hit.0 * proj_tf.scale.xy(),
            );

            if hit.is_none() {
                continue;
            }

            commands.entity(proj_e).despawn();
            despawned.insert(proj_e);

            shake.trigger(6.0);
            commands.entity(player_e).insert(PlayerDamageFlash::new());
            commands.spawn((
                DamageCollision(1),
                Name::new("UfoProjectileDamage"),
            ));
        }
    }
}

// ── Damage application ────────────────────────────────────────────────────────

fn apply_damage_system(
    mut commands:  Commands,
    damage_q:      Query<(Entity, &DamageCollision)>,
    mut ev_dead:   EventWriter<PlayerDeadEvent>,
    mut life:      ResMut<Life>,
) {
    for (entity, damage) in &damage_q {
        life.current_life = (life.current_life - damage.0).max(0);
        commands.entity(entity).despawn();

        if life.current_life == 0 {
            ev_dead.send(PlayerDeadEvent);
            break;
        }
    }
}

// ── Player ← Power-up ────────────────────────────────────────────────────────

fn player_collect_powerup_system(
    mut commands:  Commands,
    player_q:      Query<(&Transform, &HitBoxSize), With<PlayerComponent>>,
    powerup_q:     Query<(Entity, &Transform, &HitBoxSize, &PowerUpComponent), With<PowerUpComponent>>,
    mut life:      ResMut<Life>,
) {
    let mut collected: HashSet<Entity> = HashSet::new();

    for (player_tf, player_hit) in &player_q {
        let p_scale = player_tf.scale.xy();

        for (powerup_e, powerup_tf, powerup_hit, powerup) in &powerup_q {
            if collected.contains(&powerup_e) {
                continue;
            }

            let hit = collide(
                player_tf.translation, player_hit.0 * p_scale,
                powerup_tf.translation, powerup_hit.0 * powerup_tf.scale.xy(),
            );

            if hit.is_none() {
                continue;
            }

            collected.insert(powerup_e);
            commands.entity(powerup_e).despawn();

            powerup.apply(&mut life);
        }
    }
}
