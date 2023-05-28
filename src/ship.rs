use bevy::prelude::*;
use lib::{
    Stats, ShipType, DEFAULT_STATS
};
use rand::{
    prelude::*
};

use crate::
{
    player::PlayerComponent, 
    resources::GameSprites
};

#[derive(Component)]
pub struct ShipComponent {
    pub ship_type: ShipType,
    pub stats: Stats
}

impl ShipComponent {
    pub fn new() -> ShipComponent {
        let ship_type = Self::randomize_type();
        let stats = Self::get_stats_from_type(ship_type);

        ShipComponent { ship_type, stats }
    }

    fn randomize_type() -> ShipType {
        let mut rng = thread_rng();
        // Generate a random ShipType
        rng.gen()
    }

    fn get_stats_from_type(ship_type: ShipType) -> Stats {
        for (st, stats) in DEFAULT_STATS {
            if matches!(st, ship_type) {
                return stats;
            }
        }
        DEFAULT_STATS[0].1.clone()
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(randomize_type_system);
    }
}

fn randomize_type_system(
    keyboard_input: Res<Input<KeyCode>>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(&mut Handle<Image>, &mut ShipComponent), With<PlayerComponent>>,
)
{
    for (mut texture_handle, mut ship_component) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::X) {
            *ship_component = ShipComponent::new();
        }

        // Load a new texture and update the handle
        let new_texture_handle: Handle<Image> = match ship_component.ship_type {
            ShipType::Attack => game_sprites.ship_type_attack.clone(),
            ShipType::Shield => game_sprites.ship_type_shield.clone(),
            ShipType::Normal => game_sprites.ship_type_normal.clone()
        };

        *texture_handle = new_texture_handle;
    }
}