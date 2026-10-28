//! 界面 ui
use bevy::{prelude::*, sprite::Anchor};

use crate::status::{self, GameState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, startup_spawn);
    app.add_systems(Update, update_score);
    app.add_systems(Update, update_status);
    app.add_systems(Update, update_prompt);
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct PromptText;

fn startup_spawn(mut commands: Commands, game_state: Res<State<GameState>>) {
    commands.spawn((
        Text2d::new("Score: 0"),
        Anchor::CENTER_RIGHT,
        TextFont::default().with_font_size(24.),
        Transform::from_translation(Vec3::new(200., 240., 0.)),
        ScoreText,
    ));

    commands.spawn((
        Text2d::new(format!("Status: {:?}", game_state.get())),
        Anchor::CENTER_LEFT,
        TextFont::default().with_font_size(24.),
        Transform::from_translation(Vec3::new(-200., 240., 0.)),
        StatusText,
    ));

    commands.spawn((
        Text2d::new(format!("")),
        TextFont::default().with_font_size(18.),
        Transform::from_translation(Vec3::new(0., -240., 0.)),
        PromptText,
    ));
}

fn update_score(
    mut score_text: Single<&mut Text2d, With<ScoreText>>,
    game_status: Res<status::GameStatus>,
) {
    score_text.0 = format!("Score: {}", game_status.score);
}

fn update_status(
    mut status_text: Single<&mut Text2d, With<StatusText>>,
    game_state: Res<State<GameState>>,
) {
    status_text.0 = format!("{:?}", game_state.get());
}

fn update_prompt(
    mut prompt_text: Single<&mut Text2d, With<PromptText>>,
    game_state: Res<State<GameState>>,
) {
    prompt_text.0 = (match game_state.get() {
        GameState::Ready => "Press [W/A/S/D] or [Arrow Keys] to start, [Space] to pause.",
        GameState::Playing => "",
        GameState::Paused => "Press [Space] to continue.",
        GameState::GameOver => "Press [R] to restart.",
    })
    .to_string();
}
