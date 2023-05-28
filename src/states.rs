use bevy::prelude::*;
use crate::{
    common_systems::{
        movement_system, update_transform_system, update_rotation_system
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
        .add_system(update_transform_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(update_rotation_system.in_set(OnUpdate(GameStates::InGame)));
    }
}