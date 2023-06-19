use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use bevy_tweening::{EaseFunction, lens::TransformPositionLens, Tween, Animator};
use lib::{
    PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_TURN_SPEED, 
    PLAYER_MAX_SPEED, PLAYER_SHOOT_COOLDOWN
};
use crate::{common_components::{
    Velocity, RotationAngle
}};

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn direction(&self, rotation_angle: f32) -> Vec2 {
        let (y, x) = (rotation_angle + PI / 2.0).sin_cos();
        Vec2::new(x, y)
    }
}

#[derive(Component)]
pub struct PlayerShootCooldownComponent(pub Timer);

impl Default for PlayerShootCooldownComponent {
    fn default() -> Self {
        Self(Timer::from_seconds(PLAYER_SHOOT_COOLDOWN, TimerMode::Once))
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement_system);
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerComponent, &mut Velocity, &mut RotationAngle)>,
){
    if let Ok((player, mut velocity, mut rotation_angle)) = query.get_single_mut() {
        // rotate the player ship
        if keyboard_input.pressed(KeyCode::Left) {
            rotation_angle.0 += PLAYER_TURN_SPEED;
        } else if keyboard_input.pressed(KeyCode::Right) {
            rotation_angle.0 -= PLAYER_TURN_SPEED;
        }

        // accelerate the ship towards the direction it's currently facing
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.0 += player.direction(rotation_angle.0) * PLAYER_ACCELERATION;

            if velocity.0.length() > PLAYER_MAX_SPEED {
                velocity.0 = velocity.0.normalize_or_zero() * PLAYER_MAX_SPEED;
            }
        } else if !keyboard_input.pressed(KeyCode::Up) {
            velocity.0 *= 1.0 - PLAYER_DECELERATION;
        }
    }
}

pub fn player_move_to_center(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &PlayerComponent)>
){

    if let Ok((entity, transform, _player)) = query.get_single_mut() {
        
        let tween: Tween<Transform> = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformPositionLens{
                start: transform.translation.clone(),
                end: Vec3::new(transform.translation.x.clone(), 1.0, transform.translation.z.clone())
            }
        );
        commands.entity(entity).insert(Animator::<Transform>::new(tween));

        print!("{}", "tesxt");
    }
}