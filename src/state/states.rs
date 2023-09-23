use std::time::Duration;

use bevy::{prelude::{*, IntoSystemConfigs}, time::common_conditions::on_timer};
use bevy_tweening::Animator;
use lib::{POWERUP_SPAWN_TIME, METEOR_SPAWN_TIME};
use crate::{
    common::{common_systems::{
        movement_system, update_transform_system, update_rotation_system,
        despawn_if_reached_bounds_system, warp_if_reached_window_bounds_system, despawn_if_reached_bounds_timer_system
    }, collision::player_collide_despawnable_system}, 
    common::collision::{player_collide_powerup_system, player_projectile_hit_meteor_system, meteor_collision_spawn_system, collision_damage_system},
    objects::{
        powerup::spawn_powerup_system,
        meteor::spawn_meteor_system
    },
    player::{
        player::{player_move_to_center_system, PlayerComponent, player_move_out_of_screen_system}, 
        projectile::projectile_shoot_system
    }, events::events::{ChangeStateEvent, StateStartEvent, StateEndEvent, trigger_state_start_event, trigger_state_end_event, trigger_next_state_event}
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
            GameStates::EndGame => GameStates::StartGame
        }
    }
}

pub enum GameResult {
    Win,
    Lose
}

fn transition_to_in_game_state_system(
    mut commands: Commands,
    app_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>, 
    mut query: Query<(Entity, &PlayerComponent)>,
){
    if let Ok((entity, _player)) = query.get_single_mut() {
        commands.entity(entity).remove::<Animator::<Transform>>();
    }
}

pub struct StartGameStatePlugin;

impl Plugin for StartGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameStates::StartGame), (player_move_to_center_system, trigger_state_start_event))
        .add_systems(Update, (transition_to_in_game_state_system, trigger_next_state_event)
            .run_if(in_state(GameStates::StartGame).and_then(on_timer(Duration::from_secs_f32(1.5)))))
        .add_systems(OnExit(GameStates::StartGame), (trigger_state_end_event));
    }
}

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app
        // player
        .add_systems(Update, projectile_shoot_system.run_if(in_state(GameStates::InGame)))

        // warping
        .add_systems(Update, (despawn_if_reached_bounds_system,
            despawn_if_reached_bounds_timer_system,
            warp_if_reached_window_bounds_system)
            .run_if(in_state(GameStates::InGame)))

        // transform and rotation
        .add_systems(Update, (movement_system,
            update_transform_system,
            update_rotation_system)
            .run_if(in_state(GameStates::InGame)))

        // collisions
        .add_systems(Update, (meteor_collision_spawn_system, 
            collision_damage_system,
            player_collide_powerup_system,
            player_projectile_hit_meteor_system,
            player_collide_despawnable_system)
            .run_if(in_state(GameStates::InGame)))

        // spawning
        .add_systems(Update, spawn_powerup_system
            .run_if(in_state(GameStates::InGame)
            .and_then(on_timer(Duration::from_secs_f32(POWERUP_SPAWN_TIME)))))
        .add_systems(Update, spawn_meteor_system
            .run_if(in_state(GameStates::InGame)
            .and_then(on_timer(Duration::from_secs_f32(METEOR_SPAWN_TIME)))));
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
        .add_systems(OnEnter(GameStates::StartGame), player_move_out_of_screen_system);
    }
}