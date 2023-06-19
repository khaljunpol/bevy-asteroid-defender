use bevy::prelude::*;
use crate::{
    common_components::{Position, RotationAngle, Velocity, BoundsDespawnable, BoundsWarpable, BoundsDespawnableWithTimer},
    resources::{WindowSize, WindowDespawnBorder}
};

pub fn movement_system(
    mut query: Query<(&Velocity, &mut Position)>
) {
    for (velocity, mut position) in query.iter_mut() {
        let new_position = position.0 + velocity.0;

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

    for (mut position, transform, _warpable) in query.iter_mut() {
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
    despawnable_query: Query<(Entity, &Velocity, &Position, &BoundsDespawnable)>,
    border_size: Res<WindowDespawnBorder>
) {

    for(entity, velocity, position, despawnable) in despawnable_query.iter(){
        let new_position = position.0 + velocity.0;

        let mut should_despawn = false;

        if new_position.x > border_size.right + despawnable.0.x {        
            should_despawn = true;
        } else if new_position.x < border_size.left - despawnable.0.x {
            should_despawn = true;
        }

        if new_position.y > border_size.top + despawnable.0.y {
            should_despawn = true;
        } else if new_position.y < border_size.bottom - despawnable.0.y {
            should_despawn = true;
        }
        
        if should_despawn {
            commands.entity(entity).despawn();
            break;
        }
    }

}


pub fn despawn_if_reached_bounds_timer_system(
    mut commands: Commands,
    mut despawnable_query: Query<(Entity, &Velocity, &Position, &mut BoundsDespawnableWithTimer)>,
    time: Res<Time>,
    border_size: Res<WindowDespawnBorder>
) {

    for(entity, velocity, position, mut despawnable) in despawnable_query.iter_mut(){
        despawnable.initial_spawn_timer.tick(time.delta());

        if despawnable.initial_spawn_timer.finished(){
                
            let new_position = position.0 + velocity.0;

            let mut should_despawn = false;

            if new_position.x > border_size.right + despawnable.bounds.0.x {        
                should_despawn = true;
            } else if new_position.x < border_size.left - despawnable.bounds.0.x {
                should_despawn = true;
            }

            if new_position.y > border_size.top + despawnable.bounds.0.y {
                should_despawn = true;
            } else if new_position.y < border_size.bottom - despawnable.bounds.0.y {
                should_despawn = true;
            }

            if should_despawn && !despawnable.should_despawn {
                despawnable.should_despawn = true;
                despawnable.despawn_timer.reset();
            }
        }

        if despawnable.should_despawn {
            despawnable.despawn_timer.tick(time.delta());

            if despawnable.despawn_timer.finished(){
                commands.entity(entity).despawn();
                break;
            }
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