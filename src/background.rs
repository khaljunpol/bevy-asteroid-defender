use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    resources::GameSprites,
    resources::WindowSize,
    state::states::GameStates,
    utils::cleanup::CleanUpOnGameOver,
};

#[derive(Component)]
pub struct StarComponent {
    pub scroll_speed: f32,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameStates::StartGame), spawn_starfield)
            .add_systems(Update, scroll_stars_system);
    }
}

fn spawn_starfield(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size:     Res<WindowSize>,
) {
    let mut rng = thread_rng();
    let hw = wdw_size.w / 2.0;
    let hh = wdw_size.h / 2.0;

    // Layer 0: slow, small, z=-10
    for _ in 0..80 {
        let x     = rng.gen_range(-hw..hw);
        let y     = rng.gen_range(-hh..hh);
        let scale = rng.gen_range(0.08_f32..0.18);
        commands.spawn((
            SpriteBundle {
                texture: game_sprites.star3.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, -10.0),
                    scale: Vec3::splat(scale),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.8, 0.85, 1.0, 0.6),
                    ..default()
                },
                ..default()
            },
            StarComponent { scroll_speed: 0.2 },
            CleanUpOnGameOver,
        ));
    }

    // Layer 1: medium speed, z=-9
    for _ in 0..50 {
        let x     = rng.gen_range(-hw..hw);
        let y     = rng.gen_range(-hh..hh);
        let scale = rng.gen_range(0.10_f32..0.22);
        commands.spawn((
            SpriteBundle {
                texture: game_sprites.star2.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, -9.0),
                    scale: Vec3::splat(scale),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(0.9, 0.9, 1.0, 0.75),
                    ..default()
                },
                ..default()
            },
            StarComponent { scroll_speed: 0.5 },
            CleanUpOnGameOver,
        ));
    }

    // Layer 2: fast, brighter, z=-8
    for _ in 0..30 {
        let x     = rng.gen_range(-hw..hw);
        let y     = rng.gen_range(-hh..hh);
        let scale = rng.gen_range(0.12_f32..0.28);
        commands.spawn((
            SpriteBundle {
                texture: game_sprites.star1.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, -8.0),
                    scale: Vec3::splat(scale),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.9),
                    ..default()
                },
                ..default()
            },
            StarComponent { scroll_speed: 1.0 },
            CleanUpOnGameOver,
        ));
    }
}

fn scroll_stars_system(
    wdw_size: Res<WindowSize>,
    mut query: Query<(&StarComponent, &mut Transform)>,
) {
    let bottom = -wdw_size.h / 2.0 - 20.0;
    let top    =  wdw_size.h / 2.0 + 20.0;

    for (star, mut tf) in &mut query {
        tf.translation.y -= star.scroll_speed;
        if tf.translation.y < bottom {
            tf.translation.y = top;
        }
    }
}
