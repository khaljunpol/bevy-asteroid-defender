use bevy::{prelude::*, 
    sprite::collide_aabb::collide, 
    math::Vec3Swizzles};
use lib::{ShipType, MeteorSizeType};
use rand::{thread_rng, Rng};

use std::collections::HashSet;

use crate::{
    common::common_components::{HitBoxSize, CollisionDespawnableWithDamage, DamageCollision}, 
    player::{
        player::PlayerComponent,
        ship::ShipComponent,
    },
    objects::{
        powerup::PowerUpComponent,
        meteor::{MeteorComponent, spawn_meteor}, projectile::{ProjectileDespawnComponent, ProjectileComponent}
    },
    resources::{GameSprites, Life}, state::states::{GameStates}, events::events::PlayerDeadEvent
};

use super::common_components::{MeteorCollision};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (meteor_collision_spawn_system, 
                collision_damage_system,
                player_collide_powerup_system,
                player_projectile_hit_meteor_system,
                player_collide_despawnable_system)
                .run_if(in_state(GameStates::InGame)));
    }

}


fn player_collide_powerup_system(
    game_sprites: Res<GameSprites>,
    player_query: Query<(&Transform, &HitBoxSize), With<PlayerComponent>>,
    powerup_query: Query<(&Transform, &HitBoxSize, &PowerUpComponent), With<PowerUpComponent>>,
    mut ship_type_query: Query<(&mut Handle<Image>, &mut ShipComponent), With<PlayerComponent>>,
){
    // Iterate through player
    for (player_tf, player_hitbox) in player_query.iter() {

        let player_scale = player_tf.scale.xy();

        // Iterate trough asteroids
        for (powerup_tf, powerup_hitbox, powerup) in powerup_query.iter()
        {
            let powerup_scale = powerup_tf.scale.xy();

            let collision = collide(
                player_tf.translation,
                player_hitbox.0 * player_scale,
                powerup_tf.translation,
                powerup_hitbox.0 * powerup_scale,
            );

            // Check for collision
            if collision.is_some() {
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

fn player_collide_despawnable_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &HitBoxSize), With<PlayerComponent>>,
    collision_query: Query<(Entity, &Transform, &HitBoxSize, &CollisionDespawnableWithDamage), With<CollisionDespawnableWithDamage>>,
){
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    
    // Iterate through player
    for (player_tf, player_hitbox) in player_query.iter_mut() {

        let player_scale = player_tf.scale.xy();

         // Iterate trough asteroids
         for (despawnable_entity, despawnable_transform, despawnable_size, despawnable) in collision_query.iter() {
            if despawned_entities.contains(&despawnable_entity)
            {
                continue;
            }

            let asteroid_scale = despawnable_transform.scale.xy();

            let collision = collide(
                player_tf.translation,
                player_hitbox.0 * player_scale,
                despawnable_transform.translation,
                despawnable_size.0 * asteroid_scale,
            );

            // Check for collision
            if collision.is_some() {
                // Remove the asteroid
                commands.entity(despawnable_entity).despawn();
                despawned_entities.insert(despawnable_entity);

                // Spawn damage component to be queried into damage computation system
                if despawnable.should_damage {
                    commands
                    .spawn(())
                    .insert(DamageCollision(despawnable.damage))
                    .insert(Name::new("Collision Damage"));
                }
            }
        }
    }
}

fn collision_damage_system(
    mut commands: Commands,
    damage_query: Query<(Entity, &DamageCollision)>,
    mut ev_played_dead: EventWriter<PlayerDeadEvent>,
    mut life: ResMut<Life>
) {
    // Iterate through player
    for (entity, damage_collision) in damage_query.iter() {
        // deduct damage value from damage component
        life.current_life -= damage_collision.0;

        println!("{:?}", life.current_life);

        // despawn DamageCollisionDespawnable entity
        commands.entity(entity).despawn();

        if life.current_life <= 0 {
            life.current_life = 0;
            ev_played_dead.send(PlayerDeadEvent);
            break;
        }
    }
}

fn player_projectile_hit_meteor_system(
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

                let size = asteroid.size as i32;
                
                // Store position to spawn smaller asteroids
                if size > 0 {
                    commands
                    .spawn(())
                    .insert(MeteorCollision {
                        size: size - 1,
                        translation: asteroid_tf.translation.clone(),
                    })
                    .insert(Name::new("Meteor Collision"));
                }
                
            }
        }
    }
}

fn meteor_collision_spawn_system(
    mut commands: Commands,
    mut game_sprites: Res<GameSprites>,
    query: Query<(Entity, &MeteorCollision)>,
) {
    let mut rng = thread_rng();

    for (entity, collision) in query.iter() {
        if collision.size > 0 {
            // split the into smaller pieces
            for i in 1..4 {
                // randomizing rotation angle
                let randomized_rotation_angle = rng.gen_range(-1.0..1.0);

                let speed = i as f32 * 0.75;

                // randomizing movement speed
                let speed = Vec2::new(rng.gen_range(-speed..speed), rng.gen_range(-speed..speed));

                // randomizing rotation speed
                let rotation_speed =
                    rng.gen_range(-0.05..0.05);

                // convert i32 to eum
                let new_spawn_size = match collision.size {
                    1 => MeteorSizeType::Small,
                    2 => MeteorSizeType::Medium,
                    3 => MeteorSizeType::Large,
                    _ => MeteorSizeType::Large
                }; 

                spawn_meteor(
                    &mut commands,
                    &mut game_sprites,
                    new_spawn_size,
                    collision.translation,
                    Vec2::new(collision.translation.x, collision.translation.y),
                    randomized_rotation_angle,
                    rotation_speed,
                    speed,
                    Vec2::new(50.0, 50.0)
                ); 
            }
        }

        // despawn MeteorCollisionComponent entity
        commands.entity(entity).despawn();
    }
}