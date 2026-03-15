use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::resources::{Life, PlayerUpgrades, UpgradeSelectionState};

// ── Upgrade catalogue ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UpgradeType {
    // Offense
    SplitShot,
    RearGuard,        // shoot behind, then all 4 directions
    RapidFire,
    HeavyRounds,
    Ricochet,
    Accelerator,      // faster bullets, shorter range
    PiercingRounds,   // bullets pierce asteroids (slower)
    ExplosiveRounds,  // on kill, scatter shrapnel
    // Defense
    ExtraArmor,
    Afterburner,
    QuickReflexes,
    LongShot,         // NEW: longer range, slower bullet
    Bulwark,          // NEW: chance to heal on large kill
    // Special
    Overclock,
    ChainReaction,
    AsteroidMagnet,
    GlassCannon,      // NEW: double damage+speed, -1 max HP, slower fire
    DetonatorRounds,  // NEW: bullets explode at max range
}

impl UpgradeType {
    pub fn all() -> &'static [UpgradeType] {
        use UpgradeType::*;
        &[
            SplitShot, RearGuard, RapidFire, HeavyRounds, Ricochet,
            Accelerator, PiercingRounds, ExplosiveRounds,
            ExtraArmor, Afterburner, QuickReflexes, LongShot, Bulwark,
            Overclock, ChainReaction, AsteroidMagnet, GlassCannon,
            DetonatorRounds,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot       => "SPLIT SHOT",
            UpgradeType::RearGuard       => "REAR GUARD",
            UpgradeType::RapidFire       => "RAPID FIRE",
            UpgradeType::HeavyRounds     => "HEAVY ROUNDS",
            UpgradeType::Ricochet        => "RICOCHET",
            UpgradeType::Accelerator     => "ACCELERATOR",
            UpgradeType::PiercingRounds  => "PIERCING ROUNDS",
            UpgradeType::ExplosiveRounds => "EXPLOSIVE ROUNDS",
            UpgradeType::ExtraArmor      => "EXTRA ARMOR",
            UpgradeType::Afterburner     => "AFTERBURNER",
            UpgradeType::QuickReflexes   => "QUICK REFLEXES",
            UpgradeType::LongShot        => "LONG SHOT",
            UpgradeType::Bulwark         => "BULWARK",
            UpgradeType::Overclock       => "OVERCLOCK",
            UpgradeType::ChainReaction   => "CHAIN REACTION",
            UpgradeType::AsteroidMagnet  => "ASTEROID MAGNET",
            UpgradeType::GlassCannon     => "GLASS CANNON",
            UpgradeType::DetonatorRounds => "DETONATOR",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot =>
                "Lv1: 2 bullets. Lv2: 3 bullets. Lv3: 5 bullets in a spread.",
            UpgradeType::RearGuard =>
                "Fires a bullet behind the ship. Level 2 fires in all 4 directions.",
            UpgradeType::RapidFire =>
                "25% fire cooldown per level. Spray and pray.",
            UpgradeType::HeavyRounds =>
                "+1 bullet damage per level. Tear through asteroids.",
            UpgradeType::Ricochet =>
                "Bullets bounce off screen edges once.",
            UpgradeType::Accelerator =>
                "+28% bullet speed per level. 18% less range per level.",
            UpgradeType::PiercingRounds =>
                "Bullets pierce through 1 extra asteroid per level. 12% speed penalty per level.",
            UpgradeType::ExplosiveRounds =>
                "On large asteroid kill, scatter 3 shrapnel fragments.",
            UpgradeType::ExtraArmor =>
                "+1 max HP and restore 1 HP immediately.",
            UpgradeType::Afterburner =>
                "+30% top ship speed per level.",
            UpgradeType::QuickReflexes =>
                "+40% turn speed per level.",
            UpgradeType::LongShot =>
                "+35% bullet range per level. 10% speed penalty per level.",
            UpgradeType::Bulwark =>
                "35% chance to restore 1 HP when destroying a large asteroid.",
            UpgradeType::Overclock =>
                "Asteroids move at 60% normal speed.",
            UpgradeType::ChainReaction =>
                "Each kill triggers 3s of ultra-rapid fire.",
            UpgradeType::AsteroidMagnet =>
                "Powerups drift toward your ship.",
            UpgradeType::GlassCannon =>
                "DOUBLE damage and +50% bullet speed. Costs 1 max HP. 50% slower fire rate.",
            UpgradeType::DetonatorRounds =>
                "Bullets explode at max range. 55% range penalty ensures on-screen detonation.",
        }
    }

    pub fn max_level(&self) -> u32 {
        match self {
            UpgradeType::SplitShot       => 3,
            UpgradeType::RearGuard       => 2,
            UpgradeType::RapidFire       => 3,
            UpgradeType::HeavyRounds     => 2,
            UpgradeType::Ricochet        => 1,
            UpgradeType::Accelerator     => 2,
            UpgradeType::PiercingRounds  => 2,
            UpgradeType::ExplosiveRounds => 1,
            UpgradeType::ExtraArmor      => 2,
            UpgradeType::Afterburner     => 2,
            UpgradeType::QuickReflexes   => 2,
            UpgradeType::LongShot        => 2,
            UpgradeType::Bulwark         => 1,
            UpgradeType::Overclock       => 1,
            UpgradeType::ChainReaction   => 1,
            UpgradeType::AsteroidMagnet  => 1,
            UpgradeType::GlassCannon     => 1,
            UpgradeType::DetonatorRounds => 1,
        }
    }

    pub fn current_level(&self, upgrades: &PlayerUpgrades) -> u32 {
        match self {
            UpgradeType::SplitShot       => upgrades.split_shot,
            UpgradeType::RearGuard       => upgrades.rear_guard,
            UpgradeType::RapidFire       => upgrades.rapid_fire,
            UpgradeType::HeavyRounds     => upgrades.heavy_rounds,
            UpgradeType::Ricochet        => upgrades.ricochet as u32,
            UpgradeType::Accelerator     => upgrades.accelerator,
            UpgradeType::PiercingRounds  => upgrades.piercing_rounds,
            UpgradeType::ExplosiveRounds => upgrades.explosive_rounds as u32,
            UpgradeType::ExtraArmor      => upgrades.extra_armor,
            UpgradeType::Afterburner     => upgrades.afterburner,
            UpgradeType::QuickReflexes   => upgrades.quick_reflexes,
            UpgradeType::LongShot        => upgrades.long_shot,
            UpgradeType::Bulwark         => upgrades.bulwark as u32,
            UpgradeType::Overclock       => upgrades.overclock as u32,
            UpgradeType::ChainReaction   => upgrades.chain_reaction as u32,
            UpgradeType::AsteroidMagnet  => upgrades.asteroid_magnet as u32,
            UpgradeType::GlassCannon     => upgrades.glass_cannon as u32,
            UpgradeType::DetonatorRounds => upgrades.detonator_rounds as u32,
        }
    }

    pub fn is_eligible(&self, upgrades: &PlayerUpgrades) -> bool {
        // Glass Cannon and Accelerator/LongShot conflict: can only have one range-modifier.
        if *self == UpgradeType::GlassCannon && upgrades.glass_cannon {
            return false;
        }
        self.current_level(upgrades) < self.max_level()
    }

    pub fn apply(&self, upgrades: &mut PlayerUpgrades, life: &mut Life) -> bool {
        if !self.is_eligible(upgrades) {
            return false;
        }
        match self {
            UpgradeType::SplitShot       => upgrades.split_shot      += 1,
            UpgradeType::RearGuard       => upgrades.rear_guard       += 1,
            UpgradeType::RapidFire       => upgrades.rapid_fire       += 1,
            UpgradeType::HeavyRounds     => upgrades.heavy_rounds     += 1,
            UpgradeType::Ricochet        => upgrades.ricochet          = true,
            UpgradeType::Accelerator     => upgrades.accelerator       += 1,
            UpgradeType::PiercingRounds  => upgrades.piercing_rounds   += 1,
            UpgradeType::ExplosiveRounds => upgrades.explosive_rounds  = true,
            UpgradeType::ExtraArmor      => {
                upgrades.extra_armor += 1;
                life.max_life        += 1;
                life.current_life     = (life.current_life + 1).min(life.max_life);
            }
            UpgradeType::Afterburner     => upgrades.afterburner       += 1,
            UpgradeType::QuickReflexes   => upgrades.quick_reflexes    += 1,
            UpgradeType::LongShot        => upgrades.long_shot          += 1,
            UpgradeType::Bulwark         => upgrades.bulwark            = true,
            UpgradeType::Overclock       => upgrades.overclock          = true,
            UpgradeType::ChainReaction   => upgrades.chain_reaction     = true,
            UpgradeType::AsteroidMagnet  => upgrades.asteroid_magnet    = true,
            UpgradeType::GlassCannon     => {
                upgrades.glass_cannon = true;
                // Permanent HP cost — minimum 1.
                life.max_life     = (life.max_life - 1).max(1);
                life.current_life = life.current_life.min(life.max_life);
            }
            UpgradeType::DetonatorRounds => upgrades.detonator_rounds = true,
        }
        true
    }

    pub fn category(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot | UpgradeType::RearGuard | UpgradeType::RapidFire |
            UpgradeType::HeavyRounds | UpgradeType::Ricochet | UpgradeType::Accelerator |
            UpgradeType::PiercingRounds | UpgradeType::ExplosiveRounds |
            UpgradeType::DetonatorRounds => "OFFENSE",

            UpgradeType::ExtraArmor | UpgradeType::Afterburner | UpgradeType::QuickReflexes |
            UpgradeType::LongShot   | UpgradeType::Bulwark => "DEFENSE",

            _ => "SPECIAL",
        }
    }

    pub fn category_color(&self) -> Color {
        match self.category() {
            "OFFENSE" => Color::rgb(1.0, 0.35, 0.35),
            "DEFENSE" => Color::rgb(0.35, 0.65, 1.0),
            _         => Color::rgb(0.9, 0.75, 0.2),
        }
    }
}

