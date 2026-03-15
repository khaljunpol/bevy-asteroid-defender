use bevy::prelude::*;

use std::time::Duration;
use bevy_tweening::{EaseFunction, lens::TransformPositionLens, Tween, Animator};

use crate::{
    common::common_systems::{
        movement_system, update_transform_system, update_rotation_system,
        despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system,
    },
    common::common_components::{Velocity, Position, RotationAngle},
    player::player::{clean_up_player_tween, PlayerComponent},
    resources::{reset_life, reset_score, reset_level, reset_upgrades, reset_player_buff, reset_paused, CountdownResource, IsPaused, LevelResource, WindowSize},
    utils::{
        cleanup::{cleanup_system, CleanUpOnGameOver, CleanUpOnLevelEnd},
        manager::{goto_countdown, goto_upgrade_selection},
    },
};

// ── State definition ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    /// Ship selection screen. Player picks Normal/Shield/Attack before starting.
    #[default]
    ShipSelect,
    /// Player spawns from off-screen; all resources reset. Auto-advances to Countdown.
    StartGame,
    /// 3–2–1–GO! countdown displayed before each level. Auto-advances to InGame.
    Countdown,
    /// Active gameplay: asteroids, shooting, collisions. Ends when all asteroids are
    /// cleared (→ LevelComplete) or the player dies (→ GameOver).
    InGame,
    /// Brief celebration screen after clearing a level. Auto-advances to UpgradeSelection.
    LevelComplete,
    /// Roguelike upgrade picker. Player chooses one option, then → Countdown.
    UpgradeSelection,
    /// Player has died. Shows final score; Space/Enter restarts from ShipSelect.
    GameOver,
}

// ── Base plugin – runs every frame regardless of state ────────────────────────

pub struct BaseStatePlugin;

impl Plugin for BaseStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                movement_system.run_if(|p: Res<IsPaused>| !p.0),
            )
            .add_systems(
                Update,
                (update_transform_system, update_rotation_system),
            );
    }
}

// ── ShipSelect ────────────────────────────────────────────────────────────────

pub struct ShipSelectStatePlugin;

impl Plugin for ShipSelectStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ship_select_input_system.run_if(in_state(GameStates::ShipSelect)),
        );
    }
}

fn ship_select_input_system(
    kb:             Res<Input<KeyCode>>,
    mut selection:  ResMut<crate::resources::ShipSelectState>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if kb.just_pressed(KeyCode::Left) || kb.just_pressed(KeyCode::A) {
        selection.selected = (selection.selected + 2) % 3;
    }
    if kb.just_pressed(KeyCode::Right) || kb.just_pressed(KeyCode::D) {
        selection.selected = (selection.selected + 1) % 3;
    }
    if kb.just_pressed(KeyCode::Space) || kb.just_pressed(KeyCode::Return) {
        next_state.set(GameStates::StartGame);
    }
}

// ── StartGame ─────────────────────────────────────────────────────────────────

pub struct StartGameStatePlugin;

impl Plugin for StartGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Clean up any entities from the previous run, reset resources,
            // then spawn the player — in strict order using apply_deferred.
            .add_systems(
                OnEnter(GameStates::StartGame),
                (
                    (cleanup_system::<CleanUpOnGameOver>, cleanup_system::<CleanUpOnLevelEnd>),
                    apply_deferred,
                    (reset_life, reset_score, reset_level, reset_upgrades, reset_player_buff, reset_paused),
                    apply_deferred,
                    crate::player::player::player_spawn_system,
                )
                    .chain(),
            )
            // Remove the intro tween when transitioning to Countdown (initial start).
            .add_systems(
                OnTransition { from: GameStates::StartGame, to: GameStates::Countdown },
                clean_up_player_tween,
            )
            // Between levels: fly ship in from bottom again.
            .add_systems(
                OnTransition { from: GameStates::UpgradeSelection, to: GameStates::Countdown },
                level_transition_player_spawn,
            )
            // Clean up any leftover tween and snap player to center before gameplay.
            .add_systems(
                OnTransition { from: GameStates::Countdown, to: GameStates::InGame },
                (clean_up_player_tween, center_player_on_level_start).chain(),
            )
            // After the spawn animation, move to the countdown.
            .add_systems(
                Update,
                goto_countdown
                    .run_if(in_state(GameStates::StartGame))
                    .run_if(bevy::time::common_conditions::on_timer(
                        std::time::Duration::from_secs_f32(1.8),
                    )),
            );
    }
}

// ── Countdown ─────────────────────────────────────────────────────────────────

pub struct CountdownStatePlugin;

impl Plugin for CountdownStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameStates::Countdown), init_countdown)
            .add_systems(
                Update,
                countdown_tick_system.run_if(in_state(GameStates::Countdown)),
            );
    }
}

fn init_countdown(mut commands: Commands) {
    commands.insert_resource(CountdownResource::new());
}

