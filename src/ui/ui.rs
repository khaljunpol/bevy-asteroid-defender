use bevy::prelude::*;

use lib::ShipType;
use crate::{
    player::{ship::ShipComponent, player::PlayerComponent},
    resources::{CountdownResource, GameSprites, IsPaused, Life, LevelResource, PlayerBuff, PlayerUpgrades, Score, ShipSelectState, UpgradeSelectionState},
    state::states::GameStates,
    upgrades::upgrades::UpgradeType,
};

// ─────────────────────────────────────────────────────────────────────────────
// Plugin
// ─────────────────────────────────────────────────────────────────────────────

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ship select screen
            .add_systems(OnEnter(GameStates::ShipSelect),  spawn_ship_select_ui)
            .add_systems(OnExit(GameStates::ShipSelect),   despawn_ship_select_ui)
            .add_systems(Update, update_ship_select_ui.run_if(in_state(GameStates::ShipSelect)))
            // HUD
            .add_systems(OnEnter(GameStates::StartGame),   spawn_hud)
            .add_systems(OnExit(GameStates::GameOver),     despawn_hud)
            .add_systems(Update, update_score_ui)
            .add_systems(Update, update_life_ui)
            .add_systems(Update, update_level_ui)
            .add_systems(Update, update_buff_ui)
            // Countdown
            .add_systems(OnEnter(GameStates::Countdown),   spawn_countdown_ui)
            .add_systems(OnExit(GameStates::Countdown),    despawn_countdown_ui)
            .add_systems(Update, update_countdown_ui.run_if(in_state(GameStates::Countdown)))
            // Level indicator
            .add_systems(OnEnter(GameStates::InGame),      spawn_level_indicator)
            .add_systems(Update, fade_level_indicator.run_if(in_state(GameStates::InGame)))
            // Level-clear screen
            .add_systems(OnEnter(GameStates::LevelComplete),  spawn_level_clear_ui)
            .add_systems(OnExit(GameStates::LevelComplete),   despawn_level_clear_ui)
            // Upgrade selection
            .add_systems(OnEnter(GameStates::UpgradeSelection),  spawn_upgrade_ui)
            .add_systems(OnExit(GameStates::UpgradeSelection),   despawn_upgrade_ui)
            .add_systems(Update, update_upgrade_ui.run_if(in_state(GameStates::UpgradeSelection)))
            // Game-over
            .add_systems(OnEnter(GameStates::GameOver),    spawn_game_over_ui)
            .add_systems(OnExit(GameStates::GameOver),     despawn_game_over_ui)
            // Pause overlay
            .add_systems(Update, update_pause_ui.run_if(in_state(GameStates::InGame)))
            .add_systems(OnExit(GameStates::InGame),       despawn_pause_ui);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Marker components
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component)] struct HudRoot;
#[derive(Component)] struct ScoreText;
#[derive(Component)] struct LevelText;
#[derive(Component)] struct BuffText;
#[derive(Component)] struct LifeShipIcon;
#[derive(Component)] struct LifeCountText;

#[derive(Component)] struct ShipSelectRoot;
#[derive(Component)] struct ShipSelectCard(usize);

#[derive(Component)] struct CountdownRoot;
#[derive(Component)] struct CountdownText;

#[derive(Component)] struct LevelIndicator { timer: Timer }

#[derive(Component)] struct LevelClearRoot;

#[derive(Component)] struct UpgradeRoot;
#[derive(Component)] struct UpgradeCard { index: usize }

#[derive(Component)] struct GameOverRoot;
#[derive(Component)] struct PauseRoot;

