use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use player::{PlayerComponent, PlayerPlugin};
use common_components::{Position, RotationAngle, Velocity};

mod player;
mod common_components;
mod resources;

fn main() {
 App::new()
 .add_plugins(DefaultPlugins)
 .add_plugin(WorldInspectorPlugin::new())
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
    let window: &Window = window_query.get_single().unwrap();

    // commands.spawn(Camera2dBundle{
    //     transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(resources::PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 10.),
                // scale: Vec3::new(0.5, 0.5, 1.),
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