// ── Selection resource helpers ────────────────────────────────────────────────

pub fn generate_choices(upgrades: &PlayerUpgrades, count: usize) -> Vec<UpgradeType> {
    let mut eligible: Vec<UpgradeType> = UpgradeType::all()
        .iter()
        .filter(|u| u.is_eligible(upgrades))
        .copied()
        .collect();

    let mut rng = thread_rng();
    eligible.shuffle(&mut rng);
    eligible.truncate(count);
    eligible
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(crate::state::states::GameStates::UpgradeSelection),
                setup_upgrade_selection,
            )
            .add_systems(
                Update,
                upgrade_input_system
                    .run_if(in_state(crate::state::states::GameStates::UpgradeSelection)),
            );
    }
}

fn setup_upgrade_selection(
    upgrades:      Res<PlayerUpgrades>,
    mut selection: ResMut<UpgradeSelectionState>,
) {
    let count = lib::UPGRADE_CHOICES;
    selection.choices  = generate_choices(&upgrades, count);
    selection.selected = 0;
}

fn upgrade_input_system(
    kb:             Res<Input<KeyCode>>,
    mut selection:  ResMut<UpgradeSelectionState>,
    mut upgrades:   ResMut<PlayerUpgrades>,
    mut life:       ResMut<Life>,
    mut next_state: ResMut<NextState<crate::state::states::GameStates>>,
) {
    if selection.choices.is_empty() {
        return;
    }

    let len = selection.choices.len();

    if kb.just_pressed(KeyCode::Left) || kb.just_pressed(KeyCode::A) {
        selection.selected = (selection.selected + len - 1) % len;
    }
    if kb.just_pressed(KeyCode::Right) || kb.just_pressed(KeyCode::D) {
        selection.selected = (selection.selected + 1) % len;
    }

    if kb.just_pressed(KeyCode::Space) || kb.just_pressed(KeyCode::Return) {
        let chosen = selection.choices[selection.selected];
        chosen.apply(&mut upgrades, &mut life);
        next_state.set(crate::state::states::GameStates::Countdown);
    }
}