// ─────────────────────────────────────────────────────────────────────────────
// HUD
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_hud(mut commands: Commands, game_sprites: Res<GameSprites>) {
    let font = game_sprites.font.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    padding:         UiRect::all(Val::Px(14.0)),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                z_index: ZIndex::Local(1),
                ..default()
            },
            HudRoot,
            Name::new("HUD"),
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items:     AlignItems::FlexStart,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                // Left: life + level
                row.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap:        Val::Px(4.0),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|col| {
                    // Ship icon + count row
                    col.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items:    AlignItems::Center,
                            column_gap:     Val::Px(6.0),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|icon_row| {
                        icon_row.spawn((
                            ImageBundle {
                                image: UiImage::new(game_sprites.life_normal.clone()),
                                style: Style {
                                    width:  Val::Px(28.0),
                                    height: Val::Px(28.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            LifeShipIcon,
                        ));
                        icon_row.spawn((
                            TextBundle::from_section(
                                "x 3",
                                TextStyle {
                                    font:      font.clone(),
                                    font_size: 22.0,
                                    color:     Color::WHITE,
                                },
                            ),
                            LifeCountText,
                        ));
                    });

                    // Level
                    col.spawn((
                        TextBundle::from_section(
                            "LEVEL 1",
                            TextStyle {
                                font:      font.clone(),
                                font_size: 14.0,
                                color:     Color::rgba(1.0, 1.0, 0.5, 0.8),
                            },
                        ),
                        LevelText,
                    ));
                });

                // Right: score + buff
                row.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items:    AlignItems::FlexEnd,
                        row_gap:        Val::Px(4.0),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        TextBundle::from_section(
                            "000000",
                            TextStyle {
                                font:      font.clone(),
                                font_size: 28.0,
                                color:     Color::WHITE,
                            },
                        ),
                        ScoreText,
                    ));
                    col.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font:      font.clone(),
                                font_size: 14.0,
                                color:     Color::rgba(1.0, 1.0, 0.5, 0.9),
                            },
                        ),
                        BuffText,
                    ));
                });
            });
        });
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn update_score_ui(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut t in &mut query {
        t.sections[0].value = format!("{:06}", score.current);
    }
}

fn update_life_ui(
    life:         Res<Life>,
    game_sprites: Res<GameSprites>,
    ship_q:       Query<&ShipComponent, With<PlayerComponent>>,
    mut icons:    Query<&mut UiImage, With<LifeShipIcon>>,
    mut texts:    Query<&mut Text, With<LifeCountText>>,
) {
    let ship_type = ship_q.get_single().map(|s| s.ship_type).unwrap_or(ShipType::Normal);
    let life_handle = match ship_type {
        ShipType::Attack => game_sprites.life_attack.clone(),
        ShipType::Shield => game_sprites.life_shield.clone(),
        ShipType::Normal => game_sprites.life_normal.clone(),
    };
    for mut img in &mut icons {
        img.texture = life_handle.clone();
    }
    for mut t in &mut texts {
        t.sections[0].value = format!("x {}", life.current_life.max(0));
    }
}

fn update_level_ui(level: Res<LevelResource>, mut query: Query<&mut Text, With<LevelText>>) {
    for mut t in &mut query {
        t.sections[0].value = format!("LEVEL {}", level.current);
    }
}

fn update_buff_ui(buff: Res<PlayerBuff>, mut query: Query<&mut Text, With<BuffText>>) {
    for mut t in &mut query {
        let mut parts = Vec::new();
        if buff.bolt_timer > 0.0  { parts.push(format!("SPEED {:.1}s",  buff.bolt_timer)); }
        if buff.shield_timer > 0.0 { parts.push(format!("SHIELD {:.1}s", buff.shield_timer)); }
        t.sections[0].value = parts.join("  ");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ship select screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_ship_select_ui(
    mut commands: Commands,
    selection:    Res<ShipSelectState>,
    game_sprites: Res<GameSprites>,
) {
    let font = game_sprites.font.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(36.0),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.08, 0.95).into(),
                z_index: ZIndex::Local(50),
                ..default()
            },
            ShipSelectRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "SELECT YOUR SHIP",
                TextStyle { font: font.clone(), font_size: 52.0, color: Color::WHITE },
            ));

            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap:     Val::Px(28.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                let ships = [
                    (0usize, "STANDARD",  Color::rgb(0.4, 0.6, 1.0), game_sprites.ship_type_normal.clone(),
                     "HP 3  Speed 3  Fire 3",  "Balanced fighter. No bonus."),
                    (1, "GUARDIAN",  Color::rgb(0.4, 1.0, 0.5), game_sprites.ship_type_shield.clone(),
                     "HP 5  Speed 2  Fire 2",  "Starts with 2 extra HP.\nSlightly slower."),
                    (2, "DESTROYER", Color::rgb(1.0, 0.4, 0.4), game_sprites.ship_type_attack.clone(),
                     "HP 2  Speed 3  Fire 4",  "Starts with Heavy Rounds.\nCosts 1 max HP."),
                ];
                for (idx, name, color, texture, stats, desc) in ships {
                    spawn_ship_card(row, idx, name, color, texture, stats, desc, idx == selection.selected, &font);
                }
            });

            root.spawn(TextBundle::from_section(
                "LEFT RIGHT to choose   SPACE or ENTER to start",
                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgba(1.0,1.0,1.0,0.55) },
            ));
        });
}

