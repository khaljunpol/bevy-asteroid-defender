use bevy::prelude::{ResMut, NextState};
use crate::state::states::GameStates;

pub fn goto_countdown(mut next: ResMut<NextState<GameStates>>) {
    next.set(GameStates::Countdown);
}

pub fn goto_in_game(mut next: ResMut<NextState<GameStates>>) {
    next.set(GameStates::InGame);
}

pub fn goto_level_complete(mut next: ResMut<NextState<GameStates>>) {
    next.set(GameStates::LevelComplete);
}

pub fn goto_upgrade_selection(mut next: ResMut<NextState<GameStates>>) {
    next.set(GameStates::UpgradeSelection);
}

pub fn goto_game_over(mut next: ResMut<NextState<GameStates>>) {
    next.set(GameStates::GameOver);
}
