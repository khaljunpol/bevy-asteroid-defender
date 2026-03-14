use bevy::prelude::*;
use rand::prelude::*;

use lib::{Stats, ShipType, DEFAULT_STATS};
use crate::{
    player::player::PlayerComponent,
    resources::{GameSprites, SHIP_NORMAL_SPRITE, SHIP_ATTACK_SPRITE, SHIP_SHIELD_SPRITE},
};

#[derive(Component)]
pub struct ShipComponent {
    pub ship_type: ShipType,
    pub stats:     Stats,
}

impl ShipComponent {
    pub fn new() -> Self {
        let ship_type = Self::random_type();
        ShipComponent { ship_type, stats: Self::stats_for(ship_type) }
    }

    pub fn new_type(ship_type: ShipType) -> Self {
        ShipComponent { ship_type, stats: Self::stats_for(ship_type) }
    }

    fn random_type() -> ShipType {
        thread_rng().gen()
    }

    fn stats_for(ship_type: ShipType) -> Stats {
        DEFAULT_STATS
            .iter()
            .find(|(st, _)| *st == ship_type)
            .map(|(_, s)| *s)
            .unwrap_or(DEFAULT_STATS[0].1)
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_ship_texture_system);
    }
}

/// Keeps the ship sprite in sync with the current ShipComponent type.
fn sync_ship_texture_system(
    game_sprites:   Res<GameSprites>,
    mut query:      Query<(&mut Handle<Image>, &ShipComponent), With<PlayerComponent>>,
) {
    for (mut tex, ship) in &mut query {
        let new_handle = match ship.ship_type {
            ShipType::Attack => game_sprites.ship_type_attack.clone(),
            ShipType::Normal => game_sprites.ship_type_normal.clone(),
            ShipType::Shield => game_sprites.ship_type_shield.clone(),
        };
        *tex = new_handle;
    }
}