fn spawn_ship_card(
    parent:      &mut ChildBuilder,
    index:       usize,
    name:        &str,
    accent:      Color,
    texture:     Handle<Image>,
    stats:       &str,
    desc:        &str,
    is_selected: bool,
    font:        &Handle<Font>,
) {
    let border_color = if is_selected { accent } else { Color::rgba(1.0,1.0,1.0,0.15) };
    let bg_color     = if is_selected { Color::rgba(0.1, 0.1, 0.22, 0.97) }
                       else           { Color::rgba(0.05, 0.05, 0.12, 0.88) };

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width:          Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    align_items:    AlignItems::Center,
                    padding:        UiRect::all(Val::Px(18.0)),
                    row_gap:        Val::Px(10.0),
                    border:         UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: bg_color.into(),
                border_color:     border_color.into(),
                ..default()
            },
            ShipSelectCard(index),
        ))
        .with_children(|card| {
            card.spawn(ImageBundle {
                image: UiImage::new(texture),
                style: Style { width: Val::Px(64.0), height: Val::Px(64.0), ..default() },
                background_color: Color::WHITE.into(),
                ..default()
            });
            card.spawn(TextBundle::from_section(
                name,
                TextStyle { font: font.clone(), font_size: 22.0, color: accent },
            ));
            card.spawn(TextBundle::from_section(
                stats,
                TextStyle { font: font.clone(), font_size: 13.0, color: Color::rgba(1.0,1.0,1.0,0.8) },
            ));
            card.spawn(TextBundle::from_section(
                desc,
                TextStyle { font: font.clone(), font_size: 12.0, color: Color::rgba(0.8,0.8,0.8,0.7) },
            ));
        });
}

fn despawn_ship_select_ui(mut commands: Commands, query: Query<Entity, With<ShipSelectRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn update_ship_select_ui(
    selection: Res<ShipSelectState>,
    mut cards: Query<(&ShipSelectCard, &mut BackgroundColor, &mut BorderColor)>,
) {
    if !selection.is_changed() { return; }
    let colors = [Color::rgb(0.4, 0.6, 1.0), Color::rgb(0.4, 1.0, 0.5), Color::rgb(1.0, 0.4, 0.4)];
    for (card, mut bg, mut border) in &mut cards {
        let is_sel = card.0 == selection.selected;
        *bg     = if is_sel { Color::rgba(0.1,0.1,0.22,0.97).into() } else { Color::rgba(0.05,0.05,0.12,0.88).into() };
        *border = if is_sel { colors[card.0].into() } else { Color::rgba(1.0,1.0,1.0,0.15).into() };
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Countdown
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_countdown_ui(mut commands: Commands, game_sprites: Res<GameSprites>) {
    let font = game_sprites.font.clone();
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
                    TextStyle { font, font_size: 140.0, color: Color::rgba(1.0, 1.0, 1.0, 0.9) },
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
    let label = match countdown.count { 3 => "3", 2 => "2", 1 => "1", 0 => "GO", _ => "" };
    for mut t in &mut query { t.sections[0].value = label.to_string(); }
}

// ─────────────────────────────────────────────────────────────────────────────
// Level indicator
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_level_indicator(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    level:        Res<LevelResource>,
) {
    let font = game_sprites.font.clone();
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
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                format!("LEVEL {}", level.current),
                TextStyle { font, font_size: 36.0, color: Color::rgba(1.0, 1.0, 0.5, 0.0) },
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
        let pct   = indicator.timer.percent();
        let alpha = if pct < 0.2 { pct / 0.2 } else if pct > 0.7 { 1.0 - (pct - 0.7) / 0.3 } else { 1.0 };
        for &child in children {
            if let Ok(mut t) = texts.get_mut(child) {
                t.sections[0].style.color = Color::rgba(1.0, 1.0, 0.5, alpha);
            }
        }
        if indicator.timer.just_finished() { commands.entity(entity).despawn_recursive(); }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Level-clear screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_level_clear_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    level:        Res<LevelResource>,
) {
    let font    = game_sprites.font.clone();
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
                format!("LEVEL {} CLEARED", cleared),
                TextStyle { font: font.clone(), font_size: 60.0, color: Color::rgb(0.4, 1.0, 0.5) },
            ));
            root.spawn(TextBundle::from_section(
                "Choosing upgrade",
                TextStyle { font: font.clone(), font_size: 26.0, color: Color::rgba(1.0, 1.0, 1.0, 0.7) },
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
    game_sprites: Res<GameSprites>,
    selection:    Res<UpgradeSelectionState>,
    upgrades:     Res<PlayerUpgrades>,
) {
    let font = game_sprites.font.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width:           Val::Percent(100.0),
                    height:          Val::Percent(100.0),
                    flex_direction:  FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    row_gap:         Val::Px(28.0),
                    position_type:   PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.1, 0.88).into(),
                z_index: ZIndex::Local(30),
                ..default()
            },
            UpgradeRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "CHOOSE AN UPGRADE",
                TextStyle { font: font.clone(), font_size: 38.0, color: Color::WHITE },
            ));

            root.spawn(NodeBundle {
                style: Style {
                    flex_direction:  FlexDirection::Row,
                    column_gap:      Val::Px(24.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                for (i, upgrade) in selection.choices.iter().enumerate() {
                    let is_selected = i == selection.selected;
                    spawn_upgrade_card(row, *upgrade, i, is_selected, &upgrades, &font);
                }
            });

            root.spawn(TextBundle::from_section(
                "LEFT RIGHT to choose   SPACE or ENTER to confirm",
                TextStyle { font: font.clone(), font_size: 18.0, color: Color::rgba(1.0, 1.0, 1.0, 0.6) },
            ));
        });
}

