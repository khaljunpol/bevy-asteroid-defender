use bevy::prelude::{EventReader, ResMut, NextState};

use crate::{state::states::GameStates};

/**
 * Trigger Start Game
 * menu > start game
 */
pub fn intro_start(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::StartGame);
}

/**
 * Trigger In Game
 * Start Game > In Game
 */
pub fn game_start(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::InGame);
}

/**
 * Trigger Progression
 * In Game > Progression
 */
pub fn start_progression(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::Progression);
}

/**
 * Trigger Start Game
 * Progression > End Game
 */
pub fn game_end(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::EndGame);
}

/**
 * Restart Game from end game to In Game
 */
pub fn game_restart(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::StartGame);
}

/**
 * Go to Menu
 */
pub fn menu_start(
    mut next_state: ResMut<NextState<GameStates>>)
{
    next_state.set(GameStates::Menu);
}

// Menu - reset stats or the whole game
// Lose - Go to menu
// Win - Go to start game