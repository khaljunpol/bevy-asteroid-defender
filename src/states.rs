use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_tweening::Animator;
use lib::{POWERUP_SPAWN_TIME, METEOR_SPAWN_TIME};
use crate::{
    common_systems::{
        movement_system, update_transform_system, update_rotation_system,
        despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system, despawn_if_reached_bounds_timer_system
    }, 
    collision::{player_collide_powerup_system, player_projectile_hit_asteroid_system},
    powerup::{
        spawn_powerup_system
    },
    meteor::{
        spawn_meteor_system, meteor_collision_spawn_system
    }, player::{player_move_to_center, PlayerComponent}
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    Menu,
    #[default]
    StartGame,
    InGame,
    Ascension,
    EndGame
}

pub enum GameResult {
    Win,
    Lose
}

fn start_game(
    mut commands: Commands,
    mut app_state: ResMut<State<GameStates>>,
    mut query: Query<(Entity, &PlayerComponent)>
){
    if let Ok((entity, _player)) = query.get_single_mut() {
        commands.entity(entity).remove::<Animator::<Transform>>();
    }

    app_state.0 = GameStates::InGame;
}

pub struct StartGameStatePlugin;

impl Plugin for StartGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(player_move_to_center.in_schedule(OnEnter(GameStates::StartGame)))
        .add_system(start_game.in_schedule(OnEnter(GameStates::StartGame))
            .after(on_timer(Duration::from_secs_f32(3.0))));
    }
}

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(movement_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(despawn_if_reached_bounds_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(despawn_if_reached_bounds_timer_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(warp_if_reached_window_bounds_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(update_transform_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(update_rotation_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(meteor_collision_spawn_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(player_collide_powerup_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(player_projectile_hit_asteroid_system.in_set(OnUpdate(GameStates::InGame)))
        .add_system(spawn_powerup_system.in_set(OnUpdate(GameStates::InGame))
            .run_if(on_timer(Duration::from_secs_f32(POWERUP_SPAWN_TIME))))
        .add_system(spawn_meteor_system.in_set(OnUpdate(GameStates::InGame))
            .run_if(on_timer(Duration::from_secs_f32(METEOR_SPAWN_TIME))));
    }
}