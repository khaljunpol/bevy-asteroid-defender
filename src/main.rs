use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_tweening::TweeningPlugin;

use lib::{BORDER_EXTRA_SPACE, PLAYER_START_HP, PLAYER_START_SCORE, MAX_FRAMERATE};
use resources::{
    CameraShake, GameSprites, IsPaused, Life, Score, WindowSize, WindowDespawnBorder,
    LevelResource, PlayerUpgrades, UpgradeSelectionState, ShipSelectState, PlayerBuff,
    SHIP_NORMAL_SPRITE, SHIP_ATTACK_SPRITE, SHIP_SHIELD_SPRITE,
    POWERUP_HP_SPRITE, POWERUP_HP_SPRITE_GREEN, POWERUP_HP_SPRITE_RED,
    POWERUP_BOLT_SPRITE, POWERUP_BOLT_SPRITE_GREEN, POWERUP_BOLT_SPRITE_RED,
    POWERUP_SHIELD_SPRITE, POWERUP_SHIELD_SPRITE_GREEN, POWERUP_SHIELD_SPRITE_RED,
    SHIELD_EFFECT_SPRITE,
    PROJECTILE_NORMAL_SPRITE, PROJECTILE_ATTACK_SPRITE, PROJECTILE_SHIELD_SPRITE,
    LIFE_NORMAL_SPRITE, LIFE_ATTACK_SPRITE, LIFE_SHIELD_SPRITE,
    METEOR_BIG_SPRITE, METEOR_MED_SPRITE, METEOR_SML_SPRITE,
    STAR1_SPRITE, STAR2_SPRITE, STAR3_SPRITE, SPEED_SPRITE, UFO_SPRITE,
    UFO_BLUE_SPRITE, UFO_GREEN_SPRITE, UFO_YELLOW_SPRITE,
};
use state::states::{
    GameStates, BaseStatePlugin, ShipSelectStatePlugin, StartGameStatePlugin, CountdownStatePlugin,
    InGameStatePlugin, LevelCompleteStatePlugin, UpgradeSelectionStatePlugin,
    GameOverStatePlugin,
};

mod player;
mod objects;
mod common;
mod background;
mod effects;
mod resources;
mod state;
mod events;
mod ui;
mod upgrades;
mod utils;

fn main() {
    let mut app = App::new();

    app
        .add_state::<GameStates>()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroid Defender Roguelike".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    // Attach to the <canvas id="bevy"> element when running as WASM.
                    #[cfg(target_arch = "wasm32")]
                    canvas: Some("#bevy".into()),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(TweeningPlugin)
        // State machine
        .add_plugins(BaseStatePlugin)
        .add_plugins(ShipSelectStatePlugin)
        .add_plugins(StartGameStatePlugin)
        .add_plugins(CountdownStatePlugin)
        .add_plugins(InGameStatePlugin)
        .add_plugins(LevelCompleteStatePlugin)
        .add_plugins(UpgradeSelectionStatePlugin)
        .add_plugins(GameOverStatePlugin)
        // Game systems
        .add_plugins(player::player::PlayerPlugin)
        .add_plugins(player::ship::ShipPlugin)
        .add_plugins(common::collision::CollisionPlugin)
        .add_plugins(objects::meteor::MeteorPlugin)
        .add_plugins(objects::projectile::ProjectilePlugin)
        .add_plugins(objects::powerup::PowerUpPlugin)
        .add_plugins(objects::ufo::UfoPlugin)
        .add_plugins(events::events::EventsPlugin)
        .add_plugins(ui::ui::UIPlugin)
        .add_plugins(upgrades::upgrades::UpgradePlugin)
        // Visual polish
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(effects::particle::ParticlePlugin)
        .add_plugins(effects::shake::CameraShakePlugin)
        // Startup
        .add_systems(PreStartup, startup_system);

    // Frame limiter: desktop only (WASM uses browser vsync).
    #[cfg(not(target_arch = "wasm32"))]
    {
        use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
        app.add_plugins(FramepacePlugin);
        app.add_systems(Startup, |mut s: ResMut<FramepaceSettings>| {
            s.limiter = Limiter::from_framerate(MAX_FRAMERATE);
        });
    }

    app.run();
}

