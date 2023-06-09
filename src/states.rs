use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use lib::{POWERUP_SPAWN_TIME, METEOR_SPAWN_TIME};
use crate::{
    common_systems::{
        movement_system, update_transform_system, update_rotation_system,
        despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system
    }, 
    collision::player_collide_powerup_system,
    powerup::{
        spawn_powerup_system
    },
    meteor::{
        spawn_meteor_system
    }
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    Menu,
    StartGame,
    #[default]
    InGame,
    Ascension,
    EndGame
}

pub enum GameResult {
    Win,
    Lose
}

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(despawn_if_reached_bounds_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(warp_if_reached_window_bounds_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(update_transform_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(update_rotation_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(player_collide_powerup_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(spawn_powerup_system.run_if(on_timer(Duration::from_secs_f32(POWERUP_SPAWN_TIME))))
        .add_system(spawn_meteor_system.run_if(on_timer(Duration::from_secs_f32(METEOR_SPAWN_TIME))));
    }
}