use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

use crate::{
    common_components::{Position, RotationAngle, Velocity},
    resources::{WindowSize},
    states::{InGameStatePlugin, GameStates},
    player::{PlayerComponent, PlayerPlugin}
};

mod player;
mod common_components;
mod common_systems;
mod resources;
mod states;

fn main() {
 App::new()
 .add_state::<GameStates>()
 .insert_resource(WindowSize {
    w: 1280.0,
    h: 720.0
 })
 .add_plugins(DefaultPlugins)
 .add_plugin(WorldInspectorPlugin::new())
 .add_plugin(InGameStatePlugin)
 .add_plugin(PlayerPlugin)
 .add_startup_system(startup_system)
 .run();
}

fn startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
)
{
    // get singleton window
    let window: &Window = window_query.get_single().unwrap();
    WindowResolution::new(1280.0, 720.0);
    let (win_w, win_h) = (window.width(), window.height());
    let (center_x, center_y) = (window.width() / 2.0, window.height() / 2.0);

    // spawn camera
    commands.spawn(Camera2dBundle{
        // transform: Transform::from_xyz(center_x, center_y, 0.0),
        ..default()
    });

    // add WinSize resource
    let win_size = WindowSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // spawn player ship
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(resources::PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(center_x, center_y, 0.0),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(PlayerComponent)
        .insert(Velocity(Vec2::splat(0.0)))
        .insert(Position(Vec2::splat(0.0)))
        .insert(RotationAngle(0.0));
}