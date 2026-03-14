use bevy::prelude::*;
use crate::{
    common::common_components::{Position, RotationAngle, Velocity, BoundsDespawnable, BoundsWarpable},
    resources::{WindowSize, WindowDespawnBorder},
};

pub fn warp_if_reached_window_bounds_system(
    mut query:   Query<(&mut Position, &Transform), With<BoundsWarpable>>,
    wdw_size:    Res<WindowSize>,
) {
    let right  =  wdw_size.w / 2.0;
    let left   = -right;
    let top    =  wdw_size.h / 2.0;
    let bottom = -top;

    for (mut position, transform) in &mut query {
        let margin = transform.scale.max_element();
        let p = &mut position.0;

        if p.x > right  + margin { p.x = left   - margin; }
        else if p.x < left   - margin { p.x = right  + margin; }

        if p.y > top    + margin { p.y = bottom - margin; }
        else if p.y < bottom - margin { p.y = top    + margin; }
    }
}

pub fn despawn_if_reached_bounds_system(
    mut commands: Commands,
    query:        Query<(Entity, &Velocity, &Position, &BoundsDespawnable)>,
    border:       Res<WindowDespawnBorder>,
) {
    for (entity, vel, pos, despawnable) in &query {
        let next = pos.0 + vel.0;

        let out_of_bounds =
            next.x > border.right  + despawnable.0.x
            || next.x < border.left   - despawnable.0.x
            || next.y > border.top    + despawnable.0.y
            || next.y < border.bottom - despawnable.0.y;

        if out_of_bounds {
            commands.entity(entity).despawn();
        }
    }
}

pub fn movement_system(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in &mut query {
        pos.0 += vel.0;
    }
}

pub fn update_transform_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut tf) in &mut query {
        tf.translation.x = pos.0.x;
        tf.translation.y = pos.0.y;
    }
}

pub fn update_rotation_system(mut query: Query<(&RotationAngle, &mut Transform)>) {
    for (angle, mut tf) in &mut query {
        tf.rotation = Quat::from_rotation_z(angle.0);
    }
}
