use crate::{geom};
use rand;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

pub enum PowerUpType {
    INVINCIBLE = 0,
    SHIELD = 1,
    SPLIT = 2
}

pub struct PowerUp {
    pub p_type: PowerUpType,
    pub pos: geom::Position,
}

impl Distribution<PowerUpType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpType {
        match rng.gen_range(0,2) {
            0 => PowerUpType::INVINCIBLE,
            1 => PowerUpType::SHIELD,
            2 => PowerUpType::SPLIT,
            _ => PowerUpType::INVINCIBLE
        }
    }
}

impl PowerUp {
    pub fn new(x: f64, y: f64, p_type: PowerUpType) -> PowerUp {
        PowerUp {
            p_type: p_type,
            pos: geom::Position::new(x, y)
        }
    }

    pub fn new_rand(max_x: f64, max_y: f64) -> PowerUp{
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0, max_x);
        let randy = rng.gen_range(0.0, max_y);
        let p_type: PowerUpType = rand::random::<PowerUpType>();
        PowerUp::new(randx, randy, p_type)
    }
}