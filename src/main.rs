use bevy::{prelude::*, core_pipeline::bloom::BloomSettings, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, window::PresentMode};
use bevy_framepace::{FramepacePlugin, FramePaceStats, FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use bevy_tweening::TweeningPlugin;
use bevy_hanabi::prelude::*;
use common::collision::CollisionPlugin;
use lib::{BORDER_EXTRA_SPACE, PLAYER_START_HP, MAX_FRAMERATE};
use resources::{Life, LIFE_NORMAL_SPRITE, LIFE_ATTACK_SPRITE, LIFE_SHIELD_SPRITE};
use state::states::{ProgressionStatePlugin, EndGameStatePlugin, BaseStatePlugin};
use ui::ui::UIPlugin;

use crate::{
    resources::WindowSize,
    objects::{
        powerup::PowerUpPlugin,
        meteor::MeteorPlugin,
        projectile::ProjectilePlugin
    },
    events::events::EventsPlugin,
    state::states::
        {
            InGameStatePlugin, StartGameStatePlugin, GameStates
        },
    player::{
        player::PlayerPlugin,
        ship::ShipPlugin,
    },
    resources::{
        GameSprites, SHIP_ATTACK_SPRITE,SHIP_NORMAL_SPRITE,SHIP_SHIELD_SPRITE,
        POWERUP_CHANGE_NORMAL_SPRITE, POWERUP_CHANGE_ATTACK_SPRITE, POWERUP_CHANGE_SHIELD_SPRITE,
        PROJECTILE_NORMAL_SPRITE, PROJECTILE_ATTACK_SPRITE, PROJECTILE_SHIELD_SPRITE,
        WindowDespawnBorder, METEOR_BIG_SPRITE, METEOR_MED_SPRITE, METEOR_SML_SPRITE
    },
};

mod player;
mod objects;

mod common;
mod background;
mod utils;
mod effects;

mod resources;
mod state;
mod events;
mod ui;

fn main() {
 App::new()
 .add_state::<GameStates>()
 .add_plugins(DefaultPlugins
    .set(WindowPlugin{
        primary_window: Some(Window { 
            title: "Asteroid Defender Rougelike".into(),
            resolution: (1280.0, 720.0).into(),
            resizable: false,
            // present_mode: PresentMode::Fifo,
            decorations: false,
            ..default()
        }),
        ..default()
    }))
 .add_plugins(WorldInspectorPlugin::new())
 .add_plugins(FramepacePlugin)
 .add_plugins(LogDiagnosticsPlugin::default())
 .add_plugins(FrameTimeDiagnosticsPlugin::default())
 .add_plugins(TweeningPlugin)
 .add_plugins(HanabiPlugin)
 .add_plugins(BaseStatePlugin)
 .add_plugins(StartGameStatePlugin)
 .add_plugins(InGameStatePlugin)
 .add_plugins(ProgressionStatePlugin)
 .add_plugins(EndGameStatePlugin)
 .add_plugins(PlayerPlugin)
 .add_plugins(ShipPlugin)
 .add_plugins(CollisionPlugin)
 .add_plugins(PowerUpPlugin)
 .add_plugins(MeteorPlugin)
 .add_plugins(ProjectilePlugin)
 .add_plugins(EventsPlugin)
 .add_plugins(UIPlugin)
 .add_systems(PreStartup, startup_system)
 .run();
}

fn startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut frame_setting: ResMut<FramepaceSettings>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
)
{
    // limit framerate
    frame_setting.limiter = Limiter::from_framerate(MAX_FRAMERATE);

    // get singleton window
    let window: &Window = window_query.get_single().unwrap();
    let (wdw_w, wdw_h) = (window.width(), window.height());
    let (center_x, center_y) = (window.width() / 2.0, window.height() / 2.0);

    // spawn camera
    commands.spawn((Camera2dBundle
        {
        ..default()
        }, 
    BloomSettings::default()));

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
        life_normal: asset_server.load(LIFE_NORMAL_SPRITE),
        life_attack: asset_server.load(LIFE_ATTACK_SPRITE),
        life_shield: asset_server.load(LIFE_SHIELD_SPRITE),
        meteor_big: asset_server.load(METEOR_BIG_SPRITE),
        meteor_med: asset_server.load(METEOR_MED_SPRITE),
        meteor_sml: asset_server.load(METEOR_SML_SPRITE),
    };
    commands.insert_resource(game_sprites);

    // add in game values
    commands.insert_resource(Life::new(PLAYER_START_HP))
}