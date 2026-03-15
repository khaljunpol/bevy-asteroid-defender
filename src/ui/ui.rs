use bevy::prelude::*;

use crate::{
    resources::{CountdownResource, GameSprites, Life, LevelResource, PlayerUpgrades, Score, UpgradeSelectionState},
    state::states::GameStates,
    upgrades::upgrades::UpgradeType,
};

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // HUD (score + HP)
            .add_systems(OnEnter(GameStates::StartGame),   spawn_hud)
            .add_systems(OnExit(GameStates::GameOver),     despawn_hud)
            .add_systems(Update, update_score_ui)
            .add_systems(Update, update_life_ui)
            // Countdown
            .add_systems(OnEnter(GameStates::Countdown),   spawn_countdown_ui)
            .add_systems(OnExit(GameStates::Countdown),    despawn_countdown_ui)
            .add_systems(Update, update_countdown_ui.run_if(in_state(GameStates::Countdown)))
            // Level indicator overlay
            .add_systems(OnEnter(GameStates::InGame),      spawn_level_indicator)
            .add_systems(Update, fade_level_indicator.run_if(in_state(GameStates::InGame)))
            // Level-clear screen
            .add_systems(OnEnter(GameStates::LevelComplete),  spawn_level_clear_ui)
            .add_systems(OnExit(GameStates::LevelComplete),   despawn_level_clear_ui)
            // Upgrade selection screen
            .add_systems(OnEnter(GameStates::UpgradeSelection),  spawn_upgrade_ui)
            .add_systems(OnExit(GameStates::UpgradeSelection),   despawn_upgrade_ui)
            .add_systems(Update, update_upgrade_ui.run_if(in_state(GameStates::UpgradeSelection)))
            // Game-over screen
            .add_systems(OnEnter(GameStates::GameOver),    spawn_game_over_ui)
            .add_systems(OnExit(GameStates::GameOver),     despawn_game_over_ui);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Marker components
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component)] struct HudRoot;
#[derive(Component)] struct ScoreText;
#[derive(Component)] struct LifeText;

#[derive(Component)] struct CountdownRoot;
#[derive(Component)] struct CountdownText;

#[derive(Component)] struct LevelIndicator { timer: Timer }

#[derive(Component)] struct LevelClearRoot;

#[derive(Component)] struct UpgradeRoot;
#[derive(Component)] struct UpgradeCard { index: usize }
#[derive(Component)] struct UpgradeCardLevel { index: usize }

#[derive(Component)] struct GameOverRoot;

// ─────────────────────────────────────────────────────────────────────────────
// HUD  (score + HP bar, always visible while alive)
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:   Val::Percent(100.0),
                    height:  Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(16.0)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                z_index: ZIndex::Local(1),
                ..default()
            },
            HudRoot,
            Name::new("HUD Root"),
        ))
        .with_children(|root| {
            // Top row: score on left, HP on right
            root.spawn(NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items:     AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    TextBundle::from_section(
                        "SCORE: 0",
                        TextStyle {
                            font_size: 22.0,
                            color:     Color::WHITE,
                            ..default()
                        },
                    ),
                    ScoreText,
                ));

                row.spawn((
                    TextBundle::from_section(
                        "HP: ♥♥♥",
                        TextStyle {
                            font_size: 22.0,
                            color:     Color::rgb(1.0, 0.4, 0.4),
                            ..default()
                        },
                    ),
                    LifeText,
                ));
            });
        });
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn update_score_ui(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut t in &mut query {
        t.sections[0].value = format!("SCORE: {}  BEST: {}", score.current, score.high_score);
    }
}

fn update_life_ui(mut query: Query<&mut Text, With<LifeText>>, life: Res<Life>) {
    for mut t in &mut query {
        let filled  = "♥".repeat(life.current_life.max(0) as usize);
        let empty   = "♡".repeat((life.max_life - life.current_life).max(0) as usize);
        t.sections[0].value = format!("HP: {}{}", filled, empty);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Countdown  (3 → 2 → 1 → GO!)
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_countdown_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                z_index: ZIndex::Local(10),
                ..default()
            },
            CountdownRoot,
        ))
        .with_children(|root| {
            root.spawn((
                TextBundle::from_section(
                    "3",
                    TextStyle {
                        font_size: 140.0,
                        color:     Color::rgba(1.0, 1.0, 1.0, 0.9),
                        ..default()
                    },
                ),
                CountdownText,
            ));
        });
}

