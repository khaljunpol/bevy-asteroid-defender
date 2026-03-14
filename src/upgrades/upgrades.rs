use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::resources::{Life, PlayerUpgrades, UpgradeSelectionState};

// ── Upgrade catalogue ─────────────────────────────────────────────────────────

/// All available upgrades. The order here is only for internal bookkeeping.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum UpgradeType {
    // Offense
    SplitShot,
    RapidFire,
    HeavyRounds,
    Ricochet,
    // Defense
    ExtraArmor,
    Afterburner,
    QuickReflexes,
    // Special
    Overclock,
    ChainReaction,
    AsteroidMagnet,
}

impl UpgradeType {
    pub fn all() -> &'static [UpgradeType] {
        use UpgradeType::*;
        &[
            SplitShot, RapidFire, HeavyRounds, Ricochet,
            ExtraArmor, Afterburner, QuickReflexes,
            Overclock, ChainReaction, AsteroidMagnet,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot     => "SPLIT SHOT",
            UpgradeType::RapidFire     => "RAPID FIRE",
            UpgradeType::HeavyRounds   => "HEAVY ROUNDS",
            UpgradeType::Ricochet      => "RICOCHET",
            UpgradeType::ExtraArmor    => "EXTRA ARMOR",
            UpgradeType::Afterburner   => "AFTERBURNER",
            UpgradeType::QuickReflexes => "QUICK REFLEXES",
            UpgradeType::Overclock     => "OVERCLOCK",
            UpgradeType::ChainReaction => "CHAIN REACTION",
            UpgradeType::AsteroidMagnet => "ASTEROID MAGNET",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot =>
                "Fire extra bullets in a spread pattern. +2 bullets per level.",
            UpgradeType::RapidFire =>
                "Reduce fire cooldown by 25% per level.",
            UpgradeType::HeavyRounds =>
                "Bullets deal +1 damage per level (pierces tougher asteroids faster).",
            UpgradeType::Ricochet =>
                "Bullets bounce off screen edges once before despawning.",
            UpgradeType::ExtraArmor =>
                "+1 maximum HP and restore 1 HP immediately.",
            UpgradeType::Afterburner =>
                "+30% top speed per level. Push through asteroid fields.",
            UpgradeType::QuickReflexes =>
                "+40% turn speed per level. Dodge like a pro.",
            UpgradeType::Overclock =>
                "Asteroids move at 60% of their normal speed.",
            UpgradeType::ChainReaction =>
                "Each asteroid kill triggers 3 seconds of ultra-rapid fire.",
            UpgradeType::AsteroidMagnet =>
                "HP pickups slowly drift toward your ship.",
        }
    }

    pub fn max_level(&self) -> u32 {
        match self {
            UpgradeType::SplitShot     => 3,
            UpgradeType::RapidFire     => 3,
            UpgradeType::HeavyRounds   => 2,
            UpgradeType::Ricochet      => 1,
            UpgradeType::ExtraArmor    => 2,
            UpgradeType::Afterburner   => 2,
            UpgradeType::QuickReflexes => 2,
            UpgradeType::Overclock     => 1,
            UpgradeType::ChainReaction => 1,
            UpgradeType::AsteroidMagnet => 1,
        }
    }

    pub fn current_level(&self, upgrades: &PlayerUpgrades) -> u32 {
        match self {
            UpgradeType::SplitShot     => upgrades.split_shot,
            UpgradeType::RapidFire     => upgrades.rapid_fire,
            UpgradeType::HeavyRounds   => upgrades.heavy_rounds,
            UpgradeType::Ricochet      => upgrades.ricochet as u32,
            UpgradeType::ExtraArmor    => upgrades.extra_armor,
            UpgradeType::Afterburner   => upgrades.afterburner,
            UpgradeType::QuickReflexes => upgrades.quick_reflexes,
            UpgradeType::Overclock     => upgrades.overclock as u32,
            UpgradeType::ChainReaction => upgrades.chain_reaction as u32,
            UpgradeType::AsteroidMagnet => upgrades.asteroid_magnet as u32,
        }
    }

    pub fn is_eligible(&self, upgrades: &PlayerUpgrades) -> bool {
        self.current_level(upgrades) < self.max_level()
    }

    /// Apply the upgrade, returning `true` on success.
    pub fn apply(&self, upgrades: &mut PlayerUpgrades, life: &mut Life) -> bool {
        if !self.is_eligible(upgrades) {
            return false;
        }
        match self {
            UpgradeType::SplitShot     => upgrades.split_shot     += 1,
            UpgradeType::RapidFire     => upgrades.rapid_fire     += 1,
            UpgradeType::HeavyRounds   => upgrades.heavy_rounds   += 1,
            UpgradeType::Ricochet      => upgrades.ricochet        = true,
            UpgradeType::ExtraArmor    => {
                upgrades.extra_armor += 1;
                life.max_life     += 1;
                life.current_life  = (life.current_life + 1).min(life.max_life);
            }
            UpgradeType::Afterburner   => upgrades.afterburner    += 1,
            UpgradeType::QuickReflexes => upgrades.quick_reflexes += 1,
            UpgradeType::Overclock     => upgrades.overclock       = true,
            UpgradeType::ChainReaction => upgrades.chain_reaction  = true,
            UpgradeType::AsteroidMagnet => upgrades.asteroid_magnet = true,
        }
        true
    }

    /// Category label used in the UI card header.
    pub fn category(&self) -> &'static str {
        match self {
            UpgradeType::SplitShot | UpgradeType::RapidFire |
            UpgradeType::HeavyRounds | UpgradeType::Ricochet => "OFFENSE",
            UpgradeType::ExtraArmor | UpgradeType::Afterburner |
            UpgradeType::QuickReflexes => "DEFENSE",
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

/// Build a fresh set of `count` random eligible upgrades.
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
    upgrades:     Res<PlayerUpgrades>,
    mut selection: ResMut<UpgradeSelectionState>,
) {
    let count = lib::UPGRADE_CHOICES;
    selection.choices  = generate_choices(&upgrades, count);
    selection.selected = 0;
}

fn upgrade_input_system(
    kb:           Res<Input<KeyCode>>,
    mut selection: ResMut<UpgradeSelectionState>,
    mut upgrades:  ResMut<PlayerUpgrades>,
    mut life:      ResMut<Life>,
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