fn startup_system(
    mut commands:  Commands,
    asset_server:  Res<AssetServer>,
    window_query:  Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (w, h) = (window.width(), window.height());
    let (cx, cy) = (w / 2.0, h / 2.0);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Window resources
    commands.insert_resource(WindowSize { w, h });
    commands.insert_resource(WindowDespawnBorder {
        top:    cy + BORDER_EXTRA_SPACE,
        bottom: -cy - BORDER_EXTRA_SPACE,
        left:   -cx - BORDER_EXTRA_SPACE,
        right:   cx + BORDER_EXTRA_SPACE,
    });

    // Preload all sprite assets
    commands.insert_resource(GameSprites {
        ship_type_normal:  asset_server.load(SHIP_NORMAL_SPRITE),
        ship_type_attack:  asset_server.load(SHIP_ATTACK_SPRITE),
        ship_type_shield:  asset_server.load(SHIP_SHIELD_SPRITE),
        // HP powerup tiers
        powerup_hp:            asset_server.load(POWERUP_HP_SPRITE),
        powerup_hp_green:      asset_server.load(POWERUP_HP_SPRITE_GREEN),
        powerup_hp_red:        asset_server.load(POWERUP_HP_SPRITE_RED),
        // Bolt powerup tiers
        powerup_bolt:          asset_server.load(POWERUP_BOLT_SPRITE),
        powerup_bolt_green:    asset_server.load(POWERUP_BOLT_SPRITE_GREEN),
        powerup_bolt_red:      asset_server.load(POWERUP_BOLT_SPRITE_RED),
        // Shield powerup tiers
        powerup_shield:        asset_server.load(POWERUP_SHIELD_SPRITE),
        powerup_shield_green:  asset_server.load(POWERUP_SHIELD_SPRITE_GREEN),
        powerup_shield_red:    asset_server.load(POWERUP_SHIELD_SPRITE_RED),
        // Shield visual effect
        shield_effect:         asset_server.load(SHIELD_EFFECT_SPRITE),
        projectile_normal: asset_server.load(PROJECTILE_NORMAL_SPRITE),
        projectile_attack: asset_server.load(PROJECTILE_ATTACK_SPRITE),
        projectile_shield: asset_server.load(PROJECTILE_SHIELD_SPRITE),
        life_normal:       asset_server.load(LIFE_NORMAL_SPRITE),
        life_attack:       asset_server.load(LIFE_ATTACK_SPRITE),
        life_shield:       asset_server.load(LIFE_SHIELD_SPRITE),
        meteor_big:        asset_server.load(METEOR_BIG_SPRITE),
        meteor_med:        asset_server.load(METEOR_MED_SPRITE),
        meteor_sml:        asset_server.load(METEOR_SML_SPRITE),
        // Effects
        star1:             asset_server.load(STAR1_SPRITE),
        star2:             asset_server.load(STAR2_SPRITE),
        star3:             asset_server.load(STAR3_SPRITE),
        fire_frames:       (0..20)
                               .map(|i| asset_server.load(format!("sprites/effects/fire{i:02}.png")))
                               .collect(),
        speed:             asset_server.load(SPEED_SPRITE),
        // Enemies
        ufo:               asset_server.load(UFO_SPRITE),
        ufo_blue:          asset_server.load(UFO_BLUE_SPRITE),
        ufo_green:         asset_server.load(UFO_GREEN_SPRITE),
        ufo_yellow:        asset_server.load(UFO_YELLOW_SPRITE),
        // Font
        font:              asset_server.load("fonts/screen-diags-font.ttf"),
    });

    // Persistent game state resources
    commands.insert_resource(CameraShake::default());
    commands.insert_resource(Life::new(PLAYER_START_HP));
    commands.insert_resource(Score::new(PLAYER_START_SCORE));
    commands.insert_resource(LevelResource::new());
    commands.insert_resource(PlayerUpgrades::default());
    commands.insert_resource(UpgradeSelectionState::default());
    commands.insert_resource(ShipSelectState::default());
    commands.insert_resource(PlayerBuff::default());
    commands.insert_resource(IsPaused::default());
}