fn despawn_countdown_ui(mut commands: Commands, query: Query<Entity, With<CountdownRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn update_countdown_ui(
    countdown: Res<CountdownResource>,
    mut query: Query<&mut Text, With<CountdownText>>,
) {
    let label = match countdown.count {
        3 => "3",
        2 => "2",
        1 => "1",
        0 => "GO!",
        _ => "",
    };
    for mut t in &mut query {
        t.sections[0].value = label.to_string();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Level indicator  (fades in/out at the top when a new level begins)
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_level_indicator(mut commands: Commands, level: Res<LevelResource>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::FlexStart,
                    padding:         UiRect::top(Val::Px(60.0)),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                z_index: ZIndex::Local(5),
                ..default()
            },
            LevelIndicator { timer: Timer::from_seconds(2.5, TimerMode::Once) },
            Name::new("LevelIndicator"),
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                format!("— LEVEL {} —", level.current),
                TextStyle {
                    font_size: 36.0,
                    color:     Color::rgba(1.0, 1.0, 0.5, 0.0),
                    ..default()
                },
            ));
        });
}

fn fade_level_indicator(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut LevelIndicator, &Children)>,
    mut texts:    Query<&mut Text>,
) {
    for (entity, mut indicator, children) in query.iter_mut() {
        indicator.timer.tick(time.delta());
        let pct = indicator.timer.percent();

        // Fade in during first 20%, hold, fade out during last 30%.
        let alpha = if pct < 0.2 {
            pct / 0.2
        } else if pct > 0.7 {
            1.0 - (pct - 0.7) / 0.3
        } else {
            1.0
        };

        for &child in children {
            if let Ok(mut t) = texts.get_mut(child) {
                t.sections[0].style.color = Color::rgba(1.0, 1.0, 0.5, alpha);
            }
        }

        if indicator.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Level-clear screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_level_clear_ui(mut commands: Commands, level: Res<LevelResource>) {
    // level.current is still the cleared level here — advance_level fires on OnExit.
    let cleared = level.current;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(16.0),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.55).into(),
                z_index: ZIndex::Local(20),
                ..default()
            },
            LevelClearRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                format!("LEVEL {} CLEARED!", cleared),
                TextStyle { font_size: 60.0, color: Color::rgb(0.4, 1.0, 0.5), ..default() },
            ));
            root.spawn(TextBundle::from_section(
                "Choosing upgrade…",
                TextStyle { font_size: 26.0, color: Color::rgba(1.0, 1.0, 1.0, 0.7), ..default() },
            ));
        });
}

fn despawn_level_clear_ui(mut commands: Commands, query: Query<Entity, With<LevelClearRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

// ─────────────────────────────────────────────────────────────────────────────
// Upgrade selection screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_upgrade_ui(
    mut commands: Commands,
    selection:    Res<UpgradeSelectionState>,
    upgrades:     Res<PlayerUpgrades>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(32.0),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.1, 0.85).into(),
                z_index: ZIndex::Local(30),
                ..default()
            },
            UpgradeRoot,
            Name::new("Upgrade UI"),
        ))
        .with_children(|root| {
            // Header
            root.spawn(TextBundle::from_section(
                "CHOOSE AN UPGRADE",
                TextStyle { font_size: 40.0, color: Color::WHITE, ..default() },
            ));

            // Card row
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction:  FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Stretch,
                    column_gap:      Val::Px(24.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                for (i, upgrade) in selection.choices.iter().enumerate() {
                    let is_selected = i == selection.selected;
                    spawn_upgrade_card(row, *upgrade, i, is_selected, &upgrades);
                }
            });

            // Navigation hint
            root.spawn(TextBundle::from_section(
                "◄ ► to navigate    SPACE / ENTER to select",
                TextStyle {
                    font_size: 20.0,
                    color:     Color::rgba(1.0, 1.0, 1.0, 0.6),
                    ..default()
                },
            ));
        });
}

