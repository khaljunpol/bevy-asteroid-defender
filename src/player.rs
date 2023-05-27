use std::f32::consts::PI;
use bevy::prelude::*;
use lib::{
    PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_TURN_SPEED, 
    PLAYER_MAX_SPEED
};
use crate::common_components::{
    Velocity, Position, RotationAngle
};

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn direction(&self, rotation_angle: f32) -> Vec2 {
        let (y, x) = (rotation_angle + PI / 2.0).sin_cos();

        Vec2::new(x, y)
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
    for (player, mut velocity, mut rotation_angle) in query.iter_mut() {

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