fn spawn_upgrade_card(
    parent:      &mut ChildBuilder,
    upgrade:     UpgradeType,
    index:       usize,
    is_selected: bool,
    upgrades:    &PlayerUpgrades,
    font:        &Handle<Font>,
) {
    let border_color = if is_selected { upgrade.category_color() } else { Color::rgba(1.0, 1.0, 1.0, 0.15) };
    let bg_color     = if is_selected { Color::rgba(0.12, 0.12, 0.25, 0.95) } else { Color::rgba(0.06, 0.06, 0.14, 0.85) };

    let cur = upgrade.current_level(upgrades);
    let max = upgrade.max_level();
    let level_text = if max == 1 {
        if cur == 0 { "NEW".to_string() } else { "MAXED".to_string() }
    } else {
        format!("Level {} to {}", cur, cur + 1)
    };

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width:          Val::Px(260.0),
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
            card.spawn(TextBundle::from_section(
                upgrade.category(),
                TextStyle { font: font.clone(), font_size: 11.0, color: upgrade.category_color() },
            ));
            card.spawn(TextBundle::from_section(
                upgrade.name(),
                TextStyle {
                    font:      font.clone(),
                    font_size: 20.0,
                    color:     if is_selected { Color::WHITE } else { Color::rgba(1.0, 1.0, 1.0, 0.7) },
                },
            ));
            card.spawn(TextBundle {
                text: Text::from_section(
                    upgrade.description(),
                    TextStyle { font: font.clone(), font_size: 13.0, color: Color::rgba(0.85, 0.85, 0.85, 0.85) },
                ),
                style: Style { flex_grow: 1.0, ..default() },
                ..default()
            });
            card.spawn(TextBundle::from_section(
                level_text,
                TextStyle { font: font.clone(), font_size: 14.0, color: upgrade.category_color() },
            ));
        });
}

