use std::f32::consts::PI;
use bevy::prelude::*;
use lib::{POWER_UP_SIZE, POWERUP_MAX_COUNT, BORDER_EXTRA_SPACE};
use rand::{
    prelude::*
};
use crate::{
    common_components::{Position, RotationAngle, Velocity, HitBoxSize, BoundsDespawnable, BoundsWarpable},
    resources::{WindowSize, GameSprites, WindowDespawnBorder}, powerup::PowerUpComponent
};

pub fn movement_system(
    mut query: Query<(&Velocity, &mut Position)>
) {
    for (velocity, mut position) in query.iter_mut() {
        let mut new_position = position.0 + velocity.0;

        position.0 = new_position;
    }
}

pub fn warp_if_reached_window_bounds_system(
    mut query: Query<(&mut Position, &Transform, With<BoundsWarpable>)>,
    wdw_size: Res<WindowSize>
) {
    // values containing each corner of the screen
    let right_side = wdw_size.w / 2.0;
    let left_side = -right_side;
    let top = wdw_size.h / 2.0;
    let bottom = -top;

    for (mut position, transform, warpable) in query.iter_mut() {
        let mut new_position = position.0;
        let half_scale = transform.scale.max_element();


        if new_position.x > right_side + half_scale {
            new_position.x = left_side - half_scale;
        } else if new_position.x < left_side - half_scale {
            new_position.x = right_side + half_scale;
        }

        if new_position.y > top + half_scale {
            new_position.y = bottom - half_scale;
        } else if new_position.y < bottom - half_scale {
            new_position.y = top + half_scale;
        }

        position.0 = new_position;
    }
}

pub fn despawn_if_reached_bounds_system(
    mut commands: Commands,
    mut despawnable_query: Query<(Entity, &Velocity, &mut Position, With<BoundsDespawnable>)>,
    border_size: Res<WindowDespawnBorder>
) {

    for(entity, velocity, mut position, despawnable) in despawnable_query.iter(){
        let mut new_position = position.0 + velocity.0;

        let mut shouldDespawn = false;

        if new_position.x > border_size.right {        
            shouldDespawn = true;
        } else if new_position.x < border_size.left {
            shouldDespawn = true;
        }

        if new_position.y > border_size.top {
            shouldDespawn = true;
        } else if new_position.y < border_size.bottom {
            shouldDespawn = true;
        }
        
        if shouldDespawn {
            commands.entity(entity).despawn();
            break;
        }
    }

}

pub  fn update_transform_system(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.0.x, position.0.y, transform.translation.z);
    }
}

pub fn update_rotation_system(mut query: Query<(&RotationAngle, &mut Transform)>) {
    for (rotation_angle, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(rotation_angle.0);
    }
}

pub fn spawn_powerup_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<With<PowerUpComponent>>,
)
{
    let mut count = 0;
    for _ in query.iter() {
        count += 1;
    }
    
    let center = Vec2::new(wdw_size.w / 2.0, wdw_size.h / 2.0);
    let mut rng = thread_rng();

    if count < POWERUP_MAX_COUNT {
        let x_pos_rand = rng.gen_range(-center.x..center.x);
        let y_pos_rand = if rng.gen_bool(0.5) { -1 } else { 1 } as f32;

    
        let position = Vec2::new(
            x_pos_rand,  
            y_pos_rand * (center.y + 50.0));
    
        // randomizing the starting rotation angle of the powerups
        let rotation = rng.gen_range(-0.1..0.1) as f32;
    
        // randomizing rotation speed
        let rot_speed =
            rng.gen_range(-0.1..0.1) as f32;

        // randomizing movement speed
        let mut x_speed = rng.gen_range(-1.5..1.5);
        let mut y_speed = 0.0;

        if position.y > center.y{
            y_speed = rng.gen_range(-1.5..-1.0);
        } else if position.y < center.y{
            y_speed = rng.gen_range(1.0..1.5);
        }

        let mut speed = Vec2::new(x_speed, y_speed);
    
        let powerup_position = Vec3::new(position.x, position.y, 1.0);
    
        commands
            .spawn(SpriteBundle {
                texture: game_sprites.powerup_change_normal.clone(),
                transform: Transform {
                    translation: powerup_position,
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(1.0, 1.0 ,1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Power Up"))
            .insert(PowerUpComponent::new(rot_speed))
            .insert(HitBoxSize(POWER_UP_SIZE))
            .insert(Velocity(Vec2::from(speed)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(rotation))
            .insert(BoundsDespawnable());
    }
}