fn countdown_tick_system(
    time:           Res<Time>,
    mut countdown:  ResMut<CountdownResource>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    countdown.tick_timer.tick(time.delta());

    if countdown.tick_timer.just_finished() && countdown.count > 0 {
        countdown.count -= 1;
        if countdown.count > 0 {
            countdown.tick_timer.reset();
        }
        // When count reaches 0 the go_timer takes over (handled below).
    }

    if countdown.count == 0 {
        countdown.go_timer.tick(time.delta());
        if countdown.go_timer.just_finished() {
            next_state.set(GameStates::InGame);
        }
    }
}

// ── InGame ────────────────────────────────────────────────────────────────────

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawn this level's asteroids when entering play.
            .add_systems(
                OnEnter(GameStates::InGame),
                crate::objects::meteor::spawn_level_asteroids,
            )
            // Bounds systems run only during active gameplay and while not paused.
            .add_systems(
                Update,
                (despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system)
                    .run_if(in_state(GameStates::InGame))
                    .run_if(|p: Res<IsPaused>| !p.0),
            )
            // Pause toggle and restart from pause menu.
            .add_systems(
                Update,
                pause_input_system.run_if(in_state(GameStates::InGame)),
            )
            // Remove leftover projectiles and powerups when leaving InGame.
            .add_systems(OnExit(GameStates::InGame), cleanup_system::<CleanUpOnLevelEnd>);
    }
}

fn pause_input_system(
    kb:             Res<Input<KeyCode>>,
    mut paused:     ResMut<IsPaused>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if kb.just_pressed(KeyCode::Escape) {
        paused.0 = !paused.0;
    }
    // While paused: Enter/Space restarts; R resumes.
    if paused.0 {
        if kb.just_pressed(KeyCode::Space) || kb.just_pressed(KeyCode::Return) {
            paused.0 = false;
            next_state.set(GameStates::ShipSelect);
        }
        if kb.just_pressed(KeyCode::R) {
            paused.0 = false;
        }
    }
}

// ── LevelComplete ─────────────────────────────────────────────────────────────

pub struct LevelCompleteStatePlugin;

impl Plugin for LevelCompleteStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Advance the level counter when leaving, so the UI can show the
            // correct "LEVEL X CLEARED" text while we're still in this state.
            .add_systems(OnExit(GameStates::LevelComplete), advance_level)
            .add_systems(
                Update,
                goto_upgrade_selection
                    .run_if(in_state(GameStates::LevelComplete))
                    .run_if(bevy::time::common_conditions::on_timer(
                        std::time::Duration::from_secs_f32(2.0),
                    )),
            );
    }
}

fn advance_level(mut level: ResMut<LevelResource>) {
    level.advance();
}

// ── UpgradeSelection ──────────────────────────────────────────────────────────

pub struct UpgradeSelectionStatePlugin;

impl Plugin for UpgradeSelectionStatePlugin {
    fn build(&self, _app: &mut App) {
        // All logic is in UpgradePlugin (upgrades/upgrades.rs).
        // UI is in UIPlugin (ui/ui.rs).
    }
}

// ── GameOver ──────────────────────────────────────────────────────────────────

pub struct GameOverStatePlugin;

impl Plugin for GameOverStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            game_over_input_system.run_if(in_state(GameStates::GameOver)),
        );
    }
}

fn game_over_input_system(
    kb:             Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if kb.just_pressed(KeyCode::Space) || kb.just_pressed(KeyCode::Return) {
        next_state.set(GameStates::ShipSelect);
    }
}

/// Teleports the player to the bottom of the screen and re-applies the fly-in
/// tween so the ship enters from off-screen at the start of each new level.
fn level_transition_player_spawn(
    mut commands: Commands,
    wdw_size:     Res<WindowSize>,
    mut query:    Query<(Entity, &mut Transform, &mut Position, &mut Velocity, &mut RotationAngle), With<PlayerComponent>>,
) {
    let Ok((entity, mut tf, mut pos, mut vel, mut angle)) = query.get_single_mut() else { return };

    let start_y = -wdw_size.h;

    // Keep Position at center so update_transform_system doesn't fight the tween after it ends.
    pos.0   = Vec2::ZERO;
    vel.0   = Vec2::ZERO;
    angle.0 = 0.0;

    // Visually start the ship off-screen (the tween overrides Transform each frame while active).
    tf.translation = Vec3::new(0.0, start_y, tf.translation.z);
    tf.rotation    = Quat::IDENTITY;

    let tween = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs_f32(1.6),
        TransformPositionLens {
            start: Vec3::new(0.0, start_y, 0.0),
            end:   Vec3::new(0.0, 0.0, 0.0),
        },
    );
    commands.entity(entity).insert(Animator::<Transform>::new(tween));
}

/// After the tween finishes, snap Position (and rotation) to center so that
/// `update_transform_system` agrees with where the tween left the ship.
fn center_player_on_level_start(
    mut query: Query<(&mut Position, &mut Velocity, &mut RotationAngle), With<PlayerComponent>>,
) {
    let Ok((mut pos, mut vel, mut angle)) = query.get_single_mut() else { return };
    pos.0   = Vec2::ZERO;
    vel.0   = Vec2::ZERO;
    angle.0 = 0.0;
}
