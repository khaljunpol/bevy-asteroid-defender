use bevy::{prelude::*, ui::widget::UiImageSize};

use crate::{state::states::GameStates, resources::{Life, GameSprites, Score}};

pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameStates::StartGame), spawn_score_ui)
            .add_systems(Update, update_score_ui)
            .add_systems(OnExit(GameStates::InGame), despawn_score_ui)
            .add_systems(OnEnter(GameStates::StartGame), spawn_life_ui)
            .add_systems(OnExit(GameStates::InGame), despawn_life_ui)
            .add_systems(Update, update_life_image);
    }
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct LifeUI;

#[derive(Component)]
pub struct ToggleImage{
    pub index: i32,
    pub enabled: Handle<Image>,
    pub disabled: Handle<Image>
}

impl ToggleImage {
    pub fn new(index: i32, enabled: Handle<Image>, disabled: Handle<Image>) -> Self {
        ToggleImage { index, enabled, disabled }
    }
}

fn spawn_life_ui(
    mut commands: Commands, 
    game_sprites: Res<GameSprites>,
    life: Res<Life>){
    commands
        .spawn((
            NodeBundle{
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Name::new("UI Root"),
            LifeUI
        ))
        .with_children(|commands| {
            for _i in 0..=life.max_life - 1 {
                commands.spawn((
                    ImageBundle {
                        image: UiImage {
                            texture: game_sprites.life_attack.clone(),
                            ..default()
                        },
                        ..default()
                    },
                    ToggleImage::new(_i, game_sprites.life_attack.clone(), game_sprites.life_normal.clone()),
                ));
            }
        });
}

fn despawn_life_ui(mut commands: Commands, mut images: Query<Entity, With<LifeUI>>){
    for entity in &images {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_life_image(mut images: Query<(&mut UiImage, &ToggleImage), With<ToggleImage>>, life: Res<Life>){
    for (mut image, toggle_image) in &mut images {
        for _i in 0..=life.max_life - 1{
            if _i == toggle_image.index {
                if _i >= life.current_life {
                    image.texture = toggle_image.disabled.clone();
                }
                else{
                    image.texture = toggle_image.enabled.clone()
                }
            }
        }
    }
}

fn spawn_score_ui(mut commands: Commands){
    commands
        .spawn((
            NodeBundle{
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Name::new("UI Root"),
            ScoreUI
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "SCORE:",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ScoreText,
            ));
        });
}

fn despawn_score_ui(mut commands: Commands, mut text: Query<Entity, With<ScoreUI>>){
    for entity in &text {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_score_ui(mut texts: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("SCORE: {:?} - HIGHEST: {:?}", score.current, score.high_score);
    }
}