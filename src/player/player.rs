use std::{f32::consts::PI, time::Duration};
use bevy::prelude::*;
use bevy_tweening::{EaseFunction, lens::{TransformPositionLens, TransformRotationLens}, Tween, Animator, Tracks};
use lib::{
    PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_TURN_SPEED, 
    PLAYER_MAX_SPEED, PLAYER_SHOOT_COOLDOWN, ShipType, PLAYER_SIZE, PLAYER_START_HP
};
use crate::{common::common_components::{
    Velocity, RotationAngle, HitBoxSize, Position, BoundsWarpable
}, resources::{SHIP_NORMAL_SPRITE, SHIP_SHIELD_SPRITE, SHIP_ATTACK_SPRITE, WindowSize}, utils::cleanup::{CleanUpEndGame}, events::events::{PlayerSpawnEvent, event_cleanup, PlayerDeadEvent, check_player_dead_event}, objects::projectile::projectile_shoot_system, state::states::GameStates};

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
        app
            // Start Game Systems
            .add_systems(OnEnter(GameStates::StartGame), player_spawn_system)
            // In Game Systems
            .add_systems(Update, player_movement_system.run_if(in_state(GameStates::InGame)))
            .add_systems(Update, projectile_shoot_system.run_if(in_state(GameStates::InGame)))
            .add_systems(Update, check_player_dead_event.run_if(in_state(GameStates::InGame)))
            .add_systems(Update, player_died_system.run_if(in_state(GameStates::InGame))
                .run_if(on_event::<PlayerDeadEvent>())
            )
            // End Game Systems
            .add_systems(OnEnter(GameStates::EndGame), player_move_out_of_screen_system);
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

fn player_died_system(
    mut commands: Commands,
    mut query: Query<(Entity, &PlayerComponent)>
){
    if let Ok((entity, _player)) = query.get_single_mut() {
        info!("remove components after death");
        commands.entity(entity).remove::<Velocity>();
        commands.entity(entity).remove::<RotationAngle>();
        commands.entity(entity).remove::<Position>();
    }
}

pub fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wdw_size: Res<WindowSize>,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
)
{
    ev_player_spawn.send(PlayerSpawnEvent);

    // create new ship component
    let new_ship_component = ShipComponent::new();

    let player_sprite = match new_ship_component.ship_type {
        ShipType::Attack => asset_server.load(SHIP_ATTACK_SPRITE),
        ShipType::Normal => asset_server.load(SHIP_NORMAL_SPRITE),
        ShipType::Shield => asset_server.load(SHIP_SHIELD_SPRITE),
    };

    let start_pos = Vec3::new(0.0, -wdw_size.h, 0.0);

    let tween: Tween<Transform> = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs(2),
        TransformPositionLens{
                start: start_pos,
                end: Vec3::new(0.0, 1.0, 0.0)
            }
    );
    let rot_tween: Tween<Transform> = Tween::new(
        EaseFunction::ElasticIn,
        Duration::from_secs(2),
        TransformRotationLens {
            start: Quat::from_rotation_z(0.0),
            end: Quat::from_rotation_z(0.0),
        },
    );
    let tracks: Tracks<Transform> = Tracks::new([tween, rot_tween]);

    info!("Spawn Player");
    // spawn player ship
    commands
        .spawn(SpriteBundle {
            texture: player_sprite,
            transform: Transform {
                translation: start_pos,
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
        .insert(BoundsWarpable())
        .insert(CleanUpEndGame::new(true))
        .insert(Animator::<Transform>::new(tracks));
}

pub fn player_fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &PlayerComponent)>,
    wdw_size: Res<WindowSize>
){
    if let Ok((entity, mut transform, _player)) = query.get_single_mut() {
        
        transform.rotation = Quat::from_rotation_z(0.0);

        let tween: Tween<Transform> = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformPositionLens{
                    start: transform.translation.clone(),
                    end: Vec3::new(0.0, wdw_size.h, transform.translation.z.clone())
                }
        );
        commands.entity(entity)
            .insert(Animator::<Transform>::new(tween));
            // .insert(CleanUpGameState::new(GameStates::EndGame, true));
    }
}

pub fn player_move_out_of_screen_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &PlayerComponent)>,
    wdw_size: Res<WindowSize>
){
    if let Ok((entity, mut transform, _player)) = query.get_single_mut() {
        
        transform.rotation = Quat::from_rotation_z(0.0);

        let tween: Tween<Transform> = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_secs(2),
            TransformPositionLens{
                    start: transform.translation.clone(),
                    end: Vec3::new(0.0, wdw_size.h, transform.translation.z.clone())
                }
        );
        commands.entity(entity)
            .insert(Animator::<Transform>::new(tween));
            // .insert(CleanUpGameState::new(GameStates::EndGame, true));
    }
}

pub fn clean_up_player_tween(
    mut commands: Commands,
    mut query: Query<(Entity, &PlayerComponent)>
){
    if let Ok((entity, _player)) = query.get_single_mut() {
        info!("Clean Up Tween");
        commands.entity(entity).remove::<Animator::<Transform>>();
    }
}