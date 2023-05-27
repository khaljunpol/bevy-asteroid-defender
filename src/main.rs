use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod player;
mod geom;

fn main() {
 App::new()
 .add_plugins(DefaultPlugins)
 .add_startup_system(spawn_player)
 .add_startup_system(spawn_camera)
 .add_system(player::player_movement)
 .add_system(player::confine_player_movement)
 .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window: &Window = window_query.get_single().unwrap();
    let pos: Vec2 = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    commands.spawn((
        player::spawn_player(asset_server, pos),
        player::Player::new(pos)
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}