fn spawn_upgrade_card(
    parent:      &mut ChildBuilder,
    upgrade:     UpgradeType,
    index:       usize,
    is_selected: bool,
    upgrades:    &PlayerUpgrades,
) {
    let border_color = if is_selected {
        upgrade.category_color()
    } else {
        Color::rgba(1.0, 1.0, 1.0, 0.15)
    };

    let bg_color = if is_selected {
        Color::rgba(0.12, 0.12, 0.25, 0.95)
    } else {
        Color::rgba(0.06, 0.06, 0.14, 0.85)
    };

    let cur = upgrade.current_level(upgrades);
    let max = upgrade.max_level();

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width:          Val::Px(220.0),
                    flex_direction: FlexDirection::Column,
                    padding:        UiRect::all(Val::Px(18.0)),
                    row_gap:        Val::Px(10.0),
                    border:         UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: bg_color.into(),
                border_color:     border_color.into(),
                ..default()
            },
            UpgradeCard { index },
        ))
        .with_children(|card| {
            // Category badge
            card.spawn(TextBundle::from_section(
                upgrade.category(),
                TextStyle {
                    font_size: 11.0,
                    color:     upgrade.category_color(),
                    ..default()
                },
            ));

            // Upgrade name
            card.spawn(TextBundle::from_section(
                upgrade.name(),
                TextStyle {
                    font_size: 20.0,
                    color:     if is_selected { Color::WHITE } else { Color::rgba(1.0, 1.0, 1.0, 0.7) },
                    ..default()
                },
            ));

            // Description
            card.spawn(TextBundle {
                text: Text::from_section(
                    upgrade.description(),
                    TextStyle {
                        font_size: 13.0,
                        color:     Color::rgba(0.85, 0.85, 0.85, 0.8),
                        ..default()
                    },
                ),
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            });

            // Level indicator
            let level_text = if max == 1 {
                if cur == 0 { "NEW".to_string() } else { "MAXED".to_string() }
            } else {
                format!("Level {} → {}", cur, cur + 1)
            };

            card.spawn((
                TextBundle::from_section(
                    level_text,
                    TextStyle {
                        font_size: 14.0,
                        color:     upgrade.category_color(),
                        ..default()
                    },
                ),
                UpgradeCardLevel { index },
            ));
        });
}

fn despawn_upgrade_ui(mut commands: Commands, query: Query<Entity, With<UpgradeRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

/// Updates the highlight (background + border color) of upgrade cards when the
/// player moves the selection cursor.
fn update_upgrade_ui(
    selection: Res<UpgradeSelectionState>,
    mut cards: Query<(&UpgradeCard, &mut BackgroundColor, &mut BorderColor)>,
) {
    if !selection.is_changed() || selection.choices.is_empty() {
        return;
    }

    for (card, mut bg, mut border) in &mut cards {
        if card.index >= selection.choices.len() {
            continue;
        }
        let upgrade     = selection.choices[card.index];
        let is_selected = card.index == selection.selected;

        *bg = if is_selected {
            Color::rgba(0.12, 0.12, 0.25, 0.95).into()
        } else {
            Color::rgba(0.06, 0.06, 0.14, 0.85).into()
        };
        *border = if is_selected {
            upgrade.category_color().into()
        } else {
            Color::rgba(1.0, 1.0, 1.0, 0.15).into()
        };
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Game-over screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_game_over_ui(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(20.0),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.75).into(),
                z_index: ZIndex::Local(40),
                ..default()
            },
            GameOverRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle { font_size: 80.0, color: Color::rgb(1.0, 0.25, 0.25), ..default() },
            ));
            root.spawn(TextBundle::from_section(
                format!("Score: {}   Best: {}", score.current, score.high_score),
                TextStyle { font_size: 28.0, color: Color::WHITE, ..default() },
            ));
            root.spawn(TextBundle::from_section(
                "Press SPACE or ENTER to play again",
                TextStyle {
                    font_size: 22.0,
                    color:     Color::rgba(1.0, 1.0, 1.0, 0.6),
                    ..default()
                },
            ));
        });
}

fn despawn_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}
