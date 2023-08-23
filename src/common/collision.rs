use bevy::{prelude::*, 
    sprite::collide_aabb::collide, 
    math::Vec3Swizzles};
use lib::ShipType;

use std::collections::HashSet;

use crate::{
    common::common_components::{HitBoxSize, MeteorCollisionComponent}, 
    player::{
        player::PlayerComponent,
        ship::ShipComponent,
        projectile::{ProjectileDespawnComponent, ProjectileComponent}
    },
    objects::{
        powerup::PowerUpComponent,
        meteor::MeteorComponent
    },
    resources::GameSprites
};

pub fn player_collide_powerup_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    player_query: Query<(&Transform, &HitBoxSize), With<PlayerComponent>>,
    powerup_query: Query<(Entity, &Transform, &HitBoxSize, &PowerUpComponent), With<PowerUpComponent>>,
    mut ship_type_query: Query<(&mut Handle<Image>, &mut ShipComponent), With<PlayerComponent>>,
){
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    
    // Iterate through player
    for (player_tf, player_hitbox) in player_query.iter() {

        let player_scale = player_tf.scale.xy();

        // Iterate trough asteroids
        for (powerup_entity, powerup_tf, powerup_hitbox, powerup) in powerup_query.iter()
        {
            if despawned_entities.contains(&powerup_entity)
            {
                continue;
            }

            let powerup_scale = powerup_tf.scale.xy();

            let collision = collide(
                player_tf.translation,
                player_hitbox.0 * player_scale,
                powerup_tf.translation,
                powerup_hitbox.0 * powerup_scale,
            );

            // Check for collision
            if collision.is_some() {
                // Remove the asteroid
                commands.entity(powerup_entity).despawn();
                despawned_entities.insert(powerup_entity);

                for (mut texture_handle, mut ship_component) in ship_type_query.iter_mut() {
                    *ship_component = ShipComponent::new_type(powerup.get_ship_change_type());

                    // Load a new texture and update the handle
                    let new_texture_handle: Handle<Image> = match ship_component.ship_type {
                        ShipType::Attack => game_sprites.ship_type_attack.clone(),
                        ShipType::Shield => game_sprites.ship_type_shield.clone(),
                        ShipType::Normal => game_sprites.ship_type_normal.clone()
                    };

                    *texture_handle = new_texture_handle;
                }
            }
        }
    }
}

pub fn player_projectile_hit_asteroid_system(
    mut commands: Commands,
    projectile_query: Query<
        (Entity, &Transform, &HitBoxSize, &ProjectileDespawnComponent),
        With<ProjectileComponent>,
    >,
    meteor_query: Query<(Entity, &Transform, &HitBoxSize, &MeteorComponent), With<MeteorComponent>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // Iterate through player lasers
    for (proj_entity, proj_tf, proj_size, despawn_timer) in projectile_query.iter() {
        if despawned_entities.contains(&proj_entity) || despawn_timer.0.just_finished() {
            continue;
        }

        let laser_scale = proj_tf.scale.xy();

        // Iterate trough asteroids
        for (asteroid_entity, asteroid_tf, asteroid_size, asteroid) in meteor_query.iter() {
            if despawned_entities.contains(&asteroid_entity)
                || despawned_entities.contains(&proj_entity)
                || despawn_timer.0.just_finished()
            {
                continue;
            }

            let asteroid_scale = asteroid_tf.scale.xy();

            let collision = collide(
                proj_tf.translation,
                proj_size.0 * laser_scale,
                asteroid_tf.translation,
                asteroid_size.0 * asteroid_scale,
            );

            // Check for collision
            if collision.is_some() {
                // Remove the asteroid
                commands.entity(asteroid_entity).despawn();
                despawned_entities.insert(asteroid_entity);

                // Remove the player laser
                commands.entity(proj_entity).despawn();
                despawned_entities.insert(proj_entity);

                // Store position to spawn smaller asteroids
                if asteroid.size > 0 {
                    commands
                        .spawn(())
                        .insert(MeteorCollisionComponent {
                            size: asteroid.size - 1,
                            translation: asteroid_tf.translation.clone(),
                        })
                        .insert(Name::new("Meteor Collision"));
                }
            }
        }
    }
}