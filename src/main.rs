use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use lib::{ShipType, PLAYER_SIZE, BORDER_EXTRA_SPACE};
use meteor::MeteorPlugin;

use crate::{
    common_components::{Position, RotationAngle, Velocity, HitBoxSize, BoundsWarpable},
    resources::{WindowSize},
    powerup::PowerUpPlugin,
    states::{InGameStatePlugin, GameStates},
    player::{PlayerComponent, PlayerPlugin},
    ship::{ShipPlugin, ShipComponent},
    resources::{
        GameSprites, SHIP_ATTACK_SPRITE,SHIP_NORMAL_SPRITE,SHIP_SHIELD_SPRITE,
        POWERUP_CHANGE_NORMAL_SPRITE, POWERUP_CHANGE_ATTACK_SPRITE, POWERUP_CHANGE_SHIELD_SPRITE,
        PROJECTILE_NORMAL_SPRITE, PROJECTILE_ATTACK_SPRITE, PROJECTILE_SHIELD_SPRITE,
        WindowDespawnBorder, METEOR_BIG_SPRITE, METEOR_MED_SPRITE, METEOR_SML_SPRITE
    },
    projectile::ProjectilePlugin,
    player::PlayerShootCooldownComponent
};

mod player;
mod ship;
mod powerup;
mod projectile;
mod meteor;

mod common_components;
mod common_systems;
mod collision;
mod utils;

mod resources;
mod states;

fn main() {
 App::new()
 .add_state::<GameStates>()
 .add_plugins(DefaultPlugins)
 .add_plugin(WorldInspectorPlugin::new())
 .add_plugin(InGameStatePlugin)
 .add_plugin(PlayerPlugin)
 .add_plugin(ShipPlugin)
 .add_plugin(PowerUpPlugin)
 .add_plugin(MeteorPlugin)
 .add_plugin(ProjectilePlugin)
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
    let (wdw_w, wdw_h) = (window.width(), window.height());
    let (center_x, center_y) = (window.width() / 2.0, window.height() / 2.0);

    // spawn camera
    commands.spawn(Camera2dBundle{
        ..default()
    });

    // add WinSize resource
    let wdw_size = WindowSize { w: wdw_w, h: wdw_h };
    commands.insert_resource(wdw_size);

    // add Despawn Border resource
    let despawn_border = WindowDespawnBorder {
        top: center_y + BORDER_EXTRA_SPACE,
        bottom: -center_y - BORDER_EXTRA_SPACE,
        left: -center_x - BORDER_EXTRA_SPACE,
        right: center_x + BORDER_EXTRA_SPACE,
    };
    commands.insert_resource(despawn_border);

    // add GameSprites resource
    let game_sprites = GameSprites {
        ship_type_attack: asset_server.load(SHIP_ATTACK_SPRITE),
        ship_type_normal: asset_server.load(SHIP_NORMAL_SPRITE),
        ship_type_shield: asset_server.load(SHIP_SHIELD_SPRITE),
        powerup_change_normal: asset_server.load(POWERUP_CHANGE_NORMAL_SPRITE),
        powerup_change_attack: asset_server.load(POWERUP_CHANGE_ATTACK_SPRITE),
        powerup_change_shield: asset_server.load(POWERUP_CHANGE_SHIELD_SPRITE),
        projectile_normal: asset_server.load(PROJECTILE_NORMAL_SPRITE),
        projectile_attack: asset_server.load(PROJECTILE_ATTACK_SPRITE),
        projectile_shield: asset_server.load(PROJECTILE_SHIELD_SPRITE),
        meteor_big: asset_server.load(METEOR_BIG_SPRITE),
        meteor_med: asset_server.load(METEOR_MED_SPRITE),
        meteor_sml: asset_server.load(METEOR_SML_SPRITE),
    };
    commands.insert_resource(game_sprites);

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
                translation: Vec3::new(center_x, center_y, 0.0),
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