use std::time::Duration;

use bevy::{prelude::{*, IntoSystemConfigs}, time::common_conditions::on_timer};

use crate::{
    common::common_systems::{
        movement_system, update_transform_system, update_rotation_system,
        despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system, despawn_if_reached_bounds_timer_system
    }, events::events::{send_state_start_event, send_state_end_event}, utils::{cleanup::{cleanup_system, CleanUpEndGame}, manager::{game_start, game_restart}}, player::player::{player_move_out_of_screen_system, clean_up_player_tween, player_spawn_system}, resources::reset_life
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    Menu,
    #[default]
    StartGame,
    InGame,
    Progression,
    EndGame
}

impl GameStates {
    pub fn next(&self) -> Self {
        match self {
            GameStates::Menu => GameStates::StartGame,
            GameStates::StartGame => GameStates::InGame,
            GameStates::InGame => GameStates::Progression,
            GameStates::Progression => GameStates::EndGame,
            GameStates::EndGame => GameStates::Menu
        }
    }
}

pub enum GameResult {
    Win,
    Lose
}

// Base state to handle persistent systems for all states
pub struct BaseStatePlugin;
impl Plugin for BaseStatePlugin {
    fn build(&self, app: &mut App) {
        app
        // transform and rotation
        .add_systems(Update, (movement_system,
            update_transform_system,
            update_rotation_system));
    }
}

pub struct StartGameStatePlugin;

impl Plugin for StartGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameStates::StartGame), reset_life)
        .add_systems(OnEnter(GameStates::StartGame), send_state_start_event)
        .add_systems(OnExit(GameStates::StartGame), send_state_end_event)
        .add_systems(OnTransition{from: GameStates::StartGame, to: GameStates::InGame}, (clean_up_player_tween))

        .add_systems(Update, game_start
            .run_if(in_state(GameStates::StartGame)
            .and_then(on_timer(Duration::from_secs_f32(1.5)))));
    }
}

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameStates::InGame), send_state_start_event)
        .add_systems(OnExit(GameStates::InGame), (send_state_end_event).chain())

        // warping / bounds
        .add_systems(Update, (despawn_if_reached_bounds_system,
            despawn_if_reached_bounds_timer_system,
            warp_if_reached_window_bounds_system)
            .run_if(in_state(GameStates::InGame)));
    }
}

pub struct ProgressionStatePlugin;

impl Plugin for ProgressionStatePlugin {
    fn build(&self, app: &mut App) {
    }
}

pub struct EndGameStatePlugin;

impl Plugin for EndGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameStates::EndGame), send_state_start_event)
        .add_systems(OnExit(GameStates::EndGame), 
            (send_state_end_event, cleanup_system::<CleanUpEndGame>).chain())
        .add_systems(OnTransition{from: GameStates::EndGame, to: GameStates::StartGame}, clean_up_player_tween)

        .add_systems(Update, game_restart
            .run_if(in_state(GameStates::EndGame)
            .and_then(on_timer(Duration::from_secs_f32(1.5)))));
    }
}