use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use lib::ShipType;
use rand::{
    prelude::*
};

use std::collections::HashSet;

use crate::{
    common_components::{HitBoxSize}, 
    player::PlayerComponent, 
    powerup::PowerUpComponent, resources::{GameSprites}, 
    ship::ShipComponent
};

pub fn player_collide_powerup_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    player_query: Query<(&Transform, &HitBoxSize), With<PlayerComponent>>,
    powerup_query: Query<(Entity, &Transform, &HitBoxSize, &PowerUpComponent), With<PowerUpComponent>>,
    mut ship_type_query: Query<(&mut Handle<Image>, &mut ShipComponent), With<PlayerComponent>>,
){
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    let mut rng = thread_rng();
    
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