fn despawn_upgrade_ui(mut commands: Commands, query: Query<Entity, With<UpgradeRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

fn update_upgrade_ui(
    selection: Res<UpgradeSelectionState>,
    mut cards: Query<(&UpgradeCard, &mut BackgroundColor, &mut BorderColor)>,
) {
    if !selection.is_changed() || selection.choices.is_empty() { return; }
    for (card, mut bg, mut border) in &mut cards {
        if card.index >= selection.choices.len() { continue; }
        let upgrade     = selection.choices[card.index];
        let is_selected = card.index == selection.selected;
        *bg = if is_selected { Color::rgba(0.12, 0.12, 0.25, 0.95).into() } else { Color::rgba(0.06, 0.06, 0.14, 0.85).into() };
        *border = if is_selected { upgrade.category_color().into() } else { Color::rgba(1.0, 1.0, 1.0, 0.15).into() };
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Game-over screen
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_game_over_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    score:        Res<Score>,
) {
    let font = game_sprites.font.clone();
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
                TextStyle { font: font.clone(), font_size: 80.0, color: Color::rgb(1.0, 0.25, 0.25) },
            ));
            root.spawn(TextBundle::from_section(
                format!("Score {}   Best {}", score.current, score.high_score),
                TextStyle { font: font.clone(), font_size: 28.0, color: Color::WHITE },
            ));
            root.spawn(TextBundle::from_section(
                "Press SPACE or ENTER to play again",
                TextStyle { font: font.clone(), font_size: 22.0, color: Color::rgba(1.0, 1.0, 1.0, 0.6) },
            ));
        });
}

fn despawn_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for e in &query { commands.entity(e).despawn_recursive(); }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pause overlay
// ─────────────────────────────────────────────────────────────────────────────

fn update_pause_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    is_paused:    Res<IsPaused>,
    upgrades:     Res<PlayerUpgrades>,
    pause_q:      Query<Entity, With<PauseRoot>>,
) {
    let exists = !pause_q.is_empty();

    if is_paused.0 && !exists {
        let font = game_sprites.font.clone();

        // Collect active upgrades for display
        let active: Vec<String> = UpgradeType::all()
            .iter()
            .filter_map(|u| {
                let lvl = u.current_level(&upgrades);
                if lvl > 0 {
                    if u.max_level() == 1 { Some(u.name().to_string()) }
                    else { Some(format!("{} {}", u.name(), lvl)) }
                } else {
                    None
                }
            })
            .collect();

        let upgrade_text = if active.is_empty() {
            "No upgrades yet".to_string()
        } else {
            // Split into lines of 4 upgrades each
            active.chunks(4)
                .map(|chunk| chunk.join("   "))
                .collect::<Vec<_>>()
                .join("\n")
        };

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
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.72).into(),
                    z_index: ZIndex::Local(50),
                    ..default()
                },
                PauseRoot,
            ))
            .with_children(|root| {
                root.spawn(TextBundle::from_section(
                    "PAUSED",
                    TextStyle { font: font.clone(), font_size: 72.0, color: Color::WHITE },
                ));

                // Upgrades section
                root.spawn(NodeBundle {
                    style: Style {
                        flex_direction:  FlexDirection::Column,
                        align_items:     AlignItems::Center,
                        row_gap:         Val::Px(6.0),
                        padding:         UiRect::all(Val::Px(14.0)),
                        border:          UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::rgba(1.0, 1.0, 1.0, 0.05).into(),
                    border_color:     Color::rgba(1.0, 1.0, 1.0, 0.15).into(),
                    ..default()
                })
                .with_children(|section| {
                    section.spawn(TextBundle::from_section(
                        "UPGRADES",
                        TextStyle { font: font.clone(), font_size: 14.0, color: Color::rgba(1.0, 1.0, 0.5, 0.7) },
                    ));
                    section.spawn(TextBundle::from_section(
                        upgrade_text,
                        TextStyle { font: font.clone(), font_size: 16.0, color: Color::rgba(1.0, 1.0, 1.0, 0.85) },
                    ));
                });

                root.spawn(TextBundle::from_section(
                    "ESC or R to Resume",
                    TextStyle { font: font.clone(), font_size: 26.0, color: Color::rgba(0.6, 1.0, 0.6, 0.9) },
                ));
                root.spawn(TextBundle::from_section(
                    "SPACE or ENTER to Restart",
                    TextStyle { font: font.clone(), font_size: 26.0, color: Color::rgba(1.0, 0.6, 0.6, 0.9) },
                ));
            });
    } else if !is_paused.0 && exists {
        for e in &pause_q { commands.entity(e).despawn_recursive(); }
    }
}

fn despawn_pause_ui(mut commands: Commands, q: Query<Entity, With<PauseRoot>>) {
    for e in &q { commands.entity(e).despawn_recursive(); }
}
