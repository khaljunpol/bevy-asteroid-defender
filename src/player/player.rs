use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use bevy_tweening::{EaseFunction, lens::TransformPositionLens, Tween, Animator};
use lib::{
    PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_TURN_SPEED, 
    PLAYER_MAX_SPEED, PLAYER_SHOOT_COOLDOWN, ShipType, PLAYER_SIZE
};
use crate::{common::common_components::{
    Velocity, RotationAngle, HitBoxSize, Position, BoundsWarpable
}, resources::{SHIP_NORMAL_SPRITE, SHIP_SHIELD_SPRITE, SHIP_ATTACK_SPRITE, WindowSize}};

use super::ship::ShipComponent;

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
        app.add_systems(Update, player_movement_system);
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

pub fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wdw_size: Res<WindowSize>
)
{
    // create new ship component
    let new_ship_component = ShipComponent::new();

    let player_sprite = match new_ship_component.ship_type {
        ShipType::Attack => asset_server.load(SHIP_ATTACK_SPRITE),
        ShipType::Normal => asset_server.load(SHIP_NORMAL_SPRITE),
        ShipType::Shield => asset_server.load(SHIP_SHIELD_SPRITE),
    };

    // spawn player ship
    commands
        .spawn(SpriteBundle {
            texture: player_sprite,
            transform: Transform {
                translation: Vec3::new(0.0, -wdw_size.h, 0.0),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(PlayerComponent)
        .insert(new_ship_component)
        .insert(PlayerShootCooldownComponent::default())
        .insert(HitBoxSize(PLAYER_SIZE))
        .insert(Velocity(Vec2::splat(0.0)))
        .insert(Position(Vec2::splat(0.0)))
        .insert(RotationAngle(0.0))
        .insert(BoundsWarpable());
}

pub fn player_move_to_center_system(
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
    }
}