use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use lib;
use crate::geom;

pub const PLAYER_SIZE: f32 = 100.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_TURN_SPEED: f32 = 30.0;

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    pub dir: geom::MoveDirection
}

impl Player {
    pub fn new(pos: Vec2) -> Player {
        Player { 
            pos: pos, 
            dir: geom::MoveDirection::Right
        }
    }
}

pub fn spawn_player(asset_server: Res<AssetServer>, pos: Vec2) -> SpriteBundle {
    SpriteBundle{
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        texture: asset_server.load("sprites/ships/playerShip1_blue.png"),
        ..default()
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if(direction.length() > 0.0)
        {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(-PLAYER_TURN_SPEED * PLAYER_TURN_SPEED * time.delta_seconds()));
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
){

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}