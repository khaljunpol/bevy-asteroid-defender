use bevy::prelude::*;
use rand::{
    prelude::*
};
use crate::{
    common_components::{Position, RotationAngle, Velocity},
    resources::{WindowSize, GameSprites}, powerup::PowerUpComponent
};

pub fn movement_system(
    mut query: Query<(&Velocity, &mut Position, &Transform)>,
    wdw_size: Res<WindowSize>,
) {
    // values containing each corner of the screen
    let right_side = wdw_size.w / 2.0;
    let left_side = -right_side;
    let top = wdw_size.h / 2.0;
    let bottom = -top;

    for (velocity, mut position, transform) in query.iter_mut() {
        let mut new_position = position.0 + velocity.0;
        let half_scale = transform.scale.max_element();

        // screen wrapping
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
)
{
    let mut rng = thread_rng();
    let max_dist = wdw_size.w.min(wdw_size.h) / 2.0;
    let min_dist = 320.0;

    let dist_range = min_dist..max_dist;
    let angle_range = 0.0..50.0 as f32;

    let angles = vec![
        (
            rng.gen_range(angle_range.clone()),
            rng.gen_range(dist_range.clone()),
        ),
        (
            rng.gen_range(angle_range.clone()) + 120.0,
            rng.gen_range(dist_range.clone()),
        ),
        (
            rng.gen_range(angle_range.clone()) + 240.0,
            rng.gen_range(dist_range.clone()),
        )
    ];

    for (angle, dist) in angles.iter() {
        // calculating coordinates to spawn
        let (x, y) = angle.to_radians().sin_cos();
        let position = Vec2::new(x * dist, y * dist);

        // randomizing the starting rotation angle of the asteroids
        let rotation = rng.gen_range(-0.1..0.1) as f32;

        // randomizing movement speed
        let speed = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        // randomizing rotation speed
        let rot_speed =
            rng.gen_range(-0.1..0.1) as f32;

        commands
            .spawn(SpriteBundle {
                texture: game_sprites.powerup_change_normal.clone(),
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 10.),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(1.0, 1.0 ,1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Power Up"))
            .insert(PowerUpComponent::new(rot_speed))
            .insert(Velocity(Vec2::from(speed)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(rotation));
    }
}
